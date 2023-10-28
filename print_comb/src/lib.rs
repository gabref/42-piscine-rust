use std::io::{Error, Write, self};

pub fn ft_print_comb(output: &mut impl Write) -> Result<(), Error> {
    let mut i: u8;
    let mut j: u8;
    let mut k: u8;

    i = b'0';
    while i <= b'7' {
        j = i + 1;
        while j <= b'8' {
            k = j + 1;
            while k <= b'9' {
                output.write(&[i]).unwrap();
                output.write(&[j]).unwrap();
                output.write(&[k]).unwrap();
                if i != b'7' || j != b'8' || k != b'9' {
                    output.write(&[b',']).unwrap();
                    output.write(&[b' ']).unwrap();
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    Ok(())
}

// kinda wrong thought
pub fn ft_print_comb_chatgpt_conv(output: &mut impl Write) {
    let mut i = '0';

    while i <= '7' {
        let mut j = (i as u8 + 1) as char;
        while j <= '8' {
            let mut k = (j as u8 + 1) as char;
            while k <= '9' {
                let chars_to_write: [u8; 4] = [i as u8, j as u8, k as u8, b','];
                output.write(&chars_to_write).unwrap();
                k = (k as u8 + 1) as char;
            }
            j = (j as u8 + 1) as char;
        }
        i = (i as u8 + 1) as char;
    }
}

pub fn ft_print_comb_chatgpt() {
    for i in 0..10 {
        for j in (i + 1)..10 {
            for k in (j + 1)..10 {
                // check if digits are different
                if i != j && i != k && j != k {
                    print!("{:0}{:0}{:0}", i, j, k);
                    io::stdout().flush().expect("Error flushing stdout");
                    if i < 7 {
                        print!(", ");
                        io::stdout().flush().expect("Error flushing stdout");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb() {
        let mut output: Vec<u8> = Vec::new();
        ft_print_comb(&mut output).unwrap();
        assert!(output.starts_with(b"012, 013, 014, 015, 0"));
        assert!(output.ends_with(b", 579, 589, 678, 679, 689, 789"));
    }

    #[test]
    fn test_comb_chatgpt() {
        let mut output: Vec<u8> = Vec::new();
        ft_print_comb_chatgpt_conv(&mut output);
        assert!(output.starts_with(b"012,013,014,015,0"));
        assert!(output.ends_with(b",579,589,678,679,689,789,"));
    }
}
