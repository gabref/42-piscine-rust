use swap::ft_swap;

fn main() {
    let mut a = 10;
    let mut b = 20;
    println!("{} - {}", a, b);
    ft_swap(&mut a, &mut b);
    println!("{} - {}", a, b);
}
