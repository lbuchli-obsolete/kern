use crate::parser::*;

// A standard prelude of basic functions to be loaded before any program

thread_local! {
    pub static PRELUDE: Program<Name> = vec![
        // I x = x
        SuperCombinator::<Name> {
            name: "I".to_owned(),
            args: vec!["x".to_owned()],
            value: Expr::Variable("x".to_owned()),
        },
        // Fst x y = x
        SuperCombinator::<Name>{
            name: "Fst".to_owned(),
            args: vec!["x".to_owned(), "y".to_owned()],
            value: Expr::Variable("x".to_owned()),
        },
        // Snd x y = y
        SuperCombinator::<Name>{
            name: "Snd".to_owned(),
            args: vec!["x".to_owned(), "y".to_owned()],
            value: Expr::Variable("y".to_owned()),
        },
        // S f g x = f x (g x)
        SuperCombinator::<Name>{
            name: "S".to_owned(),
            args: vec!["f".to_owned(), "g".to_owned(), "x".to_owned()],
            value: Expr::Application(
                Box::new(Expr::Application(
                    Box::new(Expr::Variable("f".to_owned())),
                    Box::new(Expr::Variable("x".to_owned()))
                )),
                Box::new(Expr::Application(
                    Box::new(Expr::Variable("g".to_owned())),
                    Box::new(Expr::Variable("x".to_owned()))
                ))),
        },
        // Compose f g x = f (g x)
        SuperCombinator::<Name>{
            name: "Compose".to_owned(),
            args: vec!["f".to_owned(), "g".to_owned(), "x".to_owned()],
            value: Expr::Application(
                Box::new(Expr::Variable("f".to_owned())),
                Box::new(Expr::Application(
                    Box::new(Expr::Variable("g".to_owned())),
                    Box::new(Expr::Variable("x".to_owned()))
                ))),
        },
        // Twice f = Compose f f
        SuperCombinator::<Name>{
            name: "Twice".to_owned(),
            args: vec!["f".to_owned()],
            value: Expr::Application(
                Box::new(Expr::Application(
                    Box::new(Expr::Variable("Compose".to_owned())),
                    Box::new(Expr::Variable("f".to_owned()))
                )),
                Box::new(Expr::Variable("f".to_owned())),
            ),
        },
];
}
