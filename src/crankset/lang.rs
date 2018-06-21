
use crankset::types::RevType::*;
use crankset::types::RevType;
//#[derive(Clone)]
pub enum RevStmnt {
    PlusEq(RevExpr, RevExpr),
    MinusEq(RevExpr, RevExpr),
    MultEq(RevExpr, RevExpr),
    DivEq(RevExpr, RevExpr),
    Swap(RevExpr, RevExpr),
    IfStmnt(RevExpr, Box<RevStmnt>, Box<RevStmnt>, RevExpr),
    FromStmnt(RevExpr, Box<RevStmnt>, RevExpr),
    Stmnts(Box<[RevStmnt]>),
    Call(String),
    Uncall(String),
    CallBack(Box<Fn(&RevType) -> ()>, RevExpr),
}


#[derive(Clone)]
pub enum RevExpr {
    Lit(RevType),
    Var(String),
    Plus(Box<RevExpr>, Box<RevExpr>),
    Minus(Box<RevExpr>, Box<RevExpr>),
    Mult(Box<RevExpr>, Box<RevExpr>),
    Div(Box<RevExpr>, Box<RevExpr>),
    Equal(Box<RevExpr>, Box<RevExpr>),
    Geq(Box<RevExpr>, Box<RevExpr>),
}