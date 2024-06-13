use serde::Serialize;

#[derive(Serialize)]
#[doc(hidden)]
pub struct LoginPayload<'a> {
    pub action:   &'a str,
    pub login:    &'a str,
    pub password: &'a str,
    pub remember: u8,
    pub olduri:   &'a str,
    pub token:    &'a str,
}

#[derive(Serialize)]
#[doc(hidden)]
pub struct Payload<'a> {
    pub action: &'a str,
    pub city:   &'a str,
    pub data:   &'a str,
}
