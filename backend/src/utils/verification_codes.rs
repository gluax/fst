use rand::{distributions::Alphanumeric, Rng};

#[autometrics::autometrics]
pub fn generate_verification_code(length: usize) -> String {
    let code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    code
}
