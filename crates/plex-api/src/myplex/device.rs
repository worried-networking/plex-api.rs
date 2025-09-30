use crate::{
    http_client::HttpClient,
    media_container::devices::{DevicesMediaContainer, Feature},
    url::{MYPLEX_DEVICES, MYPLEX_RESOURCES},
    Error, Player, Result, Server,
};
use futures::{future::select_ok, FutureExt};
use secrecy::ExposeSecret;
use tracing::{debug, error, trace};

pub struct DeviceManager {
    pub client: HttpClient,
}

impl DeviceManager {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    async fn devices_internal<'a>(&'a self, url: &str) -> Result<Vec<Device<'a>>> {
        let container: DevicesMediaContainer = self
            .client
            .get(url)
            .header("Accept", "application/xml")
            .xml()
            .await?;

        Ok(container
            .devices
            .into_iter()
            .map(|device| Device {
                inner: device,
                client: &self.client,
            })
            .collect())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn devices(&self) -> Result<Vec<Device<'_>>> {
        self.devices_internal(MYPLEX_DEVICES).await
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn resources(&self) -> Result<Vec<Device<'_>>> {
        self.devices_internal(MYPLEX_RESOURCES).await
    }
}

#[derive(Debug, Clone)]
pub struct Device<'a> {
    inner: crate::media_container::devices::Device,
    client: &'a HttpClient,
}

impl Device<'_> {
    /// Returns the list of features supported by the device.
    pub fn provides(&self, feature: Feature) -> bool {
        self.inner.provides.contains(&feature)
    }

    pub fn identifier(&self) -> &str {
        &self.inner.client_identifier
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    /// Syntax sugar method for checking if the current device provides [`Feature::Server`]
    pub fn is_server(&self) -> bool {
        self.provides(Feature::Server)
    }

    /// Syntax sugar method for checking if the current device provides [`Feature::Controller`]
    pub fn is_controller(&self) -> bool {
        self.provides(Feature::Controller)
    }

    /// Returns the authentication token that should be used when connecting to the device.
    /// If it's a shared device, the main authentication token will no be accepted.
    pub fn access_token(&self) -> Option<&str> {
        self.inner.access_token.as_ref().map(|v| v.expose_secret())
    }

    /// Connect to the device.
    #[tracing::instrument(level = "debug", skip(self), fields(device_name = self.inner.name))]
    pub async fn connect(&self) -> Result<DeviceConnection> {
        if !self.is_server() && !self.is_controller() {
            error!("Device must provide Server or Controller");
            return Err(Error::DeviceConnectionNotSupported);
        }

        if !self.inner.connections.is_empty() {
            let mut client = self.client.clone();
            if let Some(access_token) = self.inner.access_token.as_ref() {
                let access_token = access_token.expose_secret();
                if access_token != client.x_plex_token() {
                    debug!("Connecting using access token for the device");
                    client = client.set_x_plex_token(access_token.to_owned());
                }
            }

            if self.is_server() {
                trace!(
                    "Connecting to server {id}",
                    id = self.inner.client_identifier,
                );
                let futures = self
                    .inner
                    .connections
                    .iter()
                    .map(|connection| {
                        trace!("Trying {address}", address = connection.uri);
                        crate::Server::new(&connection.uri, client.clone()).boxed()
                    })
                    .collect::<Vec<_>>();

                let (server, _) = select_ok(futures).await?;
                trace!("Connected via {address}", address = server.client().api_url);
                Ok(DeviceConnection::Server(Box::new(server)))
            } else {
                trace!(
                    "Connecting to player {id}",
                    id = self.inner.client_identifier,
                );
                client
                    .x_plex_target_client_identifier
                    .clone_from(&self.inner.client_identifier);

                let futures = self
                    .inner
                    .connections
                    .iter()
                    .map(|connection| {
                        trace!("Trying {address}", address = connection.uri);
                        crate::Player::new(&connection.uri, client.clone()).boxed()
                    })
                    .collect::<Vec<_>>();

                let (player, _) = select_ok(futures).await?;
                trace!("Connected via {address}", address = player.client().api_url);
                Ok(DeviceConnection::Player(Box::new(player)))
            }
        } else {
            Err(Error::DeviceConnectionsIsEmpty)
        }
    }

    /// Establish a connection to the device using server as a proxy.
    #[tracing::instrument(level = "debug", skip(self, server), fields(device_name = self.inner.name))]
    pub async fn connect_via(&self, server: &Server) -> Result<DeviceConnection> {
        if !self.is_controller() {
            error!("Only devices providing Controller can be connected via proxy");
            return Err(Error::DeviceConnectionNotSupported);
        }

        let futures = self
            .inner
            .connections
            .iter()
            .map(|connection| {
                trace!("Trying {address}", address = connection.uri);
                crate::Player::via_proxy(&connection.uri, server).boxed()
            })
            .collect::<Vec<_>>();

        let (player, _) = select_ok(futures).await?;
        Ok(DeviceConnection::Player(Box::new(player)))
    }
}

#[derive(Debug, Clone)]
pub enum DeviceConnection {
    Server(Box<Server>),
    Player(Box<Player>),
}
