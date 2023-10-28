fn ft_ft(nbr: &mut i32) {
    *nbr = 42;
}

fn main() {
    let mut num = 0;
    ft_ft(&mut num);
    println!("value: {}", num);
}
