fn main() {
    merge_sort(vec![1, 2]);
}

fn merge_sort(a: Vec<u8>) -> Vec<u8> {
    println!("a: {:?}", a);
    if a.len() <= 2 {
        return a;
    } else {

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

    #[test]
    fn test_merge_sort_n2() {
        let array = vec![2, 1];
        assert_eq!(merge_sort(array), vec![1, 2]);
    }
}
