fn main() {
    merge_sort(&mut vec![1, 2]);
}

fn merge_sort(a: &mut Vec<u8>) {
    let l =  a.len();
    if l <= 1 {
        return;
    }
    let h = l % 2;
    println!(":::array lenght:{}", l);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_n0() {
        let mut array = vec![];
        merge_sort(&mut array);
        assert_eq!(array, vec![]);
    }

    #[test]
    fn test_merge_sort_n1() {
        let mut array = vec![1];
        merge_sort(&mut array);
        assert_eq!(array, vec![1]);
    }

    #[test]
    fn test_merge_sort_n2() {
        let mut array = vec![2, 1];
        merge_sort(&mut array);
        assert_eq!(array, vec![1, 2]);
    }
}
