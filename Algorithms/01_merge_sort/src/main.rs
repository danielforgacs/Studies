fn main() {
}

fn merge_sort(a: Vec<u8>) -> Vec<u8> {
    if a.len() <= 2 {
        return a;
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_n0() {
        let array = vec![];
        assert_eq!(merge_sort(array), vec![]);
    }

    #[test]
    fn test_merge_sort_n1() {
        let array = vec![1];
        assert_eq!(merge_sort(array), vec![1]);
    }
}
