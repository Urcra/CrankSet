/*
use std::collections::HashMap;

#[derive(Debug)]
enum Reversible {
    RevInt(u32),
    RevFloat(f32),
}
*/
use std::mem;

#[derive(Debug)]
enum RevType {
    Revi64,
    Revf64,
    RevList,
    RevExtension,
}

trait RevItem {
    fn rev_type(&self) -> RevType;
    fn rev_add<T: RevItem>(Self, T) -> Box<T>;
    fn rev_sub<T: RevItem>(Self, T) -> Box<T>;
    fn prim_i64(self) -> i64;
    fn prim_f64(self) -> f64;
}


impl RevItem for i64 {
    fn rev_type(&self) -> RevType{
        RevType::Revi64
    }

    fn rev_add<T: RevItem>(x: i64, y: T) -> Box<T>{
        unsafe{
            match y.rev_type() {
                RevType::Revi64 => mem::transmute(Box::new(x + y.prim_i64())),
                _ => unreachable!(),
            }
        }
    }

    fn rev_sub<T: RevItem>(x: i64, y: T) -> Box<T>{
        unsafe{
            match y.rev_type() {
                RevType::Revi64 => mem::transmute(Box::new(x - y.prim_i64())),
                _ => unreachable!(),
            }
        }
    }

    fn prim_i64(self) -> i64 {
        self
    }

    fn prim_f64(self) -> f64 {
        self as f64
    }
    
}

impl RevItem for f64 {
    fn rev_type(&self) -> RevType{
        RevType::Revf64
    }

    fn rev_add<T: RevItem>(x: f64, y: T) -> Box<T>{
        unsafe{
            match y.rev_type() {
                RevType::Revf64 => mem::transmute(Box::new(x + y.prim_f64())),
                RevType::Revi64 => {
                    let lhs: f64 = x;
                    let rhs: f64 = y.prim_f64();
                    let res: Box<f64> = Box::new(lhs + rhs);
                    mem::transmute(res)
                },
                _ => unreachable!(),
            }
        }
    }

    fn rev_sub<T: RevItem>(x: f64, y: T) -> Box<T>{
        unsafe{
            match y.rev_type() {
                RevType::Revf64 => mem::transmute(Box::new(x - y.prim_f64())),
                _ => unreachable!(),
            }
        }
    }

    fn prim_i64(self) -> i64 {
        self as i64
    }

    fn prim_f64(self) -> f64 {
        self
    }
    
}



fn main() {

    let a = 5.0;
    let b = 10;
    
    let c = *RevItem::rev_add(a, b);
    
    //let d: f64 = *RevItem::rev_add(c, b);
    println!("Hello, world!, {:?}", c.rev_type());

}


/*
trait Pedals {
    fn revadd(Self, Self) -> Self;
}


trait Bike {
    fn revtype(&self) -> RevType;
    fn asrevnum<T: Pedals>(&self) -> T;
}


impl Bike for u32 {
    fn revtype(&self) -> RevType {
        RevType::RevNum
    }

    fn asrevnum<T: Pedals>(&self) -> T {
        unreachable!()
    }
}

impl Pedals for u32 {
    fn revadd(x: u32, y: u32) -> u32 {
        x + y
    }
}



fn main() {
    let mut store = HashMap::new();

    let test1: u32 = 5;
    let test2: u32 = 6;

    let a = 6.5 as u32;
    

    let test3 = Pedals::revadd(test1, test2);

    let rev_int = Reversible::RevInt(5);

    store.insert("x", rev_int);
    
    println!("Hello, world!, {}", test3);
}
*/

