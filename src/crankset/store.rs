use crankset::types::RevType;
use crankset::lang::RevExpr;
use crankset::lang::RevExpr::*;
use crankset::lang::RevStmnt;
use crankset::lang::RevStmnt::*;
use crankset::procs::ProcHandler;
use std::collections::HashMap;


pub struct StoreHandler {
    store: HashMap<String, RevType>,
}

impl StoreHandler {

    pub fn new() -> StoreHandler {
        StoreHandler {
            store: HashMap::new(),
        }
    }

    pub fn run_proc(&mut self, stmnt: &RevStmnt, prochandler: &ProcHandler) {
        self.run_statement(stmnt, prochandler)
    }

    pub fn run_rev_proc(&mut self, stmnt: &RevStmnt, prochandler: &ProcHandler) {
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


    pub fn create_var(&mut self, vname: String, val: RevType) {
        self.store.insert(vname, val);
    }

    pub fn get_store(&self) -> Vec<(&String, &RevType)>{
        let mut v = Vec::new();
        for (key, val) in self.store.iter() {
            v.push((key, val));
        }
        v
    }

    pub fn get_store_2(&self) -> &HashMap<String, RevType>{
        &self.store
    }
}