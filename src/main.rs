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

    fn run_statement(&mut self, stmnt: &RevStmnt, prochandler: &ProcHandler) {
        match stmnt {
            PlusEq(rhs, lhs) => self.plus_eq(rhs, lhs),
            Call(s) => self.call_stmnt(s, prochandler),
            Stmnts(ss) => self.many_stmnts(ss, prochandler),
            _ => unreachable!(),
        }
    }

    fn call_stmnt(&mut self, p: &String, prochandler: &ProcHandler) {
        let new_procedure = prochandler.get_proc(p);
        self.run_proc(new_procedure, prochandler);
    }

    fn many_stmnts(&mut self, ss_box: &Box<[RevStmnt]>, prochandler: &ProcHandler){
        for s in ss_box.iter() {
            self.run_statement(s, prochandler);
        }
    }

     fn get_var(&self, var: &String) -> &RevType {
        self.store.get(var).unwrap()
    }

    fn calc_expr(&self, expr: &RevExpr) -> RevType {
        match expr {
            Plus(expr1, expr2)  => self.plus(expr1, expr2),
            Lit(lit)            => lit.clone(),
            Var(var)            => self.get_var(var).clone(),
            _ => unreachable!(),
        }
    }

    fn plus(&self, rhs: &RevExpr, lhs: &RevExpr) -> RevType {
        let rhs_val = self.calc_expr(rhs);
        let lhs_val = self.calc_expr(lhs);
        RevType::rev_add(&rhs_val, &lhs_val)
    }

    fn plus_eq(&mut self, rhs: &RevExpr, lhs: &RevExpr) {
        let var = match rhs {
            Var(s) => s,
            _ => unreachable!(),
        };
        let res = self.plus(rhs, lhs);
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
    /*

    fn get_proc<'a>(procs: &'a HashMap<String, RevStmnt>, p: &String) -> &'a RevStmnt{
        procs.get(p).unwrap()
    }


    fn get_var(&self, var: &String) -> &RevType {
        self.store.get(var).unwrap()
    }

    fn calc_expr(&self, expr: &RevExpr) -> RevType {
        match expr {
            Plus(expr1, expr2)  => self.plus(expr1, expr2),
            Lit(lit)            => lit.clone(),
            Var(var)            => self.get_var(var).clone(),
            _ => unreachable!(),
        }
    }

    fn plus(&self, rhs: &RevExpr, lhs: &RevExpr) -> RevType {
        let rhs_val = self.calc_expr(rhs);
        let lhs_val = self.calc_expr(lhs);
        RevType::rev_add(&rhs_val, &lhs_val)
    }

    fn plus_eq(&mut self, rhs: &RevExpr, lhs: &RevExpr) {
        let var = match rhs {
            Var(s) => s,
            _ => unreachable!(),
        };
        let res = self.plus(rhs, lhs);
        *self.store.get_mut(var).unwrap() = res;
    }


    fn run_proc(&mut self, ss: &RevStmnt) {
        unreachable!();
    }
    */
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
}


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

    rpu.create_var("x".to_string(), Revi64(2));
    rpu.call_proc(&"more".to_string());

    let finalstore = rpu.get_store();
    println!("Hello, world!, {:?}", finalstore);

}
