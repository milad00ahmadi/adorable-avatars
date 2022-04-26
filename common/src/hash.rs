pub type HashFn = Box<dyn Fn(&str) -> i32 + Send + Sync>;

pub struct Hash<T>
where
    T: Clone,
{
    length: i32,
    values: Box<[T]>,
    hash_fn: HashFn,
}

impl<T> Hash<T>
where
    T: Clone,
{
    pub fn new(values: Box<[T]>, hash_fn: HashFn) -> Self {
        let length = values.len() as i32;
        assert_ne!(length, 0);

        Self {
            length,
            values,
            hash_fn,
        }
    }

    pub fn get(&self, input: &str) -> &T {
        let index = self.index_for(input);
        &self.values[index as usize]
    }

    fn index_for(&self, key: &str) -> i32 {
        let v: i32 = (self.hash_fn)(key);

        (v % self.length).abs()
    }
}

fn get_char_code(c: char) -> u16 {
    let mut codes = [0; 2];
    c.encode_utf16(&mut codes);
    return codes[0];
}

pub fn hash_factory<T: 'static>(reducer: T) -> HashFn
where
    T: Fn(Vec<i32>) -> i32 + Send + Sync,
{
    return Box::new(move |key: &str| -> i32 {
        let char_codes: Vec<i32> = key
            .chars()
            .map(get_char_code)
            .map(|value| value as i32)
            .collect();

        let char_codes_length = char_codes.len() as i32;
        reducer(char_codes) + char_codes_length
    });
}

#[cfg(test)]
mod tests {
    use crate::reducers::*;

    use super::*;

    fn create_hash() -> Hash<String> {
        Hash::new(
            vec![
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned(),
                "d".to_owned(),
            ]
            .into(),
            hash_factory(sum),
        )
    }

    #[test]
    fn test_converting_string_to_integer_by_its_char_codes_and_its_length() {
        pub fn reducer(_: Vec<i32>) -> i32 {
            1
        }

        let h = hash_factory(reducer);

        assert_eq!(h("foo"), "foo".len() as i32 + 1);
    }

    #[test]
    fn test_sum_hash_returns_zero_for_an_empty_string() {
        let h = hash_factory(sum);
        assert_eq!(h(""), 0);
    }

    #[test]
    fn test_converting_string_to_integer_by_summing_its_char_codes() {
        let h = hash_factory(sum);
        assert_eq!(h("foo"), 327);
    }

    #[test]
    fn test_hash_func_is_consistent() {
        let h = hash_factory(sum);
        for _ in 1..20 {
            assert_eq!(h("foo"), 327);
        }
    }

    #[test]
    fn return_a_value_given_a_key() {
        let hash: Hash<String> = create_hash();

        assert_eq!(hash.get("foo"), "d");
        assert_eq!(hash.get("bar"), "a");
        assert_eq!(hash.get("baz"), "a");
    }

    #[test]
    fn test_hash_struct_is_consistent() {
        let hash: Hash<String> = create_hash();

        for _ in 1..20 {
            assert_eq!(hash.get("foo"), "d");
        }
    }
}
