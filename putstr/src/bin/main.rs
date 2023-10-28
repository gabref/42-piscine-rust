use std::{io, env::args};

use putstr::{ft_putstr,ft_putstr2};

fn main() {
    let output = &mut io::stdout();
    let my_str = String::from("ciao bello!");
    ft_putstr(my_str, output);

    println!();

    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        ft_putstr(args[1].clone(), output);
    }

    println!();

    if args.len() > 1 {
        ft_putstr2(&args[1], output);
    }
}
