fn main() {
    ft_is_negative(10);
    ft_is_negative(-42);
    ft_is_negative(0);
}

pub fn ft_is_negative(n: i32) {
    if n < 0 {
        println!("N");
    } else {
        println!("P");
    }
}
