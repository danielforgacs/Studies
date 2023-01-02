fn main() {
}

fn merge_sort(a: &mut Vec<u8>) {
    *a = vec![2, 1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut array = vec![2, 1];
        merge_sort(&mut array);
        assert_eq!(array, vec![1, 2]);
    }
}
