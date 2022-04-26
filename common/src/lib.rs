extern crate core;

pub mod hash;
pub mod reducers;

pub use hash::Hash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_fn() {
        // let hash = Hash::new(vec![1, 2, 3, 4], |key| 2);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
