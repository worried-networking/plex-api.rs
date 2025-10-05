use bytes::Bytes;
use futures::AsyncWrite;
use http::Response;
use std::io;
use tokio::io::AsyncWriteExt;

pub trait ResponseExt {
    /// Read the response body as text.
    async fn text(&mut self) -> io::Result<String>;
    
    /// Copy the response body to a writer.
    async fn copy_to<W: AsyncWrite + Unpin>(&mut self, writer: W) -> io::Result<()>;
    
    /// Consume the response body.
    async fn consume(&mut self) -> io::Result<()>;
}

impl ResponseExt for Response<Bytes> {
    async fn text(&mut self) -> io::Result<String> {
        let bytes = std::mem::replace(self.body_mut(), Bytes::new());
        String::from_utf8(bytes.to_vec()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
    
    async fn copy_to<W: AsyncWrite + Unpin>(&mut self, mut writer: W) -> io::Result<()> {
        let bytes = std::mem::replace(self.body_mut(), Bytes::new());
        writer.write_all(&bytes).await?;
        writer.flush().await?;
        Ok(())
    }
    
    async fn consume(&mut self) -> io::Result<()> {
        // Just drop the body
        *self.body_mut() = Bytes::new();
        Ok(())
    }
}