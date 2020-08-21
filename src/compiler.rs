use crate::parser::*;
use crate::reducer::*;

use std::collections::HashMap;
use std::ops::Deref;

// produces the initial state of the reducer
pub fn compile<'a>(ast: Program<Name>) -> Result<State<'a>, String> {
    let globals = build_globals(ast)?;
    if globals.get("main").is_some() {
        Ok(State {
            stack: vec![NodeBox::new(HNode::SuperCombinator("main"))],
            dump: vec![],
            globals: globals.clone(),
            stats: Stat::new(),
        })
    } else {
        Err("Main supercombinator not found".to_owned())
    }
}

fn build_globals<'a>(
    ast: Program<Name>,
) -> Result<HashMap<Name, (Vec<Name>, NodeBox<'a>)>, String> {
    let mut result: HashMap<Name, (Vec<Name>, NodeBox)> = HashMap::new();
    for global in ast {
        let mut locals = HashMap::new();
        for arg in global.args.clone() {
            locals.insert(arg, NodeBox::new(HNode::Number(42))); // TODO what to put here? I'm afraid 42 isn't the answer...
        }
        result.insert(
            global.name,
            (global.args, parse_expr(global.value, locals)?),
        );
    }

    Ok(result)
}

fn parse_expr(expr: Expr<Name>, locals: HashMap<Name, NodeBox>) -> Result<NodeBox, String> {
    match expr {
        Expr::Variable(name) => match name.deref() {
            "|" | "&" | "==" | "!=" | ">" | ">=" | "<" | "<=" | "+" | "-" | "*" | "/" => {
                Ok(NodeBox::new(HNode::PrimitiveFn(&name)))
            }
            _ => {
                if let Some(var) = locals.get(&name) {
                    Ok(var.clone()) // TODO don't clone the var, point to it
                } else {
                    Ok(NodeBox::new(HNode::SuperCombinator(&name)))
                }
            }
        },
        Expr::Number(n) => Ok(NodeBox::new(HNode::Number(n))),
        Expr::Constructor { .. } => Err("Constructors are not yet supported :(".to_owned()),
        Expr::Application(a, b) => Ok(NodeBox::new(HNode::Application(
            parse_expr(*a, locals.clone())?,
            parse_expr(*b, locals.clone())?,
        ))),
        Expr::Let { .. } => Err("Let expressions are not yet supported :(".to_owned()),
        Expr::Case { .. } => Err("Case expressions are not yet supported :(".to_owned()),
        Expr::Lambda { .. } => Err("Lambda expressions are not yet supported :(".to_owned()),
    }
}
