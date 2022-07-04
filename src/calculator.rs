/* Calculator implementations */

use std::rc::Rc;
use std::cell::Cell;
pub use crate::gtk::prelude::*;
pub use crate::defs::*;
pub use crate::gtk::{Entry};

pub fn set_value(num_counter: i32, val1: &Rc<Cell<i64>>, val2: &Rc<Cell<i64>>, num: i64) {
    if num_counter == 0 {
        val1.set(val1.get() * 10 + num);
    }
    if num_counter == 1 {
        val2.set(val2.get() * 10 + num);
    }
}

pub fn operation(pre_ops: char, val1: &Rc<Cell<i64>>, val2: i64) {
    match pre_ops {
        ADD => val1.set(val1.get() + val2),
        SUBTRACT => val1.set(val1.get() - val2),
        MULTIPLY => val1.set(val1.get() * val2),
        _=> ()
    }
    if pre_ops == DIVIDE && val2 == 0 {
        println!("Division by zero error");
    }
    else if pre_ops == DIVIDE && val2 != 0 {
        val1.set(val1.get() / val2);
    }
}

pub fn equation_result(cur_ops: char, val1: &Rc<Cell<i64>>, val2: i64) -> std::string::String {
    let mut result = String::from("= ").to_owned();
    match cur_ops {
        ADD => {val1.set(val1.get() + val2);},
        SUBTRACT => {val1.set(val1.get() - val2);},
        MULTIPLY => {val1.set(val1.get() * val2);},
        _=> ()
    }
    if cur_ops == DIVIDE && val2 != 0 {
        val1.set(val1.get() / val2);
    }
    if cur_ops == DIVIDE && val2 == 0 {
        result =  String::from("Division by zero error");
    }
    else {
        result.push_str(&val1.get().to_string());
    }
    return result;
}

pub fn clear_entry(pre_ops: &Rc<Cell<char>>, entry: &Entry) {
    if pre_ops.get() == EQUALS {
        entry.set_text("");
        pre_ops.set(NONE);
    }
}