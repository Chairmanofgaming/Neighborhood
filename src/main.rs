use rand;
fn main() {
    let x: u8 = rand::random_range(1..143);
    println!("Hello Neighbor, read the poem on page {x}");
}
