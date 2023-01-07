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
        /*
        The book uses 1 based indexes.
        If i > 0 is checked, the 1st (index 0) element
        is skipped. If index 0 is included i goes
        below zero in the loop. But usize can not
        go below zero, so that variable needs
        to be type cast when it's used as an index.
        */
        while i >= 0 && a[i as usize] > key {
            a[(i+1) as usize] = a[i as usize];
            // This goes below zero for the 1st element
            // So this can't just be usize!
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
