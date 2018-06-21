

use std::any::Any;
use std::fmt;
use crankset::types::RevType::*;

pub trait RevExt {
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


pub enum RevType {
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


impl RevType {

    pub fn rev_add(rhs: &RevType, lhs: &RevType) -> RevType {
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

    pub fn rev_sub(rhs: &RevType, lhs: &RevType) -> RevType {
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

    pub fn rev_mult(rhs: &RevType, lhs: &RevType) -> RevType {
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

    pub fn rev_div(rhs: &RevType, lhs: &RevType) -> RevType {
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

    pub fn geq(rhs: &RevType, lhs: &RevType) -> RevType {
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

    pub fn eq(rhs: &RevType, lhs: &RevType) -> RevType {
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

    pub fn not(var: &RevType) -> RevType {
        match var {
            Revbool(b) => Revbool(!b),
            RevExtension(ext) => ext.not(),
            _ => unreachable!(),
        }
    }

    pub fn destroy(var: &RevType) -> RevType {
        if !RevType::is_empty(var) {
            unreachable!();
        }
        Empty
    }

    pub fn is_empty(var: &RevType) -> bool {
        match var {
            Revi64(var_i) => *var_i == 0,
            Revf64(var_f) => *var_f == 0.0,
            RevExtension(var_ext) => var_ext.is_empty(),
            _ => unreachable!(),
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            Revbool(b) => b.clone(),
            RevExtension(ext) => ext.is_true(),
            _ => unreachable!(),
        }
    }

}


// Maybe move to a util
pub fn extension_downcast<T>(r: &RevType) -> &T where T: 'static{
    let t = match r {
            RevExtension(x) => x,
            _ => unimplemented!()
    };

    match t.as_any().downcast_ref::<T>() {
            Some(b) => b,
            None    => panic!("Invalid downcast")
    }
}

pub fn safe_extension_downcast<T>(r: &RevType) -> Option<&T> where T: 'static{
    let t = match r {
            RevExtension(x) => x,
            _ => return None,
    };

    t.as_any().downcast_ref::<T>()
}
