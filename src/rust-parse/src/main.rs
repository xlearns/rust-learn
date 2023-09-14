
use parser::*;
mod test;

fn main() {
    test();
}

fn test(){
    for (i,v) in (1..=5).enumerate(){
        println!("{} {}",i,v);
    }
    hello();
    test::parse("hello from the test mod");
}
