/*
use std::collections::HashMap;

#[derive(Debug)]
enum Reversible {
    RevInt(u32),
    RevFloat(f32),
}
*/


enum RevStmnt {
    PlusEq(RevExpr, RevExpr),
    MinusEq(RevExpr, RevExpr),
    MultEq(RevExpr, RevExpr),
    DivEq(RevExpr, RevExpr),
    IfStmnt(RevExpr, Box<RevStmnt>, Box<RevStmnt>, RevExpr),
    FromStmnt(RevExpr, Box<RevStmnt>, RevExpr),
    Stmnts(Box<[RevStmnt]>),
    Call(String),
    Uncall(String),
    CallBack(Box<Fn(RevType) -> ()>),
}


enum RevExpr {
    Lit(RevType),
    Var(String),
    Plus(Box<RevExpr>, Box<RevExpr>),
    Minus(Box<RevExpr>, Box<RevExpr>),
    Mult(Box<RevExpr>, Box<RevExpr>),
    Div(Box<RevExpr>, Box<RevExpr>),
    Equal(Box<RevExpr>, Box<RevExpr>),
    Geq(Box<RevExpr>, Box<RevExpr>),
}




use std::fmt;

trait RevExt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn add_reverse_rhs(&self, &RevType) -> RevType;
    fn add_reverse_lhs(&self, &RevType) -> RevType;
    fn sub_reverse_rhs(&self, &RevType) -> RevType;
    fn sub_reverse_lhs(&self, &RevType) -> RevType;
    fn mult_reverse_rhs(&self, &RevType) -> RevType;
    fn mult_reverse_lhs(&self, &RevType) -> RevType;
    fn div_reverse_rhs(&self, &RevType) -> RevType;
    fn div_reverse_lhs(&self, &RevType) -> RevType;
    fn geq_rhs(&self, &RevType) -> RevType;
    fn geq_lhs(&self, &RevType) -> RevType;
    fn eq_rhs(&self, &RevType) -> RevType;
    fn eq_lhs(&self, &RevType) -> RevType;
    fn not(&self) -> RevType;
    fn is_empty(&self) -> bool;
}


enum RevType {
    Empty,
    Revi64(i64),
    Revf64(f64),
    Revbool(bool),
    RevItem(Box<RevType>),
    RevList(Box<[RevType]>),
    RevExtension(Box<RevExt>),
}


// Can't derive this automatically since we allow extensions
impl fmt::Debug for RevType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Empty => write!(f, "Empty()"),
            Revi64(x) => write!(f, "Revi64({:?})", x),
            Revf64(x) => write!(f, "Revf64({:?})", x),
            Revbool(x) => write!(f, "Revbool({:?})", x),
            RevItem(x) => write!(f, "RevItem({:?})", x),
            RevList(x) => write!(f, "RevList({:?})", x),
            RevExtension(x) => x.fmt(f),    // Extensions choose how to display themselves
        }
    }
}

use RevType::*;


impl RevType {

    fn rev_add(rhs: &RevType, lhs: &RevType) -> RevType {
        match rhs {
            Revi64(rhs_i) => {
                match lhs {
                    Revi64(lhs_i) => Revi64(rhs_i + lhs_i),
                    RevExtension(lhs_ext) => lhs_ext.add_reverse_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            Revf64(rhs_f) => {
                match lhs {
                    Revf64(lhs_f) => Revf64(rhs_f + lhs_f),
                    RevExtension(lhs_ext) => lhs_ext.add_reverse_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            RevExtension(rhs_ext) => rhs_ext.add_reverse_rhs(lhs),
            _ => unreachable!(),
        }
    }

    fn rev_sub(rhs: &RevType, lhs: &RevType) -> RevType {
        match rhs {
            Revi64(rhs_i) => {
                match lhs {
                    Revi64(lhs_i) => Revi64(rhs_i - lhs_i),
                    RevExtension(lhs_ext) => lhs_ext.sub_reverse_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            Revf64(rhs_f) => {
                match lhs {
                    Revf64(lhs_f) => Revf64(rhs_f - lhs_f),
                    RevExtension(lhs_ext) => lhs_ext.sub_reverse_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            RevExtension(rhs_ext) => rhs_ext.sub_reverse_rhs(lhs),
            _ => unreachable!(),
        }
    }

    fn rev_mult(rhs: &RevType, lhs: &RevType) -> RevType {
        // We can't revesibly multiply by 0
        if RevType::is_empty(lhs) {
            unreachable!();
        }

        match rhs {
            Revi64(rhs_i) => {
                match lhs {
                    Revi64(lhs_i) => Revi64(rhs_i * lhs_i),
                    RevExtension(lhs_ext) => lhs_ext.mult_reverse_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            Revf64(rhs_f) => {
                match lhs {
                    Revf64(lhs_f) => Revf64(rhs_f * lhs_f),
                    RevExtension(lhs_ext) => lhs_ext.mult_reverse_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            RevExtension(rhs_ext) => rhs_ext.mult_reverse_rhs(lhs),
            _ => unreachable!(),
        }
    }

    fn rev_div(rhs: &RevType, lhs: &RevType) -> RevType {
        // We can never divide by 0
        if RevType::is_empty(lhs) {
            unreachable!();
        }

        match rhs {
            Revi64(rhs_i) => {
                match lhs {
                    Revi64(lhs_i) => Revi64(rhs_i / lhs_i),
                    RevExtension(lhs_ext) => lhs_ext.div_reverse_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            Revf64(rhs_f) => {
                match lhs {
                    Revf64(lhs_f) => Revf64(rhs_f / lhs_f),
                    RevExtension(lhs_ext) => lhs_ext.div_reverse_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            RevExtension(rhs_ext) => rhs_ext.sub_reverse_rhs(lhs),
            _ => unreachable!(),
        }
    }

    fn geq(rhs: &RevType, lhs: &RevType) -> RevType {
        match rhs {
            Revi64(rhs_i) => {
                match lhs {
                    Revi64(lhs_i) => Revbool(rhs_i >= lhs_i),
                    RevExtension(lhs_ext) => lhs_ext.geq_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            Revf64(rhs_f) => {
                match lhs {
                    Revf64(lhs_f) => Revbool(rhs_f >= lhs_f),
                    RevExtension(lhs_ext) => lhs_ext.geq_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            RevExtension(rhs_ext) => rhs_ext.geq_rhs(lhs),
            _ => unreachable!(),
        }
    }

    fn eq(rhs: &RevType, lhs: &RevType) -> RevType {
        match rhs {
            Revi64(rhs_i) => {
                match lhs {
                    Revi64(lhs_i) => Revbool(rhs_i == lhs_i),
                    RevExtension(lhs_ext) => lhs_ext.eq_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            Revf64(rhs_f) => {
                match lhs {
                    Revf64(lhs_f) => Revbool(rhs_f == lhs_f),
                    RevExtension(lhs_ext) => lhs_ext.eq_lhs(rhs),
                    _ => unreachable!(),
                }
            },
            RevExtension(rhs_ext) => rhs_ext.eq_rhs(lhs),
            _ => unreachable!(),
        }
    }

    fn not(var: &RevType) -> RevType {
        match var {
            Revbool(b) => Revbool(!b),
            RevExtension(ext) => ext.not(),
            _ => unreachable!(),
        }
    }

    fn destroy(var: &RevType) -> RevType {
        if !RevType::is_empty(var) {
            unreachable!();
        }
        Empty
    }

    fn is_empty(var: &RevType) -> bool {
        match var {
            Revi64(var_i) => *var_i == 0,
            Revf64(var_f) => *var_f == 0.0,
            RevExtension(var_ext) => var_ext.is_empty(),
            _ => unreachable!(),
        }
    }

}

fn plus(rhs: RevExpr, lhs: RevExpr) -> RevExpr {
    Plus(Box::new(rhs), Box::new(lhs))
}

fn int(x: i64) -> RevExpr {
    Lit(RevType::Revi64(x))
}

fn var(x: &str) -> RevExpr {
    Var(x.to_string())
}


use std::collections::HashMap;
use RevExpr::*;
use RevStmnt::*;

fn main() {

    let procedure = 
    
                Stmnts(Box::new([
                    PlusEq(var("x"), int(15)),
                    PlusEq(var("y"), plus(plus(int(15), int(15)), int(20))),
                ]));
    
    let mut store = HashMap::new();
    

    let a = Revi64(5);
    let b = Revi64(10);
    let c = RevType::rev_add(&a,&b);

    let aa = Revf64(5.0);
    let bb = Revf64(10.0);
    let cc = RevType::rev_add(&aa,&bb);

    let rev_int_1 = Revi64(15);
    let rev_int_2 = Revi64(30);

    store.insert("x", rev_int_1);
    store.insert("y", rev_int_2);

    let res: RevType;
    {
        let x_var = store.get("x").unwrap();
        let y_var = store.get("y").unwrap();

        res = RevType::not(&RevType::geq(x_var, y_var));
    }
    //store.insert("x", res);


    println!("Hello, world!, {:?}", res);

}
