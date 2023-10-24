use std::io::{Error, Write};

pub fn putchar(c: char) {
    print!("{}", c);
}

pub fn putchar_2(c: char, output: &mut impl Write) -> Result<(), Error> {
    output.write(&[c as u8]).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_char_to_output() {
        let mut output: Vec<u8> = Vec::new();

        putchar_2('a', &mut output).unwrap();

        assert_eq!(&output, b"a");
    }
}
