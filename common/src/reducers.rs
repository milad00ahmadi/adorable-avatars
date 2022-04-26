pub fn sum(iter: Vec<i32>) -> i32 {
    iter.into_iter().sum()
}

pub fn product(iter: Vec<i32>) -> i32 {
    iter.into_iter().product::<i32>()
}

pub fn sum_and_diff(iter: Vec<i32>) -> i32 {
    iter.into_iter().enumerate().fold(0, |acc, item| {
        if item.0 % 2 == 0 {
            acc + item.1
        } else {
            acc - item.1
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let res = sum(vec![1, 2, 3]);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_sum_with_empty_vector() {
        let res = sum_and_diff(vec![]);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_product() {
        let res = product(vec![1, 2, 3, 4]);
        assert_eq!(res, 24);
    }

    #[test]
    fn test_product_with_empty_vector() {
        let res = product(vec![]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_sum_and_diff() {
        let res = sum_and_diff(vec![1, 2, 3, 4]);
        assert_eq!(res, -2);
    }

    #[test]
    fn test_sum_and_diff_with_empty_vector() {
        let res = sum_and_diff(vec![]);
        assert_eq!(res, 0);
    }
}
