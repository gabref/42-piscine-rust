use std::io;

use print_combn::ft_print_combn;

fn main() {
    ft_print_combn(3, &mut io::stdout());
    println!();
    ft_print_combn(2, &mut io::stdout());
    println!();
    ft_print_combn(1, &mut io::stdout());
    println!();
}
