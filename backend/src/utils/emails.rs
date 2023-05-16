use lettre::{
    message::{header, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[autometrics::autometrics]
pub async fn send_email<T: AsRef<str>, B: ToString>(
    credentials: &Credentials,
    from: &Mailbox,
    to: T,
    body: B,
) -> Result<(), ()> {
    let email = Message::builder()
        .from(from.clone())
        .to(to.as_ref().parse().expect("TODO: Failed to parse to email"))
        // TODO: @gluax take subject as an arg
        .subject("Email Verification Required")
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from(
                            "Please click the link below to confirm your email address.",
                        )),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        // .header(header::ContentTransferEncoding::Base64)
                        .body(body.to_string()),
                ),
        )
        .expect("TODO: failed to build email");

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .expect("TODO: failed to build email credentials")
            .credentials(credentials.clone())
            .build();

    mailer
        .send(email)
        .await
        .expect("TODO: Failed to send email");

    Ok(())
}
