pub fn ft_swap(a: &mut isize, b: &mut isize) {
    let t = *a;
    *a = *b;
    *b = t;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let mut a = 10;
        let mut b = 20;
        ft_swap(&mut a, &mut b);
        assert_eq!(a, 20);
        assert_eq!(b, 10);
    }
}
