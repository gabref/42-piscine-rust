pub fn ft_div_mod(a: i32, b: i32, div: &mut i32, md: &mut i32) {
    *div = a / b;
    *md = a % b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_mod() {
        let a = 10;
        let b = 5;
        let mut div = 0;
        let mut md = 0;
        ft_div_mod(a, b, &mut div, &mut md);
        assert_eq!(div, 2);
        assert_eq!(md, 0);
    }
}
