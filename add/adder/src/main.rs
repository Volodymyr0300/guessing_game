use add_one;
use rand;

fn main() {
    let num = add_one::random_number();
    println!("{num} plus one is {}", add_one::add_one(num));
}
