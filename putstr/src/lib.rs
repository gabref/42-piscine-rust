use std::io::Write;

pub fn ft_putstr(str: String, output: &mut impl Write) {
    let size = str.len();
    let new_str = str.as_bytes();
    for i in 0..size {
        output.write(&[new_str[i]]).unwrap();
    }
}

pub fn ft_putstr2(str: &str, output: &mut impl Write) {
    let bytes_str = str.as_bytes();
    for c in bytes_str {
        output.write(&[*c]).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_putstr() {
        let str = String::from("ciao bello!");
        let mut output = Vec::new();
        ft_putstr(str.clone(), &mut output);
        assert_eq!(&output, b"ciao bello!");
    }

    #[test]
    fn test_putstr2() {
        let str = String::from("ciao bello!");
        let mut output = Vec::new();
        ft_putstr2(&str, &mut output);
        assert_eq!(&output, b"ciao bello!");
    }
}
