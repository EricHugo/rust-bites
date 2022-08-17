use rand::Rng;

fn main() {
    let roll = rand::thread_rng().gen_range(1..=6);
    println!("Roll: {roll}");
}
