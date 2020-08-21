use crate::parser;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct NodeBox<'a>(Rc<RefCell<HNode<'a>>>);

impl<'a> NodeBox<'a> {
    pub unsafe fn new(node: HNode<'a>) -> Self {
        NodeBox(Rc::new(RefCell::new(node)))
    }

    pub fn replace(&self, new: HNode<'a>) -> HNode<'a> {
        self.0.replace(new)
    }

    pub fn borrow(&self) -> Ref<'_, HNode<'a>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, HNode<'a>> {
        self.0.borrow_mut()
    }
}

#[derive(Debug, Clone)]
pub enum HNode<'a> {
    Application(NodeBox<'a>, NodeBox<'a>),
    SuperCombinator(&'a str),
    PrimitiveFn(&'a str),
    Number(i64),
}

#[derive(Debug)]
pub struct State<'a> {
    pub stack: Vec<NodeBox<'a>>,     // the current graph
    pub dump: Vec<Vec<NodeBox<'a>>>, // stack of old graphs
    pub globals: HashMap<parser::Name, (Vec<parser::Name>, NodeBox<'a>)>, // a list of all supercombinators
    pub stats: Stat,                                                      // performance statistics
}

impl<'a> State<'a> {
    /// returns true if the current state is final (there are no more redexes)
    fn is_final(&self) -> bool {
        self.dump.len() == 0 && !self.is_steppable()
    }

    /// checks if there is still something to do
    fn is_steppable(&self) -> bool {
        if self.stack.len() == 0 {
            return self.dump.len() > 0;
        }

        let mut spine_stack = vec![self.stack.first().unwrap().borrow()];
        while spine_stack.len() > 0 {
            let last = spine_stack.last().unwrap();
            if let HNode::Application(ref left, _) = *last {
                spine_stack.push(left.get());
            }
        }

        spine_stack.len() > 0 // TODO check also for WHNF
    }

    fn step(&mut self) -> Result<(), String> {
        // find the outermost expression
        let mut spine_stack = vec![self.stack.first().unwrap().get()];
        while spine_stack.len() > 0 {
            if let HNode::Application(left, _) = *spine_stack.last().unwrap() {
                spine_stack.push(left.get());
            }
        }

        // check the outermost type
        let to_eval = spine_stack.last().unwrap();
        if let HNode::SuperCombinator(name) = *to_eval {
            let sc = self
                .globals
                .get(name)
                .ok_or_else(|| format!("Could not find supercombinator {}", name))?;
            self.stack.pop(); // remove supercombinator node
            self.stack.push(sc.1.clone()); // put supercombinator body on stack
                                           // TODO tell the node underneath to update its dependency
            Ok(())
        } else if let HNode::PrimitiveFn(name) = *to_eval {
            // check if arguments are evaluated; else evaluate them
            if let Some(first_app) = spine_stack.get(spine_stack.len() - 2) {
                if let HNode::Application(_, right_a) = *first_app {
                    if let HNode::Number(a) = right_a.get() {
                        // yay! The first parameter is a number. Let's check the second one
                        if let Some(second_app) = spine_stack.get(spine_stack.len() - 3) {
                            if let HNode::Application(_, right_b) = *second_app {
                                if let HNode::Number(b) = right_b.get() {
                                    // yay! we can calculate now and then save the result
                                    let result = calculate_primitve(name.to_owned(), a, b);
                                    self.stack.pop();
                                    self.stack.pop();
                                    self.stack.push(NodeBox::new(HNode::Number(result)));
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
#[derive(Debug)]
pub struct Stat {
    steps: i64,
}

impl Stat {
    pub fn new() -> Stat {
        Stat { steps: 0 }
    }
}

/// reduces expressions (and thus evaluates them)
pub fn reduce<'a>(state: &'a mut State<'a>) -> Result<&'a NodeBox<'a>, String> {
    while !state.is_final() {
        state.step()?;
        state.stats.steps += 1;
        println!("{:?}", state);
    }
    state.stack.last().ok_or("Stack empty".to_owned())
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
