use super::*;

#[derive(Debug, Default, Serialize, PartialEq, Eq, ErrorIf, ApiError)]
#[api_error(error = "not_acceptable")]
pub struct Login {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
