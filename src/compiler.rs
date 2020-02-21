use crate::parser::*;
use crate::reducer::*;

use std::collections::HashMap;
use std::ops::Deref;

// the compiler produces the initial state of the reducer

pub fn compile(ast: Program<Name>) -> Result<State, String> {
    let globals = buildGlobals(ast)?;
    if let Some(main) = globals.get("main") {
        Ok(State {
            stack: vec![Box::new(HNode::SuperCombinator(Box::new(main.clone())))],
            dump: vec![],
            globals: globals.clone(),
            stats: Stat::new(),
        })
    } else {
        Err("Main supercombinator not found".to_owned())
    }
}

fn buildGlobals(ast: Program<Name>) -> Result<HashMap<Name, (Vec<Name>, HNode)>, String> {
    let mut result = HashMap::new();
    for global in ast {
        result.insert(
            global.name,
            (global.args, parseExpr(global.value, result.clone())?),
        );
    }

    Ok(result)
}

fn parseExpr(
    expr: Expr<Name>,
    globals: HashMap<Name, (Vec<Name>, HNode)>,
) -> Result<HNode, String> {
    match expr {
        Expr::Variable(name) => match name.deref() {
            "|" | "&" | "==" | "!=" | ">" | ">=" | "<" | "<=" | "+" | "-" | "*" | "/" => {
                Ok(HNode::PrimitiveFn(name))
            }
            _ => {
                let def = globals.get(&name);
                if let Some(sc) = def {
                    Ok(HNode::SuperCombinator(Box::new(sc.clone())))
                } else {
                    Err(format!("Definition for {} not found", name))
                }
            }
        },
        Expr::Number(n) => Ok(HNode::Number(n)),
        Expr::Constructor { .. } => Err("Constructors are not yet supported :(".to_owned()),
        Expr::Application(a, b) => Ok(HNode::Application(
            Box::new(parseExpr(*a, globals.clone())?),
            Box::new(parseExpr(*b, globals.clone())?),
        )),
        Expr::Let { .. } => Err("Let expressions are not yet supported :(".to_owned()),
        Expr::Case { .. } => Err("Case expressions are not yet supported :(".to_owned()),
        Expr::Lambda { .. } => Err("Lambda expressions are not yet supported :(".to_owned()),
    }
}
