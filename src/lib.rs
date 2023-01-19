//!
//!
//! HyperDB Rust Client
//!
//! Author: [Afaan Bilal](https://afaan.dev)
//!
//! Source: [GitHub](https://github.com/AfaanBilal/hyperdb-rs)
//!
use core::panic;
use reqwest::{blocking::Client, Method, Result};

pub const R_OK: &str = "OK";
pub const R_TRUE: &str = "YES";
const R_PONG: &str = "PONG";
const R_INVALID_CREDENTIALS: &str = "INVALID_CREDENTIALS";
const R_AUTH_FAILED: &str = "AUTH_FAILED";

pub struct HyperClient {
    address: String,
    username: String,
    password: String,
    auth_enabled: bool,
    token: String,
    client: Option<Client>,
}

impl HyperClient {
    pub fn new(address: String) -> HyperClient {
        let mut hc = HyperClient {
            address,
            username: String::from(""),
            password: String::from(""),
            auth_enabled: false,
            token: String::from(""),
            client: None,
        };

        hc.connect();

        hc
    }

    #[allow(dead_code)]
    pub fn authenticate(&mut self, username: String, password: String) {
        self.username = username;
        self.password = password;
        self.auth_enabled = true;
        self.auth().expect("Authentication failed");
    }

    fn client(&self) -> Client {
        match &self.client {
            Some(c) => c.to_owned(),
            None => Client::new(),
        }
    }

    fn auth(&mut self) -> Result<String> {
        let client = Client::new();
        let resp = client
            .post(&self.address)
            .header("username", &self.username)
            .header("password", &self.password)
            .send()?
            .text()?;

        if resp == R_INVALID_CREDENTIALS {
            panic!("Invalid credentials.");
        }

        self.token = resp.trim().to_string();

        Ok(resp)
    }

    pub fn connect(&mut self) {
        let r = self.http(Method::GET, &format!("ping"), "");

        match r {
            Err(e) => panic!("Unable to connect to the HyperDB server (E_001): {}", e),
            Ok(r) => {
                if r != R_PONG {
                    panic!("Unable to connect to the HyperDB server (E_002).")
                }

                println!("Connected to {}", self.address);
            }
        }
    }

    pub fn ping(&self) -> Result<String> {
        self.client()
            .get(String::from(&self.address) + "/ping")
            .send()?
            .text()
    }

    pub fn http(&mut self, method: Method, url: &String, body: &str) -> Result<String> {
        let m = method.clone();

        let mut r = self
            .client()
            .request(method, format!("{}/{}", &self.address, url))
            .header("Auth", &self.token)
            .body(body.to_string())
            .send()?
            .text()?;

        if r == R_AUTH_FAILED && self.auth_enabled {
            self.auth()?;
            r = self
                .client()
                .request(m, format!("{}/{}", &self.address, url))
                .header("Auth", &self.token)
                .body(body.to_string())
                .send()?
                .text()?;
        }

        Ok(r)
    }

    pub fn version(&mut self) -> Result<String> {
        self.http(Method::GET, &format!(""), "")
    }

    pub fn has(&mut self, key: &str) -> Result<String> {
        self.http(Method::GET, &format!("has/{}", key), "")
    }

    pub fn get(&mut self, key: &str) -> Result<String> {
        self.http(Method::GET, &format!("data/{}", key), "")
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<String> {
        self.http(Method::POST, &format!("data/{}", key), value)
    }

    pub fn delete(&mut self, key: &str) -> Result<String> {
        self.http(Method::DELETE, &format!("data/{}", key), "")
    }

    pub fn all(&mut self) -> Result<String> {
        self.http(Method::GET, &format!("data"), "")
    }

    pub fn clear(&mut self) -> Result<String> {
        self.http(Method::DELETE, &format!("data"), "")
    }

    pub fn empty(&mut self) -> Result<String> {
        self.http(Method::GET, &format!("empty"), "")
    }

    pub fn save(&mut self) -> Result<String> {
        self.http(Method::POST, &format!("save"), "")
    }

    pub fn reload(&mut self) -> Result<String> {
        self.http(Method::POST, &format!("reload"), "")
    }

    pub fn reset(&mut self) -> Result<String> {
        self.http(Method::DELETE, &format!("reset"), "")
    }
}
