use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct User<'a> {
    _id: Option<ObjectId>,
    #[serde(borrow)]
    username: Cow<'a, str>,
}
