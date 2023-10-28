use ultimate_ft::ft_ultimate_ft;

fn main() {
    let mut nbr: i32 = 0;
    let mut ptr1 = &mut nbr;
    let mut ptr2 = &mut ptr1;
    let mut ptr3 = &mut ptr2;
    let mut ptr4 = &mut ptr3;
    let mut ptr5 = &mut ptr4;
    let mut ptr6 = &mut ptr5;
    let mut ptr7 = &mut ptr6;
    let mut ptr8 = &mut ptr7;
    let ptr9 = &mut ptr8;

    ft_ultimate_ft(ptr9);

    println!("value: {}", nbr);
}
