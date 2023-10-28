use ultimate_div_mod::ft_ultimate_div_mod;

fn main() {
    let mut a = 10;
    let mut b = 4;
    print!("{} and {} - ", a, b);
    ft_ultimate_div_mod(&mut a, &mut b);
    println!("div: {}. mod: {}", a, b);
}
