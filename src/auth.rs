use rocket::http::Status;
use rocket::request::{Request, Outcome, FromRequest};

pub struct BasicAuth {
    username: String,
    password: String
}

impl BasicAuth {
    fn from_auth_header(header: &str) -> Option<BasicAuth>{
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2  || split[0] != "Basic"{
            return None
        }
    
        return Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth>{
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        if split.len() != 2 {
            return None
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());

        return Some(BasicAuth { 
            username, 
            password,
        })
    }
}

#[rocket::async_trait]
impl <'r> FromRequest <'r> for BasicAuth {
    type Error = ();
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error>{
        let auth_header = request.headers().get_one("Authorization");

        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_auth_header(auth_header) {
                return Outcome::Success(auth)
            }
        }

        return Outcome::Failure((Status::Unauthorized, ()));
    }

}