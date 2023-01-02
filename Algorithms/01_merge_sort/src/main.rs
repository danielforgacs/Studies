fn main() {
}

fn merge_sort(a: Vec<u8>) -> Vec<u8> {
    if a.len() == 1 {
        return a;
    } else if a.len() == 2 {
        if a[0] > a[1] {
            return vec![a[1], a[0]];
        }
        return a;
    } else {
        let mut result: Vec<u8> = Vec::new();
        let half: usize = (a.len() / 2) as usize;
        let a1: Vec<u8> = a[0..half].to_vec();
        let a2: Vec<u8> = a[half + 1..a.len() - 1].to_vec();

        result.extend(merge_sort(a1));
        result.extend(merge_sort(a2));

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_n2() {
        let mut array = vec![2, 1];
        assert_eq!(merge_sort(array), vec![1, 2]);
    }

    #[test]
    fn test_merge_sort_n3() {
        let mut array = vec![3, 2, 1];
        assert_eq!(merge_sort(array), vec![1, 2, 3]);
    }
}
