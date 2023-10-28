
pub fn ft_ultimate_div_mod(a: &mut isize, b: &mut isize) {
    *a = *a / *b;
    *b = (*a * *b) % *b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ultimate_div_mod() {
        let mut a = 10;
        let mut b = 5;
        ft_ultimate_div_mod(&mut a, &mut b);
        assert_eq!(a, 2);
        assert_eq!(b, 0);
    }
}
