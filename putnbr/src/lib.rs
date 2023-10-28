use std::io::Write;

pub fn ft_putnbr(nb: i32, output: &mut impl Write)
{
    let mut n: i64 = i64::try_from(nb).ok().unwrap();
    if n < 0
    {
        output.write(b"-").unwrap();
        n = n * -1;
    }
    if n > 9 {
        ft_putnbr((n / 10) as i32, output);
    }
    let c = [(n % 10) as u8 + b'0'];
    output.write(&c).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_putnbr_positive() {
        let mut output = Vec::new();
        ft_putnbr(42, &mut output);
        assert_eq!(&output, b"42");
    }

    #[test]
    fn test_putnbr_null() {
        let mut output = Vec::new();
        ft_putnbr(0, &mut output);
        assert_eq!(&output, b"0");
    }

    #[test]
    fn test_putnbr_negative() {
        let mut output = Vec::new();
        ft_putnbr(-42, &mut output);
        assert_eq!(&output, b"-42");
    }
}
