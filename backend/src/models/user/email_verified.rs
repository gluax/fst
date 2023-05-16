use chrono::Duration;

use super::*;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EmailVerification {
    #[serde(rename = "_id")]
    id: ObjectId,
    email: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct EmailVerificationCollection {
    collection: Collection<EmailVerification>,
}

#[autometrics::autometrics]
impl EmailVerificationCollection {
    pub fn new() -> Self {
        Self {
            collection: get_mongodb().collection("email_verifications"),
        }
    }

    // TODO: @gluax... still repetitive
    pub async fn find(&self, id: &ObjectId) -> salvo::Result<Option<EmailVerification>> {
        let filter = doc! { "id": id };
        let email_verification = self.collection.find_one(filter, None).await.expect("todo");
        Ok(email_verification)
    }

    pub async fn create(
        &self,
        data: EmailVerificationTokenMessage,
    ) -> salvo::Result<EmailVerification> {
        let email_verification = EmailVerification::try_from(data)?;
        self.collection
            .insert_one(&email_verification, None)
            .await
            .expect("todo");
        Ok(email_verification)
    }

    pub async fn delete(&self, id: &ObjectId) -> salvo::Result<()> {
        let filter = doc! { "id": id };
        self.collection
            .delete_one(filter, None)
            .await
            .expect("todo");
        Ok(())
    }
}

#[autometrics::autometrics]
impl TryFrom<EmailVerificationTokenMessage> for EmailVerification {
    type Error = salvo::Error;
    fn try_from(value: EmailVerificationTokenMessage) -> salvo::Result<Self> {
        let created_at = Utc::now();
        let expires_at = created_at + Duration::hours(12);
        Ok(Self {
            id: Default::default(),
            email: value.email,
            created_at,
            expires_at,
        })
    }
}
