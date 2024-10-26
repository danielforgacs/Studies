fn main() {
    // let mut a = vec![555, 444, 333, 222, 111];
    let mut a = vec![777, 666, 555, 444, 333, 222, 111];
    // let mut a = vec![555, 444, 333, 222, 111];
    merge_sort(&mut a);
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

        from the book:
            while i > 0 && a[i as usize] > key {
        this is a fix:
            while i >= 0 && a[i as usize] > key {
        */
        while i > -1 && a[i as usize] > key {
            a[(i+1) as usize] = a[i as usize];
            // This goes below zero for the 1st element (index 0)
            // So this can't just be usize!
            i -= 1;
        }
        a[(i+1) as usize] = key;
    }
}

fn insertion_sort_no_cast(a: &mut Vec<i32>) {
    /*
    In this version the index does not go
    below zero while working, so no type cast is needed.
    But this way the indexes are shifted with one higher.
    */
    for k in 2..=a.len() {
        let key = a[k - 1];
        let mut i = k - 1;
        while i > 0 && a[i - 1] > key {
            a[i] = a[i - 1];
            i -= 1;
        }
        a[i] = key;
    }
}

fn merge_sort(a: &mut Vec<i32>) {
    // if the array lenght is 1 we can start merging
    if a.len() < 2 {
        return;
    }
    // find the middle to cut the array in two sub-array
    let m = a.len() / 2;
    let mut l = a[..m].to_owned();
    let mut r = a[m..].to_owned();
    // sorting left half, right half independently before merging them.
    merge_sort(&mut l);
    merge_sort(&mut r);
    // going through the array we take the smaller element
    // from it's two already sorted subarrays picking always
    // the smaller element. If one of the two sub-arrays
    // runs out of elements, the other sorted sub-arrays
    // elements are added.
    let mut li = 0;
    let mut ri = 0;
    for i in 0..a.len() {
        if li == l.len() {
            a[i] = r[ri];
            ri += 1;
            continue;
        }
        if ri == r.len() {
            a[i] = l[li];
            li += 1;
            continue;
        }
        if l[li] < r[ri] {
            a[i] = l[li];
            li += 1;
        } else {
            a[i] = r[ri];
            ri += 1;
        }
    }


}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut a = vec![7, 6, 5, 4, 3, 2, 1];
        insertion_sort(&mut a);
        assert_eq!(a, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_insertion_sort_no_cast() {
        let mut a = vec![7, 6, 5, 4, 3, 2, 1];
        insertion_sort_no_cast(&mut a);
        assert_eq!(a, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_merge_sort() {
        let mut a = vec![999, 888, 777, 666, 555, 444, 333, 222, 111];
        merge_sort(&mut a);
        assert_eq!(a, vec![111, 222, 333, 444, 555, 666, 777, 888, 999]);
    }
}
