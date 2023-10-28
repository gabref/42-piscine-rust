use std::io::{Write,Error, self};

pub fn ft_print_comb2(output: &mut impl Write) -> Result<(), Error> {
    let mut i: u8 = 0;

    while i <= 98 {
        let mut j = i + 1;
        while j <= 99 {
            let chars_to_write: [u8; 5] = [i / 10 + b'0', i % 10 + 48, b' ', j / 10 + 48, j % 10 + 48];
            output.write(&chars_to_write).unwrap();
            if i != 98 {
                output.write(b", ").unwrap();
            }
            j += 1;
        }
        i += 1;
    }
    Ok(())
}

// so wrong chat gpt, my solution is right
pub fn ft_print_comb2_chatgpt() {
    for i in 0..100 {
        for j in i + 1..100 {
            let tens_i = i / 10;
            let ones_i = i % 10;
            let tens_j = j / 10;
            let ones_j = j % 10;

            print!("{:02} {:02}", tens_i, ones_i);
            print!(", {:02} {:02}", tens_j, ones_j);

            io::stdout().flush().expect("Error flushing stdout");

            if i < 98 {
                print!(", ");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_comb2() {
        let mut output = Vec::new();
        ft_print_comb2(&mut output).unwrap();
        assert!(output.starts_with(b"00 01, 00 02, 00 03, 00 04, 00 05, 00 06, 00 07,"));
        assert!(output.ends_with(b"96, 95 97, 95 98, 95 99, 96 97, 96 98, 96 99, 97 98, 97 99, 98 99"));
    }
}
