mod compiler;
pub mod parser;
mod prelude;
mod reducer;

fn main() {
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
