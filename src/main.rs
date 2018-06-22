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

extern crate num_rational;
extern crate num_bigint;
use num_rational::BigRational;
use num_bigint::BigInt;
use num_bigint::ToBigInt;


impl RevExt for BigRational {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
    fn clone(&self) -> Self where Self: Sized {
        std::clone::Clone::clone(self)
    }
    fn add_reverse_rhs(&self, lhs: &RevType) -> RevType {
        let lhs_ratio: &BigRational = extension_downcast::<BigRational>(lhs);
        RevExtension(Box::new(self + lhs_ratio))
    }
    fn add_reverse_lhs(&self, rhs: &RevType) -> RevType {
        let rhs_ratio: &BigRational = extension_downcast::<BigRational>(rhs);
        RevExtension(Box::new(rhs_ratio + self))
    }
    fn sub_reverse_rhs(&self, lhs: &RevType) -> RevType {
        let lhs_ratio: &BigRational = extension_downcast::<BigRational>(lhs);
        RevExtension(Box::new(self - lhs_ratio))
    }
    fn sub_reverse_lhs(&self, rhs: &RevType) -> RevType {
        let rhs_ratio: &BigRational = extension_downcast::<BigRational>(rhs);
        RevExtension(Box::new(rhs_ratio - self))
    }
    fn mult_reverse_rhs(&self, lhs: &RevType) -> RevType {
        let lhs_ratio: &BigRational = extension_downcast::<BigRational>(lhs);
        RevExtension(Box::new(self * lhs_ratio))
    }
    fn mult_reverse_lhs(&self, rhs: &RevType) -> RevType {
        let rhs_ratio: &BigRational = extension_downcast::<BigRational>(rhs);
        RevExtension(Box::new(rhs_ratio * self))
    }
    fn div_reverse_rhs(&self, lhs: &RevType) -> RevType {
        let lhs_ratio: &BigRational = extension_downcast::<BigRational>(lhs);
        RevExtension(Box::new(self / lhs_ratio))
    }
    fn div_reverse_lhs(&self, rhs: &RevType) -> RevType {
        let rhs_ratio: &BigRational = extension_downcast::<BigRational>(rhs);
        RevExtension(Box::new(rhs_ratio / self))
    }
    fn geq_rhs(&self, lhs: &RevType) -> RevType {
        let lhs_ratio: &BigRational = extension_downcast::<BigRational>(lhs);
        Revbool(self >= lhs_ratio)
    }
    fn geq_lhs(&self, rhs: &RevType) -> RevType {
        let rhs_ratio: &BigRational = extension_downcast::<BigRational>(rhs);
        Revbool(rhs_ratio >= self)
    }
    fn eq_rhs(&self, lhs: &RevType) -> RevType {
        let lhs_ratio: &BigRational = extension_downcast::<BigRational>(lhs);
        Revbool(self == lhs_ratio)
    }
    fn eq_lhs(&self, rhs: &RevType) -> RevType {
        let rhs_ratio: &BigRational = extension_downcast::<BigRational>(rhs);
        Revbool(rhs_ratio == self)
    }
    fn not(&self) -> RevType {
        // Not a boolean type
        unreachable!()
    }
    fn is_empty(&self) -> bool {
        self.is_integer() && (self.to_integer() == 0_i32.to_bigint().unwrap())
    }
    fn is_true(&self) -> bool {
        // Not a boolean type
        unreachable!()
    }
    fn as_any(&self) -> &Any {
        self
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

    //let q: Box<RevExt> = Box::new(Testi {tag: 2});
    
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

    rpu.load_proc2("Test".to_string(), PlusEq(var("x"), plus(int(2), int(3))));
    rpu.load_proc2("Test2".to_string(), Call("Test".to_string()));
    rpu.load_proc2("Infi".to_string(), Call("Infi".to_string()));

    rpu.load_proc2("more".to_string(), Stmnts(Box::new([PlusEq(var("x"), plus(int(2), int(3))), PlusEq(var("x"), plus(int(2), int(3)))])));

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

    let freefall = 
        FROM![(equal(var!(ts), int(0)))
        DO {
            CALLBACK!(callback_test, var!(ts));
            PLUSEQ!(ts, int(1));
            MINUSEQ!(h, minus(var!(v), int(5)))
        } UNTIL (equal(var!(ts), var!(te)))];


    rpu.load_proc2("freefall".to_string(), freefall);


    /*
    let fib = 
        IF![(equal(var!(n), int(0)))
        THEN{
            PLUSEQ!(x1, int(1));
            PLUSEQ!(x2, int(1))
        }ELSE{
            MINUSEQ!(n, int(1));
            CALL!(fib);
            PLUSEQ!(x1, var!(x2));
            SWAP!(x1, x2)
        }FI(equal(var!(x1), var!(x2)))];

    */

    PROCEDURE!{main:
        PLUSEQ!(n, int(4));
        CALL!(fib)
    }

    PROCEDURE!{fib:
        (IF![(equal(var!(n), int(0)))
        THEN{
            PLUSEQ!(x1, int(1));
            PLUSEQ!(x2, int(1))
        }ELSE{
            MINUSEQ!(n, int(1));
            CALL!(fib);
            PLUSEQ!(x1, var!(x2));
            SWAP!(x1, x2)
        }FI(equal(var!(x1), var!(x2)))])
    };
    
    
    load_proc!(rpu; fib);
    load_proc!(rpu; main);
    with_vars!(rpu; n, x1, x2);
    rpu.run();
    {
        let finalstore = rpu.get_store_2();
        println!("Hello, world!, {:?}", finalstore);
    }
    /*
    
    //rpu.create_var("n".to_string(), Revi64(0));
    //rpu.create_var("x1".to_string(), Revi64(5));
    //rpu.create_var("x2".to_string(), Revi64(8));
    
    //rpu.with_vals(&[("x2".to_string(), Revi64(0)), ("x2".to_string(), Revi64(0))]);

    with_vars!(rpu; n, x1, x2);

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

    */

}
