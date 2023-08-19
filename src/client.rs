use reqwest::Client;

/// A struct representing authentication credentials.
pub struct Auth {
    /// The username for authentication.
    pub username: String,
    /// The password for authentication.
    pub password: String,
}

/// A struct representing the configuration for making requests to the MTA service.
pub struct Mta {
    /// The IP address of the MTA server.
    pub ip: String,
    /// The port number of the MTA server.
    pub port: u16,
    /// The authentication credentials for accessing the MTA server.
    pub auth: Auth,
    /// Determines whether HTTP or HTTPS should be used for requests.
    pub http: bool,
}

impl Mta{
    /// Creates a new `Mta` instance.
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address of the MTA server.
    /// * `port` - The port number of the MTA server.
    /// * `auth` - Authentication credentials for accessing the MTA server.
    /// * `http` - Whether to use HTTP (`true`) or HTTPS (`false`) for requests.
    ///
    /// # Example
    ///```rust
    /// use mta_sdk::client;
    ///
    /// let auth = client::Auth {
    ///     username: "user".to_string(),
    ///     password: "pass".to_string(),
    /// };
    ///
    /// let mta = client::Mta::new("127.0.0.1".to_string(), 22005, auth, true);
    ///```
    pub fn new(ip: String, port: u16, auth: Auth, http: bool) -> Mta {
        Mta {
            ip,
            port,
            auth,
            http
        }
    }

        /// Makes a remote call to the MTA service.
        ///
        /// This method sends a POST request to the specified resource and function
        /// on the MTA server, passing the provided arguments.
        ///
        /// # Arguments
        ///
        /// * `resource` - The resource to call.
        /// * `function` - The function to invoke.
        /// * `args` - The arguments for the function call.
        ///
        /// # Returns
        ///
        /// Returns a `Result` containing the response text if the request is successful,
        /// or an `Errors` enum if an error occurs during the request.
        ///
        /// # Errors
        ///
        /// Returns an `HTTP404ERROR` variant of the `Errors` enum if there is a request error.
        ///
        /// # Example
        ///
        /// ```rust
        /// use mta_sdk::client;
        ///
        /// #[tokio::main]
        /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
        ///     let auth = client::Auth {
        ///         username: "user".to_string(),
        ///         password: "pass".to_string(),
        ///     };
        ///
        ///     let mta_instance = client::Mta::new("127.0.0.1".to_string(), 22005, auth, true);
        ///     let response = mta_instance.call("resource", "function", vec!["arg1".to_string()]).await?;
        ///     println!("Response: {}", response);
        ///
        ///     Ok(())
        /// }
        /// ```
    pub async fn call(&self, resource: &str, function: &str, args: Vec<String>) -> Result<String, crate::error::Errors> {
        let url = format!("{}{}/call/{}", self.get_uri(), resource, function);

        let client = Client::new();
        let res = client.post(&url)
            .header("Content-Type", "application/json")
            .basic_auth(&self.auth.username, Some(&self.auth.password))
            .json(&args)
            .send()
            .await;

        match res {
            Ok(res) => {
                let status = res.status().to_string();
                if status == "200 OK" {
                    let body = res.text().await.unwrap();
                    Ok(body)
                } else {
                    let err = crate::error::Errors::RequestError(format!("The server returned an error. Status: {} 🛑 .", status));
                    Err(err)
                }
            },
            Err(_err) => {
                let err = crate::error::Errors::RequestError("There was a failure in transmitting your request to the server. We recommend checking the connection and trying again.".to_string());
                Err(err)
            }
        }
    }

    /// Returns the URI based on the configured HTTP/HTTPS setting.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use mta_sdk::client;
    ///
    /// let auth = client::Auth {
    ///     username: "user".to_string(),
    ///     password: "pass".to_string(),
    /// };
    ///
    /// let instance = client::Mta::new("127.0.0.1".to_string(), 22005, auth, true);
    ///
    /// let uri = instance.get_uri();
    /// println!("URI: {}", uri);
    /// ```
    pub fn get_uri (&self) -> String {
        if self.http {
            format!("http://{}:{}/", self.ip, self.port)
        } else {
            format!("https://{}:{}/", self.ip, self.port)
        }
    }
}