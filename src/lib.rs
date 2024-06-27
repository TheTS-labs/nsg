//! Nsg is a library for interacting with Portal.
//! It provides methods for basic search, view request, work schedule and brief
//! request
//!
//! **Note**: This library is written to work only with users with the role of
//! "Монтажник (все)" It may not work properly with other roles
//!
//! ```
//! use chrono::Utc;
//! use nsg::Nsg;
//! # use nsg::error::NsgError;
//!
//! # tokio_test::block_on(async {
//! let nsg = Nsg::from_creds(
//!     "login".to_string(),
//!     "password_hash".to_string(),
//!     "https://net-stroy.itnet.lviv.ua".to_string(),
//!     "client".to_string(),
//!     "x.y".to_string(),
//! )
//! .await?;
//!
//! // Fetch orders from Work Schedule for today
//! let work_schedule = nsg.work_schedule(Utc::now().date_naive()).await;
//! println!("{:?}", work_schedule.0);
//!
//! # Ok::<(), NsgError>(())
//! # });
//! ```
//!
//! ## Terminology
//!
//! - **Internal order id**: Kyivstar assigns unique identifiers to its orders.
//!   When orders are processed by the Portal, it creates its own record of the
//!   order in its database, thus creating an internal identifier for the
//!   request
//! - **Data pillar**: Portal's endpoint or the way to access data from Portal.
//!   There are 4 data pillars: [`BasicSearch`](basic_search::BasicSearch),
//!   [`BriefRequest`](brief_request::BriefRequest),
//!   [`ViewRequest`](view_request::ViewRequest) and
//!   [`WorkSchedule`](work_schedule::WorkSchedule)
//! - **Portal**: Refers to "Нет-Строй"'s "IC Портал-К"
//! - **Kyivstar's version**: Kyivstar's data is not always 100% right. It's
//!   really apparent in the client's full name

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
#[doc(hidden)]
pub mod methods;

pub mod serializable_int_error_kind;
pub mod serializable_parse_error_kind;

pub(crate) mod traits;
#[macro_use]
pub(crate) mod macros;
pub(crate) mod payload;
#[cfg(test)]
mod tests;

/// Wrapper around data pillar parsers and Portal
#[derive(Clone, Debug)]
pub struct Nsg {
    /// Used session cookie value to interact with Portal
    pub session:     Option<String>,
    client:          Client,
    root:            String,
    http_client:     String,
    http_client_ver: String,

    login:         Option<String>,
    password_hash: Option<String>,
}

/// Implementation of essential wrapper methods
impl Nsg {
    /// Creates a new Nsg instance from credentials (login and password hash in
    /// md5) and performs relogin to get a session.
    ///
    /// **Note**: root must be without `/`, i.e. `https://net-stroy.itnet.lviv.ua` or `https://nsg.zp.ua`
    /// ```
    /// use nsg::Nsg;
    /// # use nsg::error::NsgError;
    ///
    /// # tokio_test::block_on(async {
    /// let nsg = Nsg::from_creds(
    ///     "login".to_string(),
    ///     "password_hash".to_string(),
    ///     "https://net-stroy.itnet.lviv.ua".to_string(),
    ///     "client".to_string(),
    ///     "x.y".to_string(),
    /// )
    /// .await?;
    ///
    /// println!("Session: {:?}", nsg.session);
    ///
    /// # Ok::<(), NsgError>(())
    /// # });
    /// ```
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

    /// If you already have a session, you can create an instance to use this
    /// session.
    ///
    /// **Note**: You can't use [Nsg::relogin] because it requires
    /// login and password hash to be set. If you have login and password hash
    /// simply use [Nsg::from_creds]
    ///
    /// ```
    /// use nsg::Nsg;
    /// # use nsg::error::NsgError;
    ///
    /// # tokio_test::block_on(async {
    /// let nsg = Nsg::from_session(
    ///     "75tlg96e5id8c3r0k0d6c8j4s0".to_string(),
    ///     "https://net-stroy.itnet.lviv.ua".to_string(),
    ///     "client".to_string(),
    ///     "x.y".to_string(),
    /// )
    /// .await;
    ///
    /// println!("Is session alive?: {}", nsg.is_session_alive().await?);
    ///
    /// # Ok::<(), NsgError>(())
    /// # });
    /// ```
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

    /// Perform request to get new session cookie.
    ///
    /// **Note**: Portal will return session disregarding creds, so after
    /// relogin you can check validness of session via
    /// [Nsg::is_session_alive]
    ///
    /// Panics if [Nsg::relogin] called on instance created with
    /// [Nsg::from_session] i.e. login or password hash is not set
    ///
    /// ```
    /// use nsg::Nsg;
    /// # use nsg::error::NsgError;
    ///
    /// # tokio_test::block_on(async {
    /// let mut nsg = Nsg::from_creds(
    ///     "login".to_string(),
    ///     "password_hash".to_string(),
    ///     "https://net-stroy.itnet.lviv.ua".to_string(),
    ///     "client".to_string(),
    ///     "x.y".to_string(),
    /// )
    /// .await?;
    ///
    /// println!("Session (before): {:?}", nsg.session);
    /// nsg.relogin().await?;
    /// println!("Session (after): {:?}", nsg.session);
    ///
    /// # Ok::<(), NsgError>(())
    /// # });
    /// ```
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

    /// Constructs headers with session cookie. Primarily used for
    /// [Nsg::request]
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

    /// Makes an arbitrary POST request to Portal. Url should be relative to
    /// root and shouldn't start with `/`. For example `headless.php`, not
    /// `/headless.php`. If `url` is `None`, then `headless.php` is used
    ///
    /// ```
    /// use nsg::Nsg;
    /// # use nsg::error::NsgError;
    ///
    /// # tokio_test::block_on(async {
    /// let nsg = Nsg::from_creds(
    ///     "login".to_string(),
    ///     "password_hash".to_string(),
    ///     "https://net-stroy.itnet.lviv.ua".to_string(),
    ///     "client".to_string(),
    ///     "x.y".to_string(),
    /// )
    /// .await?;
    ///
    /// let response = nsg
    ///     .request(nsg.construct_headers(), String::from(""), None)
    ///     .await?;
    /// let body = response.text().await?;
    ///
    /// println!("{}", body);
    ///
    /// # Ok::<(), NsgError>(())
    /// # });
    /// ```
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

    /// Performs empty request to check [current session](`Nsg::session`)
    /// validness
    ///
    /// ```
    /// use nsg::Nsg;
    /// # use nsg::error::NsgError;
    ///
    /// # tokio_test::block_on(async {
    /// let nsg = Nsg::from_creds(
    ///     "login".to_string(),
    ///     "password_hash".to_string(),
    ///     "https://net-stroy.itnet.lviv.ua".to_string(),
    ///     "client".to_string(),
    ///     "x.y".to_string(),
    /// )
    /// .await?;
    ///
    /// println!("Is session alive?: {}", nsg.is_session_alive().await?);
    ///
    /// # Ok::<(), NsgError>(())
    /// # });
    /// ```
    pub async fn is_session_alive(&self) -> Result<bool, NsgError> {
        let response = self.request(self.construct_headers(), String::from(""), None).await?;

        let body = response.text().await.unwrap();

        Ok(!body.contains("logoform"))
    }
}
