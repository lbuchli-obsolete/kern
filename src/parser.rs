///////////////////////////////////////////////////////////////////////////////
//                         Syntax Tree Representation                        //
///////////////////////////////////////////////////////////////////////////////

pub type Name = String;

#[derive(Debug)]
pub enum Expr<T> {
    Variable(Name),
    Number(i64),
    Constructor {
        tag: i64,
        arity: i64,
    },
    Application(Box<Expr<T>>, Box<Expr<T>>),
    Let {
        is_recursive: bool,
        definitions: Vec<(T, Expr<T>)>,
        body: Box<Expr<T>>,
    },
    Case {
        expr: Box<Expr<T>>,
        alts: Vec<(i64, Vec<T>, Expr<T>)>,
    },
    Lambda {
        args: Vec<T>,
        body: Box<Expr<T>>,
    },
}

impl<T> Expr<T> {
    fn is_atomic_expression(&self) -> bool {
        match self {
            Expr::Variable(_) => true,
            Expr::Number(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct SuperCombinator<T> {
    name: Name,
    args: Vec<T>,
    value: Expr<T>,
}

pub type Program<T> = Vec<SuperCombinator<T>>;

///////////////////////////////////////////////////////////////////////////////
//                                  Parsing                                  //
///////////////////////////////////////////////////////////////////////////////

extern crate peg;
peg::parser! {
    pub grammar kern() for str {
        pub rule program() -> Program<Name>
            = _? combs:(supercomb() ++ (_? ";" _?)) _? { combs }

        rule supercomb() -> SuperCombinator<Name>
            = name:name() _ args:(name() ** _) _? "=" _? expr:expr()
        {
            SuperCombinator::<Name>{
                name: name,
                args: args,
                value: expr,
            }
        }

        rule expr() -> Expr<Name>
            = precedence!{
                x:@ _? "|" _? y:(@) { binaryop_expr("|", x, y) }
                --
                x:@ _? "&" _? y:(@) { binaryop_expr("&", x, y) }
                --
                x:(@) _? "==" _? y:@  { binaryop_expr("==", x, y) } // TODO infix
                x:(@) _? "!=" _? y:@  { binaryop_expr("!=", x, y) } // infix
                x:(@) _? ">"  _? y:@  { binaryop_expr(">", x, y) }  // infix
                x:(@) _? ">=" _? y:@  { binaryop_expr(">=", x, y) } // infix
                x:(@) _? "<"  _? y:@  { binaryop_expr("<", x, y) }  // infix
                x:(@) _? "<=" _? y:@  { binaryop_expr("<=", x, y) } // infix
                --
                x:@ _? "+" _? y:(@) { binaryop_expr("+", x, y) }
                x:(@) _? "-" _? y:@ { binaryop_expr("-", x, y) } // infix
                --
                x:@ _? "*" _? y:(@) { binaryop_expr("*", x, y) }
                x:(@) _? "/" _? y:@ { binaryop_expr("/", x, y) } // infix
                --
                x:(@) _ y:@   { Expr::Application(Box::new(x), Box::new(y)) }
                --
                "(" _? e:expr() _? ")" { e }
                d:data() { d }
            }


        rule data() -> Expr<Name>
            = constructor() /
            r#let() /
            case() /
            lambda() /
            number() /
            variable()

        // Expression that can be used as a function argument
        rule aexpr() -> Expr<Name>
            =  constructor() / "(" e:expr() ")" {e} / number() / variable()

        rule constructor() -> Expr<Name>
            = "Pack{" tag:$(['0'..='9']+) "," _? arity:$(['0'..='9']+) "}"
        {?
            if let Ok(p_tag) = tag.parse() {
                if let Ok(p_arity) = arity.parse() {
                    Ok(Expr::Constructor{
                        tag: p_tag,
                        arity: p_arity,
                    })
                } else {
                    Err("Could not parse arity of constructor")
                }
            } else {
                Err("Could not parse tag of constructor")
            }
        }

        rule r#let() -> Expr<Name>
            = "let" _ defs:definitions() _ "in" _ epxr:expr()
        {
            Expr::Let{
                is_recursive: false, // TODO
                definitions: defs,
                body: Box::new(epxr),
            }
        }

        rule case() -> Expr<Name>
            = "case" _ expr:expr() _ "of" _ alts:alts()
        {
            Expr::Case{
                expr: Box::new(expr),
                alts: alts,
            }
        }

        rule lambda() -> Expr<Name>
            = "\\" _? vars:(name() ** ".") ":" _? expr:expr()
        {
            Expr::Lambda{
                args: vars,
                body: Box::new(expr),
            }
        }

        rule number() -> Expr<Name>
            = n:$(['0'..='9']+)
        {?
            if let Ok(pn) = n.parse() {
                Ok(Expr::Number(pn))
            } else {
                Err("NaN")
            }
        }

        rule variable() -> Expr<Name>
            = v:name() { Expr::Variable(v) }


        rule alt() -> (i64, Vec<Name>, Expr<Name>)
            ="<" tag:$(['0'..='9']+) ">" _ vars:(name() ** _) _? "=>" _ expr:expr()
        {?
            if let Ok(p_tag) = tag.parse() {
                Ok((p_tag, vars, expr))
            } else {
                Err("Could not parse alt tag")
            }
        }

        rule alts() -> Vec<(i64, Vec<Name>, Expr<Name>)>
            = alt() ++ (";" _?)

        rule definition() -> (Name, Expr<Name>)
            = name:name() _? "=" _? expr:expr() {(name, expr)}

        rule definitions() -> Vec<(Name, Expr<Name>)>
            = definition() ++ (";" _?)

        rule _() = quiet!{[' ' | '\n' | '\t']+} / expected!("whitespace")
        rule name() -> Name =
            quiet!{s:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { s.to_owned() }} /
            expected!("name")
    }
}

fn binaryop_expr(operator: &str, x: Expr<Name>, y: Expr<Name>) -> Expr<Name> {
    Expr::Application(
        Box::new(Expr::Application(
            Box::new(Expr::Variable(operator.to_owned())),
            Box::new(x),
        )),
        Box::new(y),
    )
}
