use std::io::Cursor;
use std::io::Read;
use std::io::Write;

pub mod error;

pub struct Attachment {
    pub ip: String,
    pub token: String,
}
impl Attachment {
    pub fn to_bytes(self) -> Vec<u8> {
        let ip = self.ip.as_bytes().to_vec();
        let token = self.token.as_bytes().to_vec();
        let ip_len = ip.len().to_be_bytes();
        let token_len = token.len().to_be_bytes();

        let mut bytes = Vec::new();
        bytes.extend(ip_len);
        bytes.extend(token_len);
        bytes.extend(ip);
        bytes.extend(token);
        bytes
    }
    pub fn from_bytes(vec: Vec<u8>) -> Result<Attachment, crate::error::Error> {
        let mut buf = Cursor::new(vec);
        let mut ip_len = [0u8; 4];
        buf.read_exact(&mut ip_len)?;
        let ip_len = u32::from_be_bytes(ip_len);

        let mut token_len = [0u8; 4];
        buf.read_exact(&mut token_len)?;
        let token_len = u32::from_be_bytes(token_len);

        let mut ip = vec![0u8; ip_len as usize];
        buf.read_exact(&mut ip)?;
        let ip = String::from_utf8(ip)?;

        let mut token = vec![0u8; token_len as usize];
        buf.read_exact(&mut token)?;
        let token = String::from_utf8(token)?;

        Ok(Attachment { ip, token })
    }
}
pub fn attach(ip: &str, token: &str) -> Attachment {
    Attachment {
        ip: ip.to_owned(),
        token: token.to_owned(),
    }
}
