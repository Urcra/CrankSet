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
        write!(f, "{:?}", self)
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

    let rational_1 = BigRational::new(1_i32.to_bigint().unwrap(), 1_i32.to_bigint().unwrap());
    //let ratext = ext!(rattest);

    PROCEDURE!{addtwo:
        PLUSEQ!(k, ext!(rational_1));
        CALL!(fib)
    }    


    let callback_test = |x: &RevType| {
        println!("{:?}", x);
    };


    let mut rpu = RPU::new();



    let freefall = 
        FROM![(equal(var!(ts), int(0)))
        DO {
            CALLBACK!(callback_test, var!(ts));
            PLUSEQ!(ts, int(1));
            MINUSEQ!(h, minus(var!(v), int(5)))
        } UNTIL (equal(var!(ts), var!(te)))];


    rpu.load_proc2("freefall".to_string(), freefall);



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

}
