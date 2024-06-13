use error::{NsgError, NsgResult};
use payload::LoginPayload;
use reqwest::header::HeaderMap;
use reqwest::Client;

pub mod basic_search;
pub mod brief_request;
pub mod data;
pub mod view_request;
pub mod work_schedule;

pub mod error;
pub mod methods;

pub(crate) mod traits;
#[macro_use]
pub(crate) mod macros;
pub(crate) mod payload;
pub(crate) mod serializable_int_error_kind;
pub(crate) mod serializable_parse_error_kind;
#[cfg(test)]
mod tests;

/// Main struct and entry point for interacting with Nsg website
#[derive(Clone, Debug)]
pub struct Nsg {
    pub session:     Option<String>,
    client:          Client,
    root:            String,
    http_client:     String,
    http_client_ver: String,

    login:         Option<String>,
    password_hash: Option<String>,
}

impl Nsg {
    /// Creates a new Nsg instance from credentials
    pub async fn from_creds(
        login: String,
        password_hash: String,
        root: String,
        http_client: String,
        http_client_ver: String,
    ) -> Result<Nsg, NsgError> {
        let mut nsg = Nsg {
            client: Client::builder().build().unwrap(),
            session: None,
            root,
            http_client,
            http_client_ver,

            login: Some(login),
            password_hash: Some(password_hash),
        };

        nsg.relogin()
            .await
            .map_err(|err| NsgError::FailedToRelogin(Box::new(err)))?;

        Ok(nsg)
    }

    /// Creates a new Nsg instance from existing session
    pub async fn from_session(session: String, root: String, http_client: String, http_client_ver: String) -> Nsg {
        Nsg {
            client: Client::builder().build().unwrap(),
            session: Some(session),
            root,
            http_client,
            http_client_ver,

            login: None,
            password_hash: None,
        }
    }

    /// Fetches and sets a new session code, i.e., relogin. Panics if
    /// password_hash and login are None, i.e. struct created via
    /// [Nsg::from_session]
    pub async fn relogin(&mut self) -> NsgResult<()> {
        let headers = self.construct_headers();
        let payload = Nsg::construct_login_payload(
            self.login.as_ref().expect("No login"),
            self.password_hash.as_ref().expect("No password hash"),
        )?;

        let response = self.request(headers, payload, None).await?;

        let session_cookie = response.cookies().next().ok_or(NsgError::NoSessionCookie)?;
        self.session = Some(session_cookie.value().to_string());

        Ok(())
    }

    /// Constructs headers with auth for requests. Suitable for all requests
    pub fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert("accept", "*/*".parse().unwrap());
        headers.insert(
            "content-type",
            "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap(),
        );
        headers.insert(
            "user-agent",
            format!("{}/{}", self.http_client, self.http_client_ver)
                .parse()
                .unwrap(),
        );

        if let Some(session) = &self.session {
            headers.insert("cookie", format!("SESSION_nsg={}", session).parse().unwrap());
        }

        headers
    }

    fn construct_login_payload(login: &str, password_hash: &str) -> Result<String, NsgError> {
        let payload = LoginPayload {
            action: "user/login",
            login,
            password: password_hash,
            remember: 0,
            olduri: "",
            token: "",
        };

        serde_urlencoded::to_string(payload).map_err(|_| NsgError::UrlEncodeError)
    }

    /// Makes a request to Nsg website. Url should be relative to root and
    /// shouldn't start with /. For example `headless.php`, not `/headless.php`.
    /// If url is None, then `headless.php` is used
    pub async fn request(
        &self,
        headers: HeaderMap,
        payload: String,
        url: Option<&str>,
    ) -> Result<reqwest::Response, NsgError> {
        let request_url = match url {
            Some(url) => format!("{}/{}", self.root, url),
            None => format!("{}/headless.php", self.root),
        };

        let response = self
            .client
            .request(reqwest::Method::POST, request_url)
            .headers(headers)
            .body(payload)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn is_session_alive(&self) -> Result<bool, NsgError> {
        let response = self.request(self.construct_headers(), String::from(""), None).await?;

        let body = response.text().await.unwrap();

        Ok(!body.contains("logoform"))
    }
}
