/* Calculator implementations */

use std::rc::Rc;
use std::cell::{Cell, RefCell};
pub use crate::gtk::prelude::*;
pub use crate::symbol_names::*;
pub use crate::gtk::Entry;

pub struct Values {
    pub num1: Rc<Cell<f64>>,
    pub num2: Rc<Cell<f64>>,
}

pub struct Operators {
    pub current: Rc<Cell<char>>,
    pub previous: Rc<Cell<char>>,
}

pub fn set_value(num_counter: i32, vals: &Rc<RefCell<Values>>, num: f64) {
    if num_counter == 0 {
        vals.borrow().num1.set(vals.borrow().num1.get() * 10.0 + num); 
    }
    if num_counter == 1 {
        vals.borrow().num2.set(vals.borrow().num2.get() * 10.0 + num); 
    }
}

pub fn calculation(operation: char, vals: &Rc<RefCell<Values>>) {
    match operation {
        ADD => vals.borrow().num1.set(vals.borrow().num1.get() + vals.borrow().num2.get()),
        SUBTRACT => vals.borrow().num1.set(vals.borrow().num1.get() - vals.borrow().num2.get()),
        MULTIPLY => vals.borrow().num1.set(vals.borrow().num1.get() * vals.borrow().num2.get()),
        _=> ()
    }
    if operation == DIVIDE && vals.borrow().num2.get() != 0.0 {
        vals.borrow().num1.set(vals.borrow().num1.get() / vals.borrow().num2.get());
    }
}

pub fn check_divison_by_zero(ops: char, val2: f64, divide_zero: &Rc<Cell<bool>>) {
    /* Set division by zero status */
    if ops == DIVIDE && val2 == 0.0 {
        divide_zero.set(true);
    }
}

pub fn operation(symbol_operator: char, num_counter: &Rc<Cell<i32>>, ops: &Rc<RefCell<Operators>>,
                 vals: &Rc<RefCell<Values>>, divide_zero: &Rc<Cell<bool>>) {
    /* Do the operation when two values are received for calucaltion */ 
    if num_counter.get() == 2 {
        // Set previous and current operation
        ops.borrow().previous.set(ops.borrow().current.get());
        ops.borrow().current.set(symbol_operator);
        
        // Do calculation
        calculation(ops.borrow().previous.get(), &vals);

        // Check divison by zero
        check_divison_by_zero(ops.borrow().previous.get(), vals.borrow().num2.get(), divide_zero);
        
        // reset variables
        num_counter.set(num_counter.get() - 1);
        vals.borrow().num2.set(0.0);
    }
    else {
        ops.borrow().current.set(symbol_operator);
    }
}

pub fn equation_result(ops: &Rc<RefCell<Operators>>, vals: &Rc<RefCell<Values>>, divide_zero: &Rc<Cell<bool>>) 
    -> std::string::String {
    let mut result = String::from("= ");

    // Check divison by zero
    check_divison_by_zero(ops.borrow().current.get(), vals.borrow().num2.get(), divide_zero);

    // Do calculation
    calculation(ops.borrow().current.get(), &vals);

    if divide_zero.get() {
        // The result stores Divsion by Zero Error Message
        result =  String::from("Divide by 0 error");
    }
    else {
        // append the first value to the result
        // in other words, it just appends the result after the equals sign
        result.push_str(&vals.borrow().num1.get().to_string());
    }
    result
}

pub fn clear_entry_before_calculation(pre_ops: &Rc<Cell<char>>, entry: &Entry) {
    /* Clear entry once the user clicks a number after getting the 
     * result of the calculation */
    if pre_ops.get() == EQUALS {
        entry.set_text("");
        pre_ops.set(NONE);
    }
}
