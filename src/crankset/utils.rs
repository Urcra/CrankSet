use crankset::lang::RevExpr::*;
use crankset::lang::RevExpr;
use crankset::types::RevType::*;
use crankset::types::RevType;

pub fn plus(rhs: RevExpr, lhs: RevExpr) -> RevExpr {
    Plus(Box::new(rhs), Box::new(lhs))
}

pub fn minus(rhs: RevExpr, lhs: RevExpr) -> RevExpr {
    Minus(Box::new(rhs), Box::new(lhs))
}

pub fn equal(rhs: RevExpr, lhs: RevExpr) -> RevExpr {
    Equal(Box::new(rhs), Box::new(lhs))
}

pub fn int(x: i64) -> RevExpr {
    Lit(Revi64(x))
}

pub fn var(x: &str) -> RevExpr {
    Var(x.to_string())
}
/*
fn stmnts(stmnts: &[RevStmnt]) -> RevStmnt {
    Stmnts(Box::new(*stmnts))
}*/


#[macro_export]
macro_rules! stmnts {
    ($( $x:expr );* ) => {
        {
            Box::new(
            [
            $(
                $x,
            )*
            ])
        }
    };
}

#[macro_export]
macro_rules! var {
    ($a:ident) => {
        {
            Var(stringify!($a).to_string())
        }
    };
}


#[macro_export]
macro_rules! IF {
    (($precond:expr) THEN{ $( $ifitem:expr );*}  ELSE{ $( $elseitem:expr );*} FI($postcond:expr)) => {
        {
            IfStmnt(
            $precond,
            Box::new(Stmnts(Box::new(
            [
            $(
                $ifitem,
            )*
            ]))),
            Box::new(Stmnts(Box::new(
            [
            $(
                $elseitem,
            )*
            ]))),
            $postcond
            )
        }
    };
}


pub fn extension_downcast<T>(r: &RevType) -> &T where T: 'static{
    let t = match r {
            RevExtension(x) => x,
            _ => unimplemented!()
    };

    match t.as_any().downcast_ref::<T>() {
            Some(b) => b,
            None    => panic!("Invalid downcast")
    }
}

pub fn safe_extension_downcast<T>(r: &RevType) -> Option<&T> where T: 'static{
    let t = match r {
            RevExtension(x) => x,
            _ => return None,
    };

    t.as_any().downcast_ref::<T>()
}