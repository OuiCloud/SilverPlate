//! A minimal Rust library project with
//! - copyright
//! - licences
//! - code of conduct
//!
//! README should be sync with this doc with `cargo-sync-rdme`.
//! More informations here :
//! https://github.com/gifnksm/cargo-sync-rdme

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
