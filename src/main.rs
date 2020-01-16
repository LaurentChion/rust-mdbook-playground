use rand::Rng;

fn main() {
    let x = rand::thread_rng().gen_range(1,101);
    println!("Random value using external crate: {}", x);
}