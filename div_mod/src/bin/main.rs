use div_mod::ft_div_mod;

fn main() {
    let a = 10;
    let b = 5;
    let mut div = 0;
    let mut md = 0;
    ft_div_mod(a, b, &mut div, &mut md);
    println!("{} / {} = {}. {} % {} = {}", a, b, div, a, b, md);
}
