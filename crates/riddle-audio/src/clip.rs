use crate::*;

use std::{io::Read, sync::Arc};

#[derive(Clone)]
pub struct Clip {
    pub(crate) data: ClipData,
}

impl Clip {
    pub fn new<R>(mut data: R) -> Result<Clip>
    where
        R: Read,
    {
        let mut owned_data: Vec<u8> = vec![];
        data.read_to_end(&mut owned_data)
            .map_err(|e| CommonError::IOError(e))?;

        Ok(Self {
            data: ClipData::new(owned_data),
        })
    }
}

#[derive(Clone)]
pub struct ClipData {
    data: Arc<Vec<u8>>,
}

impl ClipData {
    fn new(data: Vec<u8>) -> Self {
        Self { data: data.into() }
    }
}

impl AsRef<[u8]> for ClipData {
    fn as_ref(&self) -> &[u8] {
        Arc::as_ref(&self.data).as_ref()
    }
}
