use std::io;

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct SerialConnection {}

impl SerialConnection {
    pub fn new() -> Self {
        SerialConnection {}
    }
}

impl Decoder for SerialConnection {
    type Item = String;

    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < std::mem::size_of::<u64>() {
            return Ok(None);
        }

        let str_len = src
            .get_u64_le()
            .try_into()
            .expect("converting len to usize");

        if src.len() < str_len {
            return Ok(None);
        }

        let data = src[..str_len].to_vec();
        src.advance(data.len());

        let out = String::from_utf8(data).expect("converting data to string");
        Ok(Some(out))
    }
}

impl Encoder<String> for SerialConnection {
    type Error = io::Error;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u64_le(usize::try_into(item.len()).expect("converting cmd len to usize"));
        dst.put(item.as_bytes());
        Ok(())
    }
}
