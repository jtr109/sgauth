extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn create_jwt_secret(length: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect();

    rand_string
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_jwt_secret() {
        // unimplemented!();
        let length = rand::thread_rng().gen_range(8, 32);
        let rand_string = create_jwt_secret(length);
        assert_eq!(rand_string.len(), length);
    }
}
