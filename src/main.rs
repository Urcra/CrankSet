#[macro_use]
mod crankset;

use crankset::utils::*;
use crankset::types::RevType::*;
use crankset::lang::RevExpr::*;
use crankset::lang::RevStmnt::*;
use crankset::types::RevType;
use crankset::types::RevExt;

use std::collections::HashMap;

use std::any::Any;
use std::fmt;

use crankset::RPU;

struct Testi {
    tag: u32,
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
