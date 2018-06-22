pub mod types;
pub mod lang;
mod procs;
mod store;

#[macro_use]
pub mod utils;


use crankset::types::RevType;
use crankset::lang::RevStmnt;
use crankset::procs::ProcHandler;
use crankset::store::StoreHandler;
use std::collections::HashMap;

pub struct RPU {
    storehandler: StoreHandler,
    prochandler: ProcHandler
}

impl RPU {

    pub fn new() -> RPU {
        RPU {
            storehandler: StoreHandler::new(),
            prochandler: ProcHandler::new(),
        }
    }

    pub fn run(&mut self) {
        let proc_to_run = self.prochandler.get_proc(&"main".to_string());
        self.storehandler.run_proc(proc_to_run, &self.prochandler);
    }

    pub fn call_proc(&mut self, p: &String) {
    
        let proc_to_run = self.prochandler.get_proc(p);
        self.storehandler.run_proc(proc_to_run, &self.prochandler);
    }

    pub fn uncall_proc(&mut self, p: &String) {
    
        let proc_to_run = self.prochandler.get_proc(p);
        self.storehandler.run_rev_proc(proc_to_run, &self.prochandler);
    }

    pub fn load_proc2(&mut self, pname: String, ss: RevStmnt) {
        self.prochandler.load_proc(pname, ss);
    }

    pub fn load_proc(&mut self, proced: (String, RevStmnt)) {
        let (pname, ss) = proced;
        self.prochandler.load_proc(pname, ss);
    }
    

    pub fn create_var(&mut self, vname: String, val: RevType) {
        self.storehandler.create_var(vname, val);
    }

    pub fn with_vals(&mut self, vars: &[(String, RevType)]) {
        self.storehandler.with_vals(vars);
    }

    pub fn with_vars(&mut self, vars: &[String]) {
        self.storehandler.with_vars(vars);
    }

    pub fn get_store(&self) -> Vec<(&String, &RevType)> {
        self.storehandler.get_store()
    }

    pub fn get_store_2(&self) -> &HashMap<String, RevType> {
        self.storehandler.get_store_2()
    }
}