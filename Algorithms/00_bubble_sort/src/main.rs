fn main() {
    let mut a = vec![2, 3, 1, 5, 4];
    bubble_sort(&mut a);
}

fn bubble_sort(a: &mut Vec<usize>) {
    // println!("sort");
    // dbg!(0..a.len());
    let length = a.len();
    dbg!(&length);
    for i in 0..length - 1 {
        dbg!(&i);
        // let inner_range = length..i + 1;
        // let inner_range = (0..3).rev();
        let inner_range = (i + 1..length).rev();
        dbg!(&inner_range);
        for j in inner_range {
            dbg!(&j);
            if a[j] < a[j-1] {
                let temp = a[j];
                a[j] = a[j-1];
                a[j-1] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let mut a = vec![2, 1];
        bubble_sort(&mut a);
        assert_eq!(a, vec![1, 2]);
    }
}
