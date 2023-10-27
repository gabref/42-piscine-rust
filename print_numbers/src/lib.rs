use std::io::{Write, Error};

pub fn ft_print_numbers(output: &mut impl Write) -> Result<(), Error> {
    let mut number: u8 = b'0';

    while number <= b'9' {
        output.write(&[number]).unwrap();
        number += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_all_numbers() {
        let mut output: Vec<u8> = Vec::new();
        ft_print_numbers(&mut output).unwrap();
        assert_eq!(&output, b"0123456789");
    }
}
