fn main() {
    println!("Hello, world!");
    let mut i = 10;
    println!("{}",i);
    i = 2;
    println!("{}",i);
    let y = 10;
    let z = 20;
    let vol = calc_volume(i, y, z);
    println!("Volume {}", vol);
}

fn calc_volume(x:i32, y:i32, z:i32) -> i32 {
    x*y*z
}