use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct User<'a> {
    #[serde(borrow)]
    username: Cow<'a, str>,
}
