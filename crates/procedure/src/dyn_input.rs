use std::{
    any::{type_name, Any},
    fmt,
};

use serde::{de::Error, Deserialize};

use crate::{DeserializeError, DowncastError, ProcedureError};

// It would be really nice if this with `&'a DynInput<'de>` but that would require `#[repr(transparent)]` with can only be constructed with unsafe which is probally not worth it.

/// TODO
pub struct DynInput<'a, 'de> {
    inner: Repr<'a, 'de>,
    pub(crate) type_name: &'static str,
}

enum Repr<'a, 'de> {
    Value(&'a mut (dyn Any + Send)),
    Deserializer(&'a mut (dyn erased_serde::Deserializer<'de> + Send)),
}

impl<'a, 'de> DynInput<'a, 'de> {
    // TODO: Discuss using `Option` as a workaround for ownership
    pub fn new_value<T: Send + 'static>(value: &'a mut T) -> Self {
        Self {
            inner: Repr::Value(value),
            type_name: type_name::<T>(),
        }
    }

    // TODO: In a perfect world this would be public.
    pub(crate) fn new_deserializer<D: erased_serde::Deserializer<'de> + Send>(
        deserializer: &'a mut D,
    ) -> Self {
        Self {
            inner: Repr::Deserializer(deserializer),
            type_name: type_name::<D>(),
        }
    }

    /// TODO
    pub fn deserialize<T: Deserialize<'de>>(self) -> Result<T, ProcedureError> {
        let Repr::Deserializer(deserializer) = self.inner else {
            return Err(ProcedureError::Deserialize(DeserializeError(
                erased_serde::Error::custom(format!(
                    "attempted to deserialize from value '{}' but expected deserializer",
                    self.type_name
                )),
            )));
        };

        erased_serde::deserialize(deserializer)
            .map_err(|err| ProcedureError::Deserialize(DeserializeError(err)))
    }

    /// TODO
    pub fn value<T: 'static>(&mut self) -> Result<&mut T, ProcedureError> {
        let Repr::Value(ref mut value) = self.inner else {
            return Err(DowncastError {
                from: None,
                to: type_name::<T>(),
            }
            .into());
        };
        Ok(value.downcast_mut::<T>().ok_or(DowncastError {
            from: Some(self.type_name),
            to: type_name::<T>(),
        })?)
    }
}

impl<'a, 'de> fmt::Debug for DynInput<'a, 'de> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}
