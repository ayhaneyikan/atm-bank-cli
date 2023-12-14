pub const BANK_SERVER_ADDR: &str = "127.0.0.1:32001";

#[repr(u8)]
pub enum RequestType {
    AuthUser,
}

impl TryFrom<u8> for RequestType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AuthUser),
            _ => Err(()),
        }
    }
}