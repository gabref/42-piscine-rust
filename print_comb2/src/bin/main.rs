use std::io;

use print_comb2::{ft_print_comb2, ft_print_comb2_chatgpt};

fn main() {
    ft_print_comb2(&mut io::stdout()).unwrap();
    println!();
    println!();
    // chat gpt very wrong
    ft_print_comb2_chatgpt();
}
