pub mod parser;
mod prelude;

fn main() {
    println!("{:?}", prelude::PRELUDE);
    println!(
        "{:?}",
        parser::kern::program(
            "
main = double 32;
double x = x+x
"
        )
    );
}
