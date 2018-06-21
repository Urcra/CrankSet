
use std::collections::HashMap;
use crankset::lang::RevStmnt;

pub struct ProcHandler {
    procs: HashMap<String, RevStmnt>,
}

impl ProcHandler {

    pub fn new() -> ProcHandler {
        ProcHandler {
            procs: HashMap::new(),
        }
    }

    pub fn get_proc(&self, p: &String) -> &RevStmnt {
        self.procs.get(p).unwrap()
    }

    pub fn load_proc(&mut self, pname: String, ss: RevStmnt) {
        self.procs.insert(pname, ss);
    }

}