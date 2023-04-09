// Dummy program to demonstrate the memory location of a variable

fn main() {
    let x: u32 = 4_u32;
    println!("Original x-value: {}", x);
    println!("Memory location: &x: {}", &x as *const _ as usize);

    while x == 4_u32 {}

    println!("New x-value: {}", x);
}