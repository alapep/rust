const STARTING_MISSILES : u32 = 8;
const READY_MISSILES : u32 = 2;
fn main() {
 
    let (missiles, ready)  = (STARTING_MISSILES, READY_MISSILES);
     println!("Firing {} of missiles {}", ready, missiles);
     println!("Remaining {} missiles", STARTING_MISSILES - ready);
}
