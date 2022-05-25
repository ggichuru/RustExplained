fn increase_mut(mut val: u32, how_much: u32) {
    val += how_much;

    println!("Increased to {}", val);
}

fn main() {
    let score = 2000;
    increase_mut(score, 30);
}
