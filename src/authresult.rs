use serde::{Serialize, Deserialize};
use rocket::request::{FromForm, FromFormValue};

/// Result status of authentication flow
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthStatus {
    /// Authentication flow completed succesfully, resulting in attributes
    #[serde(rename = "succes")]
    Succes,
    /// Authentication flow completed unsuccesfully, no attributes were obtained
    #[serde(rename = "failed")]
    Failed,
}

impl<'v> FromFormValue<'v> for AuthStatus {
    type Error = &'v rocket::http::RawStr;

    fn from_form_value(form_value: &'v rocket::http::RawStr) -> Result<Self, &'v rocket::http::RawStr> {
        let value = form_value.url_decode().map_err(|_| form_value)?;
        if value == "succes" {
            Ok(AuthStatus::Succes)
        } else if value == "failed" {
            Ok(AuthStatus::Failed)
        } else {
            Err(form_value)
        }
    }
}

/// Result of an authentication flow
#[derive(Debug, Serialize, Deserialize, FromForm, PartialEq, Eq)]
pub struct AuthResult {
    /// Status of the result
    pub status: AuthStatus,
    /// Attribute jwe containing the obtained attributes
    pub attributes: Option<String>,
    /// URL on which the authentication plugin wants to be kept updated on session status
    pub session_url: Option<String>
}

/// Session activity status update type
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionActivity {
    /// User has had sufficient activity to extend session timeout
    #[serde(rename="user_active")]
    UserActive,
    /// User has indicated desire to logout
    #[serde(rename="logout")]
    Logout
}

impl<'v> FromFormValue<'v> for SessionActivity {
    type Error = &'v rocket::http::RawStr;

    fn from_form_value(form_value: &'v rocket::http::RawStr) -> Result<Self, Self::Error> {
        let value = form_value.url_decode().map_err(|_| form_value)?;
        if value == "user_active" {
            Ok(SessionActivity::UserActive)
        } else if value == "logout" {
            Ok(SessionActivity::Logout)
        } else {
            Err(form_value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_status_decode() {
        assert_eq!(AuthStatus::from_form_value("succes".into()).unwrap(), AuthStatus::Succes);
        assert_eq!(AuthStatus::from_form_value("failed".into()).unwrap(), AuthStatus::Failed);
        AuthStatus::from_form_value("assjde".into()).unwrap_err();
        AuthStatus::from_form_value("%%%sd%s%d".into()).unwrap_err();
    }

    #[test]
    fn test_session_activity_decode() {
        assert_eq!(SessionActivity::from_form_value("user_active".into()).unwrap(), SessionActivity::UserActive);
        assert_eq!(SessionActivity::from_form_value("logout".into()).unwrap(), SessionActivity::Logout);
        SessionActivity::from_form_value("assjde".into()).unwrap_err();
        SessionActivity::from_form_value("%%%sd%s%d".into()).unwrap_err();
    }
}
