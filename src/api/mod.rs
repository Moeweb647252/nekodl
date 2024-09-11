use salvo::Router;
use serde::Serialize;

mod login;

pub fn routes() -> Vec<Router> {
    todo!()
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    code: Code,
    data: T,
    msg: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(code: Code, data: T, msg: &str) -> Self {
        Self {
            code,
            data,
            msg: msg.to_owned(),
        }
    }
}

#[derive(Debug)]
enum Code {
    Success,
    AuthenticationError,
}

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(match self {
            Self::Success => 200,
            Self::AuthenticationError => 501,
        })
    }
}


