use std::io::{self};
use putchar::{putchar, putchar_2};

fn main() {
    let str = vec!['h', 'e', 'l', 'l', 'o', '\n'];
    for c in str.iter() {
        putchar(*c);
    }

    for c in str.iter() {
        putchar_2(*c, &mut io::stdout()).unwrap();
    }
}
