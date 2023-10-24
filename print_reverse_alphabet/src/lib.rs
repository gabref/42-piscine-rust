use std::io::{Error, Write};

pub fn ft_print_reverse_alphabet(output: &mut impl Write) -> Result<(), Error> {
    let mut letter: u8 = b'z';

    while letter >= b'a' {
        output.write(&[letter]).unwrap();
        letter -= 1;
    }

    Ok(())
}

pub fn ft_print_reverse_alphabet_chatgpt(output: &mut impl Write) {
    let start_char = 'z';
    let end_char = 'a';

    for c in (end_char..=start_char).rev() {
        output.write(&[c as u8]).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_reverse_alphabet() {
        let mut output: Vec<u8> = Vec::new();

        ft_print_reverse_alphabet(&mut output).unwrap();

        assert_eq!(&output, b"zyxwvutsrqponmlkjihgfedcba");
    }

    #[test]
    fn write_reverse_alphabet_chatgpt() {
        let mut output: Vec<u8> = Vec::new();

        ft_print_reverse_alphabet_chatgpt(&mut output);

        assert_eq!(&output, b"zyxwvutsrqponmlkjihgfedcba");
    }
}
