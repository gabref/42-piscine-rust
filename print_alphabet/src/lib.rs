use std::io::{Error, Write};

pub fn ft_print_alphabet(output: &mut impl Write) -> Result<(), Error> {
    let mut letter: u8 = 97;

    loop {
        if letter > 122 {
            break;
        }
        output.write(&[letter]).unwrap();
        letter += 1;
    }

    Ok(())
}

pub fn ft_print_alphabet_chatgpt(output: &mut impl Write) {
    let mut letter: u8 = b'a';

    while letter <= b'z' {
        output.write(&[letter]).unwrap();
        letter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_the_alphabet() {
        let mut output: Vec<u8> = Vec::new();

        ft_print_alphabet(&mut output).unwrap();
        assert_eq!(&output, b"abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn writes_the_alphabet_chatgpt_version() {
        let mut output: Vec<u8> = Vec::new();

        ft_print_alphabet_chatgpt(&mut output);
        assert_eq!(&output, b"abcdefghijklmnopqrstuvwxyz");
    }
}
