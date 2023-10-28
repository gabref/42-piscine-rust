use std::io::Write;

fn print_combn(comb: &mut Vec<u8>, n: u8, index: u8, output: &mut impl Write) {
    if index == 0 {
        comb.into_iter().for_each(|c| {
            output.write(&[*c]).unwrap();
        });
        if comb[0] != 58 - comb.len() as u8 {
            output.write(b", ").unwrap();
        }
        return ;
    }
    let mut i = n;
    while i <= (58 - index) {
        comb.push(i);
        print_combn(comb, i + 1, index - 1, output);
        comb.pop();
        i += 1;
    }
}

pub fn ft_print_combn(n: u8, output: &mut impl Write) {
    if n > 0 && n < 10 {
        let mut comb = Vec::new();
        print_combn(&mut comb, b'0', n, output);
    }
}
