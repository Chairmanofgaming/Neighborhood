use rand::{rng, Rng};
fn main() {
    let x: u8 = rand::thread_rng().gen_range(1..143);
    println!("Hello Neighbor, read the poem on page {x}");
}
