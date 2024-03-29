/*
use std::collections::HashMap;

#[derive(Debug)]
enum Reversible {
    RevInt(u32),
    RevFloat(f32),
}
*/

/*
#[derive(Clone)]
enum RevStmntTest {
    CallBack(Box<Fn(&RevType) -> ()>),
}
*/


//#[derive(Clone)]
enum RevStmnt {
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



use std::any::Any;
use std::fmt;

trait RevExt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn clone(&self) -> Self where Self: Sized;
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
    fn is_true(&self) -> bool;
    fn as_any(&self) -> &Any;
    fn testi(&self, Box<RevExt>) -> RevType;
}


struct Testi {
    tag: u32,
}

fn extension_downcast<T>(r: &RevType) -> &T where T: 'static{
    let t = match r {
            RevExtension(x) => x,
            _ => unimplemented!()
    };

    match t.as_any().downcast_ref::<T>() {
            Some(b) => b,
            None    => panic!("Invalid downcast")
    }
}

fn safe_extension_downcast<T>(r: &RevType) -> Option<&T> where T: 'static{
    let t = match r {
            RevExtension(x) => x,
            _ => return None,
    };

    t.as_any().downcast_ref::<T>()
}

impl RevExt for Testi {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("{:?}", self.tag);
        unimplemented!()
    }
    fn clone(&self) -> Self where Self: Sized {
        unimplemented!()
    }
    fn add_reverse_rhs(&self, r: &RevType) -> RevType {
        /*
        let t = match r {
            RevExtension(x) => x,
            _ => unimplemented!()
        };
        
        let b: &Testi = match t.as_any().downcast_ref::<Testi>() {
            Some(b) => b,
            None    => panic!("&a isn't a B!")
        };
        */
        let b: &Testi = extension_downcast::<Testi>(r);

        RevExtension(Box::new(Testi { tag: b.tag + self.tag}))
    }
    fn add_reverse_lhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn sub_reverse_rhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn sub_reverse_lhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn mult_reverse_rhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn mult_reverse_lhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn div_reverse_rhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn div_reverse_lhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn geq_rhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn geq_lhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn eq_rhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn eq_lhs(&self, r: &RevType) -> RevType {
        unimplemented!()
    }
    fn not(&self) -> RevType {
        unimplemented!()
    }
    fn is_empty(&self) -> bool {
        unimplemented!()
    }
    fn is_true(&self) -> bool {
        unimplemented!()
    }
    fn as_any(&self) -> &Any {
        self
    }
    fn testi(&self, other: Box<RevExt>) -> RevType{
        unimplemented!()
    }

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

impl Clone for RevType {
    fn clone(&self) -> RevType {
        match self {
            Empty => Empty,
            Revi64(x) => Revi64(x.clone()),
            Revf64(x) => Revf64(x.clone()),
            Revbool(x) => Revbool(x.clone()),
            RevItem(x) => RevItem(x.clone()),
            RevList(x) => RevList(x.clone()),
            RevExtension(x) => unimplemented!(),
        }
    }
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

    fn is_true(&self) -> bool {
        match self {
            Revbool(b) => b.clone(),
            RevExtension(ext) => ext.is_true(),
            _ => unreachable!(),
        }
    }

}



use std::collections::HashMap;


struct ProcHandler {
    procs: HashMap<String, RevStmnt>,
}

impl ProcHandler {

    fn new() -> ProcHandler {
        ProcHandler {
            procs: HashMap::new(),
        }
    }

    fn get_proc(&self, p: &String) -> &RevStmnt {
        self.procs.get(p).unwrap()
    }

}

struct StoreHandler {
    store: HashMap<String, RevType>,
}

impl StoreHandler {

    fn new() -> StoreHandler {
        StoreHandler {
            store: HashMap::new(),
        }
    }

    fn run_proc(&mut self, stmnt: &RevStmnt, prochandler: &ProcHandler) {
        self.run_statement(stmnt, prochandler)
    }

    fn run_rev_proc(&mut self, stmnt: &RevStmnt, prochandler: &ProcHandler) {
        self.run_rev_statement(stmnt, prochandler)
    }

    fn run_rev_statement(&mut self, stmnt: &RevStmnt, prochandler: &ProcHandler) {
        match stmnt {
            PlusEq(rhs, lhs) => self.minus_eq(rhs, lhs),
            MinusEq(rhs, lhs) => self.plus_eq(rhs, lhs),
            Swap(rhs, lhs) => self.swap(rhs, lhs),
            Call(s) => self.uncall_stmnt(s, prochandler),
            Stmnts(ss) => self.rev_many_stmnts(ss, prochandler),
            FromStmnt(precond, ss, postcond) => self.rev_from_stmnt(precond, ss, postcond, prochandler),
            IfStmnt(precond, ss1, ss2, postcond) => self.rev_if_stmnt(precond, ss1, ss2, postcond, prochandler),
            CallBack(func, expr) => self.callback_stmnt(func, expr),
            _ => unreachable!(),
        }
    }

    fn run_statement(&mut self, stmnt: &RevStmnt, prochandler: &ProcHandler) {
        match stmnt {
            PlusEq(rhs, lhs) => self.plus_eq(rhs, lhs),
            MinusEq(rhs, lhs) => self.minus_eq(rhs, lhs),
            Swap(rhs, lhs) => self.swap(rhs, lhs),
            Call(s) => self.call_stmnt(s, prochandler),
            Stmnts(ss) => self.many_stmnts(ss, prochandler),
            FromStmnt(precond, ss, postcond) => self.from_stmnt(precond, ss, postcond, prochandler),
            IfStmnt(precond, ss1, ss2, postcond) => self.if_stmnt(precond, ss1, ss2, postcond, prochandler),
            CallBack(func, expr) => self.callback_stmnt(func, expr),
            _ => unreachable!(),
        }
    }

    fn callback_stmnt(&self, func: &Box<Fn(&RevType) -> ()>, expr: &RevExpr){
        let res = self.calc_expr(expr);
        func(&res);
    }

    fn if_stmnt(&mut self, precond: &RevExpr, ss1: &RevStmnt, ss2: &RevStmnt, postcond: &RevExpr, prochandler: &ProcHandler) {
        let pre_res = self.calc_expr(precond);

        if pre_res.is_true() {
            self.run_statement(ss1, prochandler);
            let post_res = self.calc_expr(postcond);
            assert!(post_res.is_true());
        } else {
            self.run_statement(ss2, prochandler);
            let post_res = self.calc_expr(postcond);
            assert!(!post_res.is_true());
        }
    }

    fn rev_if_stmnt(&mut self, precond: &RevExpr, ss1: &RevStmnt, ss2: &RevStmnt, postcond: &RevExpr, prochandler: &ProcHandler) {
        let post_res = self.calc_expr(postcond);

        if post_res.is_true() {
            self.run_rev_statement(ss1, prochandler);
            let pre_res = self.calc_expr(precond);
            assert!(pre_res.is_true());
        } else {
            self.run_rev_statement(ss2, prochandler);
            let pre_res = self.calc_expr(precond);
            assert!(!pre_res.is_true());
        }
    }

    fn from_stmnt(&mut self, precond: &RevExpr, ss: &RevStmnt, postcond: &RevExpr, prochandler: &ProcHandler) {
        let pre_res = self.calc_expr(precond);
        assert!(pre_res.is_true());

        // Black magic do while loop
        while {
            self.run_statement(ss, prochandler);
            let exit_res = self.calc_expr(postcond);
            !exit_res.is_true()
        }{}
    }

    fn rev_from_stmnt(&mut self, precond: &RevExpr, ss: &RevStmnt, postcond: &RevExpr, prochandler: &ProcHandler) {
        let post_res = self.calc_expr(postcond);
        assert!(post_res.is_true());

        // Black magic do while loop
        while {
            self.run_rev_statement(ss, prochandler);
            let exit_res = self.calc_expr(precond);
            !exit_res.is_true()
        }{}
    }


    fn call_stmnt(&mut self, p: &String, prochandler: &ProcHandler) {
        let new_procedure = prochandler.get_proc(p);
        self.run_proc(new_procedure, prochandler);
    }

    fn uncall_stmnt(&mut self, p: &String, prochandler: &ProcHandler) {
        let new_procedure = prochandler.get_proc(p);
        self.run_rev_proc(new_procedure, prochandler);
    }

    fn many_stmnts(&mut self, ss_box: &Box<[RevStmnt]>, prochandler: &ProcHandler){
        for s in ss_box.iter() {
            self.run_statement(s, prochandler);
        }
    }

    fn rev_many_stmnts(&mut self, ss_box: &Box<[RevStmnt]>, prochandler: &ProcHandler){
        for s in ss_box.iter().rev() {
            self.run_rev_statement(s, prochandler);
        }
    }

     fn get_var(&self, var: &String) -> &RevType {
        self.store.get(var).unwrap()
    }

    fn calc_expr(&self, expr: &RevExpr) -> RevType {
        match expr {
            Plus(expr1, expr2)  => self.plus(expr1, expr2),
            Minus(expr1, expr2)  => self.minus(expr1, expr2),
            Equal(expr1, expr2)  => self.equal(expr1, expr2),
            Lit(lit)            => lit.clone(),
            Var(var)            => self.get_var(var).clone(),
            _ => unreachable!(),
        }
    }

    fn equal(&self, rhs: &RevExpr, lhs: &RevExpr) -> RevType {
        let rhs_val = self.calc_expr(rhs);
        let lhs_val = self.calc_expr(lhs);
        RevType::eq(&rhs_val, &lhs_val)
    }

    fn plus(&self, rhs: &RevExpr, lhs: &RevExpr) -> RevType {
        let rhs_val = self.calc_expr(rhs);
        let lhs_val = self.calc_expr(lhs);
        RevType::rev_add(&rhs_val, &lhs_val)
    }

    fn minus(&self, rhs: &RevExpr, lhs: &RevExpr) -> RevType {
        let rhs_val = self.calc_expr(rhs);
        let lhs_val = self.calc_expr(lhs);
        RevType::rev_sub(&rhs_val, &lhs_val)
    }

    fn swap(&mut self, rhs: &RevExpr, lhs: &RevExpr) {
        let rhs_s = match rhs {
            Var(s) => s,
            _ => unreachable!(),
        };
        let lhs_s = match lhs {
            Var(s) => s,
            _ => unreachable!(),
        };

        let rhs_v = self.get_var(rhs_s).clone();
        let lhs_v = self.get_var(lhs_s).clone();

        *self.store.get_mut(rhs_s).unwrap() = lhs_v;
        *self.store.get_mut(lhs_s).unwrap() = rhs_v;
    }

    fn plus_eq(&mut self, rhs: &RevExpr, lhs: &RevExpr) {
        let var = match rhs {
            Var(s) => s,
            _ => unreachable!(),
        };
        let res = self.plus(rhs, lhs);
        *self.store.get_mut(var).unwrap() = res;
    }

    fn minus_eq(&mut self, rhs: &RevExpr, lhs: &RevExpr) {
        let var = match rhs {
            Var(s) => s,
            _ => unreachable!(),
        };
        let res = self.minus(rhs, lhs);
        *self.store.get_mut(var).unwrap() = res;
    }


    fn create_var(&mut self, vname: String, val: RevType) {
        self.store.insert(vname, val);
    }

    fn get_store(&self) -> Vec<(&String, &RevType)>{
        let mut v = Vec::new();
        for (key, val) in self.store.iter() {
            v.push((key, val));
        }
        v
    }

    fn get_store_2(&self) -> &HashMap<String, RevType>{
        &self.store
    }
}

struct RPU {
    procs: HashMap<String, RevStmnt>,
    store: HashMap<String, RevType>,

    storehandler: StoreHandler,
    prochandler: ProcHandler
}

impl RPU {

    fn new() -> RPU {
        RPU {
            procs: HashMap::new(),
            store: HashMap::new(),
            storehandler: StoreHandler::new(),
            prochandler: ProcHandler::new(),
        }
    }

    fn call_proc(&mut self, p: &String) {
    
        let proc_to_run = self.prochandler.get_proc(p);
        self.storehandler.run_proc(proc_to_run, &self.prochandler);
    }

    fn uncall_proc(&mut self, p: &String) {
    
        let proc_to_run = self.prochandler.get_proc(p);
        self.storehandler.run_rev_proc(proc_to_run, &self.prochandler);
    }

    fn load_proc(&mut self, pname: String, ss: RevStmnt) {
        self.prochandler.procs.insert(pname, ss);
    }
    

    fn create_var(&mut self, vname: String, val: RevType) {
        //self.store.insert(vname, val);
        self.storehandler.store.insert(vname, val);
    }

    fn get_store(&self) -> Vec<(&String, &RevType)> {
        self.storehandler.get_store()
    }

    fn get_store_2(&self) -> &HashMap<String, RevType> {
        self.storehandler.get_store_2()
    }
}


fn plus(rhs: RevExpr, lhs: RevExpr) -> RevExpr {
    Plus(Box::new(rhs), Box::new(lhs))
}

fn minus(rhs: RevExpr, lhs: RevExpr) -> RevExpr {
    Minus(Box::new(rhs), Box::new(lhs))
}

fn equal(rhs: RevExpr, lhs: RevExpr) -> RevExpr {
    Equal(Box::new(rhs), Box::new(lhs))
}

fn int(x: i64) -> RevExpr {
    Lit(RevType::Revi64(x))
}

fn var(x: &str) -> RevExpr {
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

use RevExpr::*;
use RevStmnt::*;

mod crankset;

fn main() {

    //let mactest = IF![(5) THEN{ 1;2;3} ELSE{ 4;5;6} FI(5)];

    //println!("{:?}", mactest);
    /*
    let mactest = IfStmnt(equal(var("n"), int(0)),
            Box::new(Stmnts(Box::new([PlusEq(var("x1"), int(1)), PlusEq(var("x2"), int(1))]))),
            Box::new(Stmnts(Box::new([MinusEq(var("n"), int(1)), Call("fib".to_owned()), PlusEq(var("x1"), var("x2")), Swap(var("x1"), var("x2"))]))),
            equal(var("x1"), var("x2")));
    */

    let mactest = 
        IF![(equal(var("n"), int(0)))
        THEN{
            PlusEq(var("x1"), int(1));
            PlusEq(var("x2"), int(1))
        }ELSE{
            MinusEq(var("n"), int(1));
            Call("fib".to_owned());
            PlusEq(var("x1"), var("x2"));
            Swap(var("x1"), var("x2"))
        }FI(equal(var("x1"), var("x2")))];

    let testitest = stmnts![1;2;3];
    println!("{:?}", testitest);
    let testitest2 = [1,2,3,];

    let chartest = var!(b);
    //println!("{:?}", chartest);

    let procedure = 
    
                Stmnts(Box::new([
                    PlusEq(var("x"), int(15)),
                    PlusEq(var("y"), plus(plus(int(15), int(15)), int(20))),
                ]));


    let procedure2 = stmnts![
        PlusEq(var!(x), int(15));
        PlusEq(var!(y), plus(plus(int(15), int(15)), int(20)))
    ];
    
    let mut store = HashMap::new();
    
    let callback_test = |x: &RevType| {
        println!("{:?}", x);
    };

    //let test_2 = callback_test.clone();

    let q: Box<RevExt> = Box::new(Testi {tag: 2});
    

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

    let mut rpu = RPU::new();
    //rpu.call_proc(&"TestProc".to_string());

    //rpu.create_var("x".to_string(), Revi64(1));
    //rpu.plus_eq(&var("x"), &plus(int(2), int(3)));
    //let res2 = rpu.plus(&var("x"), &int(4));
    //println!("Hello, world!, {:?}", res2);

    rpu.load_proc("Test".to_string(), PlusEq(var("x"), plus(int(2), int(3))));
    rpu.load_proc("Test2".to_string(), Call("Test".to_string()));
    rpu.load_proc("Infi".to_string(), Call("Infi".to_string()));

    rpu.load_proc("more".to_string(), Stmnts(Box::new([PlusEq(var("x"), plus(int(2), int(3))), PlusEq(var("x"), plus(int(2), int(3)))])));

    let freefall = 
        FromStmnt(equal(var("ts"), int(0)), 
        Box::new(Stmnts(Box::new([
            CallBack(Box::new(callback_test), var("ts")),
            PlusEq(var("ts"), int(1)),
            PlusEq(var("v"), int(10)),
            MinusEq(var("h"), minus(var("v"), int(5)))]))),
        equal(var("ts"), var("te")));

    rpu.load_proc("freefall".to_string(), freefall);

    //rpu.create_var("x".to_string(), Revi64(2));
    //rpu.call_proc(&"more".to_string());

    //rpu.create_var("ts".to_string(), Revi64(0));
    //rpu.create_var("v".to_string(), Revi64(0));
    //rpu.create_var("h".to_string(), Revi64(176));
    //rpu.create_var("te".to_string(), Revi64(3));

    //rpu.call_proc(&"freefall".to_string());
    //rpu.uncall_proc(&"freefall".to_string());
    
    //let addtox = Stmnts(Box::new([PlusEq(var("x"), int(1))]));
    //rpu.load_proc("addtox".to_string(), addtox);
    //rpu.create_var("x".to_string(), Revi64(0));
    //rpu.call_proc(&"addtox".to_string());
    //rpu.uncall_proc(&"addtox".to_string());

    /*
    let fib = IfStmnt(equal(var("n"), int(0)),
            Box::new(Stmnts(Box::new([PlusEq(var("x1"), int(1)), PlusEq(var("x2"), int(1))]))),
            Box::new(Stmnts(Box::new([MinusEq(var("n"), int(1)), Call("fib".to_owned()), PlusEq(var("x1"), var("x2")), Swap(var("x1"), var("x2"))]))),
            equal(var("x1"), var("x2")));
    */

    let fib = 
        IF![(equal(var!(n), int(0)))
        THEN{
            PlusEq(var!(x1), int(1));
            PlusEq(var!(x2), int(1))
        }ELSE{
            MinusEq(var!(n), int(1));
            Call("fib".to_owned());
            PlusEq(var!(x1), var!(x2));
            Swap(var!(x1), var!(x2))
        }FI(equal(var!(x1), var!(x2)))];
    
    rpu.load_proc("fib".to_string(), fib);
    rpu.create_var("n".to_string(), Revi64(0));
    rpu.create_var("x1".to_string(), Revi64(5));
    rpu.create_var("x2".to_string(), Revi64(8));

    rpu.uncall_proc(&"fib".to_string());

    {
        let finalstore = rpu.get_store_2();
        println!("Hello, world!, {:?}", finalstore);
    }

    rpu.call_proc(&"fib".to_string());

    {
        let finalstore = rpu.get_store_2();
        println!("Hello, world!, {:?}", finalstore);
    }



}
