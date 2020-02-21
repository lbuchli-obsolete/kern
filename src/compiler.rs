use crate::parser::*;
use crate::reducer::*;

use std::collections::HashMap;
use std::ops::Deref;

// the compiler produces the initial state of the reducer

pub fn compile<'a>(ast: Program<Name>) -> Result<State<'a>, String> {
    let globals = buildGlobals(ast)?;
    if let Some(main) = globals.get("main") {
        Ok(State {
            stack: vec![HNode::SuperCombinator(main)],
            dump: vec![],
            globals: globals.clone(),
            stats: Stat::new(),
        })
    } else {
        Err("Main supercombinator not found".to_owned())
    }
}

fn buildGlobals<'a>(ast: Program<Name>) -> Result<HashMap<Name, (Vec<Name>, HNode<'a>)>, String> {
    let mut result = HashMap::new();
    for global in ast {
        result.insert(
            global.name,
            (global.args, parseExpr(global.value, result.clone())?),
        );
    }

    Ok(result)
}

fn parseExpr<'a>(
    expr: Expr<Name>,
    globals: HashMap<Name, (Vec<Name>, HNode<'a>)>,
) -> Result<HNode<'a>, String> {
    match expr {
        Expr::Variable(name) => match name.deref() {
            "|" | "&" | "==" | "!=" | ">" | ">=" | "<" | "<=" | "+" | "-" | "*" | "/" => {
                Ok(HNode::PrimitiveFn(name))
            }
            _ => {
                let def = globals.get(&name);
                if let Some(sc) = def {
                    Ok(HNode::<'a>::SuperCombinator(sc))
                } else {
                    Err(format!("Definition for {} not found", name))
                }
            }
        },
        Expr::Number(n) => Ok(HNode::Number(n)),
        Expr::Constructor { .. } => Err("Constructors are not yet supported :(".to_owned()),
        Expr::Application(a, b) => Ok(HNode::Application(
            &parseExpr(*a, globals.clone())?,
            &parseExpr(*b, globals.clone())?,
        )),
        Expr::Let { .. } => Err("Let expressions are not yet supported :(".to_owned()),
        Expr::Case { .. } => Err("Case expressions are not yet supported :(".to_owned()),
        Expr::Lambda { .. } => Err("Lambda expressions are not yet supported :(".to_owned()),
    }
}
