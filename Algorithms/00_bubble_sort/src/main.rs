fn main() {
    println!("Hello, world!");
}

fn bubble_sort(a: &mut Vec<usize>) {

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
