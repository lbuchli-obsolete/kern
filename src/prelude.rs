use crate::parser::*;

pub const PRELUDE: Program<Name> = vec![SuperCombinator::<Name> {
    name: "I".to_string(),
    args: vec!["x".to_owned()],
    value: Expr::Variable("x".to_owned()),
}];
