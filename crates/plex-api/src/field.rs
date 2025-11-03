use std::{ops::Deref, result};

use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::{Error, Result};

const NULL: &str = "null";
const BOOL: &str = "boolean";
const ARRAY: &str = "array";
const OBJECT: &str = "object";
const STRING: &str = "string";
const NUMBER: &str = "number";
const OUT_OF_BOUNDS: &str = "out of bounds";

trait FromValue
where
    Self: Sized,
{
    fn from_value(value: Value) -> result::Result<Self, (&'static str, &'static str)>;
}

impl FromValue for String {
    fn from_value(value: Value) -> result::Result<Self, (&'static str, &'static str)> {
        match value {
            Value::Null => Err((NULL, STRING)),
            Value::Bool(v) => Ok(v.to_string()),
            Value::Number(v) => Ok(v.to_string()),
            Value::String(v) => Ok(v),
            Value::Array(_) => Err((ARRAY, STRING)),
            Value::Object(_) => Err((OBJECT, STRING)),
        }
    }
}

impl FromValue for u32 {
    fn from_value(value: Value) -> result::Result<Self, (&'static str, &'static str)> {
        match value {
            Value::Null => Err((NULL, NUMBER)),
            Value::Bool(_) => Err((BOOL, NUMBER)),
            Value::Number(v) => {
                if let Some(u) = v.as_u64() {
                    Ok(u32::try_from(u).map_err(|_| (OUT_OF_BOUNDS, NUMBER))?)
                } else {
                    Err((NULL, NULL))
                }
            }
            Value::String(v) => v.parse::<u32>().map_err(|e| (STRING, NUMBER)),
            Value::Array(_) => Err((ARRAY, NUMBER)),
            Value::Object(_) => Err((OBJECT, NUMBER)),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum MetadataField<T> {
    Value(T),
    /// This is a little awkward but free-form strings don't implement `Copy`
    Error((&'static str, &'static str)),
    #[default]
    Missing,
}

impl<'de, T> Deserialize<'de> for MetadataField<T>
where
    T: FromValue,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        match T::from_value(value) {
            Ok(v) => Ok(Self::Value(v)),
            Err(e) => Ok(Self::Error(e)),
        }
    }
}

impl<T> MetadataField<T> {
    /// Unwraps the inner value, panics if there is no value or the value failed deserialization.
    pub fn unwrap(self) -> T {
        self.result().unwrap()
    }

    /// Returns a reference to the inner value.
    pub fn as_ref(&self) -> MetadataField<&T> {
        match self {
            Self::Value(v) => MetadataField::Value(v),
            Self::Error((u, e)) => MetadataField::Error((u, e)),
            Self::Missing => MetadataField::Missing,
        }
    }

    /// Coerces the inner value via `Deref`.
    pub fn as_deref(&self) -> MetadataField<&T::Target>
    where
        T: Deref,
    {
        match self {
            Self::Value(v) => MetadataField::Value(v.deref()),
            Self::Error((u, e)) => MetadataField::Error((u, e)),
            Self::Missing => MetadataField::Missing,
        }
    }

    /// Converts self into an Option<T>, discarding the error, if any.
    pub fn ok(self) -> Option<T> {
        match self {
            Self::Value(v) => Some(v),
            _ => None,
        }
    }

    /// Converts self into a Result<T>, missing values become an error.
    pub fn result(self) -> Result<T> {
        match self {
            Self::Value(v) => Ok(v),
            Self::Error((u, e)) => Err(Error::FieldError(format!("Unexpected {u}, expected {e}."))),
            Self::Missing => Err(Error::FieldError(
                "An expected field was not present.".to_string(),
            )),
        }
    }
}
