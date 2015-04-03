extern crate anthologion;
use anthologion::psalter;

fn main() {
    println!("Hello");
    let psalm = psalter::get_psalm(1);
    println!("{}", psalm.to_string());
}
