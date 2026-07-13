use tokio_util::codec::{Decoder, Encoder};
use bytes::BytesMut;
use std::io::{self, ErrorKind};
use super::message::Message;

const MAGIC: [u8; 4] = *b"LOCA";
const VERSION: u8 = 0x01;

pub struct MessageCodec;

impl Encoder<Message> for MessageCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Message, mut dst: &mut BytesMut) -> Result<(), Self::Error> {
        let payload_bytes = serde_json::to_vec(&item)
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
        
        let length = payload_bytes.len() as u32;
        
        dst.reserve(9 + payload_bytes.len());
        dst.extend_from_slice(&MAGIC);
        dst.extend_from_slice(&[VERSION]);
        dst.extend_from_slice(&[item.message_type as u8]);
        dst.extend_from_slice(&length.to_be_bytes());
        dst.extend_from_slice(&payload_bytes);
        
        Ok(())
    }
}

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 9 {
            return Ok(None);
        }

        if &src[0..4] != &MAGIC {
            return Err(io::Error::new(ErrorKind::InvalidData, "Invalid magic bytes"));
        }

        if src[4] != VERSION {
            return Err(io::Error::new(ErrorKind::InvalidData, "Unsupported version"));
        }

        let length = u32::from_be_bytes(src[6..10].try_into().unwrap()) as usize;
        
        if src.len() < 10 + length {
            return Ok(None);
        }

        let data = src.split_to(10 + length);
        let payload_bytes = &data[10..];
        
        let message: Message = serde_json::from_slice(payload_bytes)
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
        
        Ok(Some(message))
    }
}