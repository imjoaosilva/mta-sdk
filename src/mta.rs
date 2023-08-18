use reqwest::{Client, RequestBuilder};

pub struct Auth {
    pub username: String,
    pub password: String,
}

pub struct Mta {
    pub ip: String,
    pub port: u16,
    pub auth: Auth,
    pub http: bool,
}

impl Mta{
    pub fn new(ip: String, port: u16, auth: Auth, http: bool) -> Mta {
        Mta {
            ip,
            port,
            auth,
            http
        }
    }
    
    pub fn call(&self,resource: &str, function: &str, args: Vec<String>) -> String {
        let url = format!("{}{}/call/{}", self.get_uri(), resource, function);
        let client = Client::new();
    }

    pub fn get_uri (&self) -> String {
        if self.http {
            format!("http://{}:{}/", self.ip, self.port)
        } else {
            format!("https://{}:{}/", self.ip, self.port)
        }
    }


}