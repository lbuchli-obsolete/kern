use crate::parser;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub enum HNode {
    Application(Box<HNode>, Box<HNode>),
    SuperCombinator(Box<(Vec<parser::Name>, HNode)>),
    PrimitiveFn(parser::Name),
    Number(i64),
}

pub struct State {
    pub stack: Vec<Box<HNode>>,     // the current graph
    pub dump: Vec<Box<Vec<HNode>>>, // stack of old graphs
    pub globals: HashMap<parser::Name, (Vec<parser::Name>, HNode)>, // a list of all supercombinators
    pub stats: Stat,                                                // performance statistics
}

impl State {
    /// Create a blank state
    pub fn new() -> State {
        State {
            stack: vec![],
            dump: vec![],
            globals: HashMap::new(),
            stats: Stat::new(),
        }
    }

    /// Unwind the node (given it's an Application) onto the current stack
    fn unwind(&mut self, node: HNode) -> Result<(), &str> {
        if let HNode::Application(a, _) = node {
            self.stack.push(a.clone());
            Ok(())
        } else {
            Err("Not an Application node")
        }
    }

    /// returns true if the current state is final (there are no more redexes)
    fn is_final(&self) -> bool {
        self.stack.len() == 0 && self.dump.len() == 0
    }

    fn step(&mut self) -> Result<(), String> {
        // find the outermost expression
        let mut spine_stack: Vec<&HNode> = vec![self.stack.first().unwrap()];
        while let HNode::Application(left, _) = spine_stack.last().unwrap_or(&&HNode::Number(0)) {
            spine_stack.push(left);
        }

        // check the outermost type
        let to_eval = spine_stack.last().unwrap_or(&&HNode::Number(-1));
        if let HNode::SuperCombinator(body) = to_eval {
            Ok(())
        } else if let HNode::PrimitiveFn(name) = to_eval {
            // check if arguments are evaluated; else evaluate them
            if let Some(first_app) = spine_stack.get(spine_stack.len() - 2) {
                if let HNode::Application(_, right_a) = first_app {
                    if let HNode::Number(a) = **right_a {
                        // yay! The first parameter is a number. Let's check the second one
                        if let Some(second_app) = spine_stack.get(spine_stack.len() - 3) {
                            if let HNode::Application(_, right_b) = second_app {
                                if let HNode::Number(b) = **right_b {
                                    // yay! we can calculate now and then save the result
                                    let result = calculate_primitve(name.clone(), a, b);
                                    // TODO save result
                                    Ok(())
                                } else {
                                    // We have to evaluate
                                    // TODO
                                    Ok(())
                                }
                            } else {
                                Err("Primitive function grandparent is not an application"
                                    .to_owned())
                            }
                        } else {
                            // Weak head normal form
                            Ok(())
                        }
                    } else {
                        // We'll have to evaluate the argument so we put aside the current stack and build
                        // a new one with `right` as the root
                        // TODO
                        Ok(())
                    }
                } else {
                    Err("Primitive function parent is not an application".to_owned())
                }
            } else {
                // We're in weak head normal form
                Ok(())
            }
        } else {
            Err(format!("Illegal state: tried to evaluate {:?}", to_eval))
        }
    }
}

/// statistics about the evaluation
pub struct Stat {
    steps: i64,
}

impl Stat {
    pub fn new() -> Stat {
        Stat { steps: 0 }
    }
}

/// reduces expressions (and thus evaluates them)
pub fn reduce(root: HNode) -> HNode {
    let mut state = State::new();
    HNode::Number(42) // answer
}

/// calculates a primitive function
fn calculate_primitve(operator: parser::Name, a: i64, b: i64) -> i64 {
    match operator.deref() {
        "|" => a | b,
        "&" => a & b,
        "==" => (a == b) as i64,
        "!=" => (a != b) as i64,
        ">" => (a > b) as i64,
        ">=" => (a >= b) as i64,
        "<" => (a < b) as i64,
        "<=" => (a <= b) as i64,
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => 0,
    }
}
