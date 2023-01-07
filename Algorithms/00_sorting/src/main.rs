fn main() {
    // let mut a = vec![9, 1, 8, 2, 7, 3];
    let mut a = vec![300, 200, 100];
    insertion_sort(&mut a);
    println!("a: {:?}", a);
}

fn insertion_sort(a: &mut Vec<i32>) {
/*
    THIS WORKS AS WEll
    for j in 1..a.len() {
        let mut i = j;
        while i > 0 && a[i-1] > a[i] {
            let temp = a[i-1];
            a[i-1] = a[i];
            a[i] = temp;
            i -= 1;
        }
    }
 */
    for j in 1..a.len() {
        let key = a[j];
        let mut i: i32 = j as i32 - 1;
        while i >= 0 && a[i as usize] > key {
            a[(i+1) as usize] = a[i as usize];
            i -= 1;
        }
        a[(i+1) as usize] = key;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut a = vec![9, 1, 8, 2, 7, 3];
        insertion_sort(&mut a);
        assert_eq!(a, vec![1, 2, 3, 7, 8, 9]);
    }
}
