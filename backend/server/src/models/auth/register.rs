use super::*;

#[derive(Debug, Deserialize)]
pub struct Register<'a> {
    #[serde(borrow)]
    username: Cow<'a, str>,
    #[serde(borrow)]
    password: Cow<'a, str>,
    #[serde(borrow)]
    password_confirmation: Cow<'a, str>,
}


// impl Summary for RegisterSummary {
//     fn return_summary(self) -> Option<Self> {
//         if self.username.is_none() && self.password.is_none() {
//             None
//         } else {
//             Some(self)
//         }
//     }
// }

// impl<'a> Validate for Register<'a> {
//     type Summary = RegisterSummary;
//     fn validate(&self) -> Option<Self::Summary> {
//         let mut summary = Self::Summary::default();

//         if self.username.is_empty() {
//             summary.username = Some("Username cannot be empty.".to_string());
//         }

//         match (self.password.as_ref(), self.password_confirmation.as_ref()) {
//             (pw, _) if pw.is_empty() => {
//                 summary.password = Some("Password cannot be empty.".to_string())
//             }
//             // TODO if pw contains length is too long or short
//             // TODO if pw contains invalid characters
//             // (pw, _) if pw
//             (_, cf) if cf.is_empty() => {
//                 summary.password_confirmation =
//                     Some("Password confirmation cannot be empty.".to_string())
//             }
//             (pw, cf) if pw != cf => {
//                 summary.password =
//                     Some("Password and Password Confirmation must match.".to_string())
//             }
//             _ => {}
//         }

//         summary.return_summary()
//     }
// }
