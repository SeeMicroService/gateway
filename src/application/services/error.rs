use reqwest::StatusCode;
use std::io;

pub fn map_status(status: StatusCode) -> io::Result<()> {
    match status {
        _ if status.is_success() => Ok(()),
        StatusCode::NOT_FOUND => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Can not find entity by given keys",
        )),
        StatusCode::CONFLICT => Err(io::Error::new(io::ErrorKind::AlreadyExists, "conflict")),
        _ if status.is_client_error() => {
            Err(io::Error::new(io::ErrorKind::InvalidData, "bad request"))
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, status.to_string())),
    }
}
