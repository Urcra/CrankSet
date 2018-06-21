mod types;
mod lang;
mod procs;
mod store;
mod utils;


use crankset::types::RevType;
use crankset::lang::RevStmnt;
use crankset::procs::ProcHandler;
use crankset::store::StoreHandler;
use std::collections::HashMap;

pub struct RPU {
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
        self.prochandler.load_proc(pname, ss);
    }
    

    fn create_var(&mut self, vname: String, val: RevType) {
        self.storehandler.create_var(vname, val);
    }

    fn get_store(&self) -> Vec<(&String, &RevType)> {
        self.storehandler.get_store()
    }

    fn get_store_2(&self) -> &HashMap<String, RevType> {
        self.storehandler.get_store_2()
    }
}