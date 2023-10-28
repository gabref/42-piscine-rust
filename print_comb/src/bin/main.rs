use std::io;

use print_comb::{ft_print_comb, ft_print_comb_chatgpt};

fn main() {
    ft_print_comb(&mut io::stdout()).unwrap();
    println!();
    ft_print_comb_chatgpt();
}
