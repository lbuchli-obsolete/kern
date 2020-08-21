mod compiler;
pub mod parser;
mod prelude;
mod reducer;

fn main() {
    println!(
        "{:?}",
        reducer::reduce(
            &mut compiler::compile(
                parser::kern::program(
                    "
double x = x+x;
main = double 32
"
                )
                .unwrap()
            )
            .unwrap()
        )
    );
}
