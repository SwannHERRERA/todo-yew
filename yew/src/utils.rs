use rand::{Rng, distributions::Alphanumeric};

pub fn generate_random_id() -> String {
    rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(7)
    .map(char::from)
    .collect::<String>()
}
