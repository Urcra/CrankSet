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
macro_rules! PLUSEQ {
    ($a:ident, $b:expr ) => {
        {
            PlusEq(var!($a), $b)
        }
    };
}

#[macro_export]
macro_rules! MINUSEQ {
    ($a:ident, $b:expr ) => {
        {
            MinusEq(var!($a), $b)
        }
    };
}


#[macro_export]
macro_rules! CALLBACK {
    ($func:expr, $a:expr ) => {
        {
            CallBack(Box::new($func), $a)
        }
    };
}

#[macro_export]
macro_rules! CALL {
    ($func:ident ) => {
        {
            Call(stringify!($func).to_owned())
        }
    };
}

#[macro_export]
macro_rules! UNCALL {
    ($func:ident ) => {
        {
            Uncall(stringify!($func).to_owned())
        }
    };
}


#[macro_export]
macro_rules! SWAP {
    ($x1:ident , $x2:ident ) => {
        {
            Swap(var!($x1), var!($x2))
        }
    };
}

#[macro_export]
macro_rules! PROCEDURE {
    ($procname:ident : $( $ss:expr );* ) => {
        
            let $procname = 
            (
            stringify!($procname).to_string(),
            Stmnts(Box::new(
            [
            $(
                $ss,
            )*
            ]))
            );
        
    };
}

#[macro_export]
macro_rules! load_proc2 {
    ($rpu:expr ; $procname:ident) => {
        {
            $rpu.load_proc(stringify!($procname).to_string(), $procname)
        }
    };
}

#[macro_export]
macro_rules! load_proc {
    ($rpu:expr ; $procname:ident) => {
        {
            $rpu.load_proc($procname)
        }
    };
}

#[macro_export]
macro_rules! with_vars {
    ($rpu:expr ; $( $var:ident ),*) => {
        {
            $rpu.with_vars(
            &[
            $(
                stringify!($var).to_string(),
            )*
            ]
            )
        }
    };
}




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

#[macro_export]
macro_rules! FROM {
    (($precond:expr) DO{ $( $doitem:expr );*}  UNTIL($postcond:expr)) => {
        {
            FromStmnt(
            $precond,
            Box::new(Stmnts(Box::new(
            [
            $(
                $doitem,
            )*
            ]))),
            $postcond
            )
        }
    };
}




/*
    let freefall = 
        FromStmnt(equal(var("ts"), int(0)), 
        Box::new(Stmnts(Box::new([
            CallBack(Box::new(callback_test), var("ts")),
            PlusEq(var("ts"), int(1)),
            PlusEq(var("v"), int(10)),
            MinusEq(var("h"), minus(var("v"), int(5)))]))),
        equal(var("ts"), var("te")));
*/

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