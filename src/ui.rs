/* User Interface */

use std::rc::Rc;
use std::cell::Cell;

use gtk4 as gtk;
use gtk::{Application, ApplicationWindow, Grid};
use glib_macros::clone;

pub use crate::defs::*;
pub use crate::button::*;
pub use crate::calculator::*;

pub fn build_ui(application: &Application) {
    // Create buttons
    let button_num0 = create_button("0");
    let button_num1 = create_button("1");
    let button_num2 = create_button("2");
    let button_num3 = create_button("3");
    let button_num4 = create_button("4");
    let button_num5 = create_button("5");
    let button_num6 = create_button("6");
    let button_num7 = create_button("7");
    let button_num8 = create_button("8");
    let button_num9 = create_button("9");

    let button_plus = create_button("+");
    let button_minus = create_button("-");
    let button_multiply = create_button("×");
    let button_divide = create_button("÷");
    let button_equals = create_button("=");
    let button_clear = create_button("clear");
    let entry = create_entry();

    button_clear.set_widget_name("clear_button");
    button_equals.set_widget_name("equals_button");

    // A mutable integer
    let val1: Rc<Cell<i64>> = Rc::new(Cell::new(0));
    let val2: Rc<Cell<i64>> = Rc::new(Cell::new(0));
    let num_counter = Rc::new(Cell::new(0));
    let cur_ops = Rc::new(Cell::new(NONE));
    let pre_ops = Rc::new(Cell::new(NONE));

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_num0.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong pre_ops, @strong entry =>
        move |_| {
            clear_entry(&pre_ops, &entry);
            set_value(num_counter.get(), &val1, &val2, 0);
            entry.insert_text("0", &mut -1);
        }));
    button_num1.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong pre_ops, @strong entry =>
        move |_| {
            clear_entry(&pre_ops, &entry);
            set_value(num_counter.get(), &val1, &val2, 1);
            entry.insert_text("1", &mut -1);
        }));
    button_num2.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong pre_ops, @strong entry =>
        move |_| {
            clear_entry(&pre_ops, &entry);
            set_value(num_counter.get(), &val1, &val2, 2);
            entry.insert_text("2", &mut -1);
        }));
    button_num3.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong pre_ops, @strong entry =>
        move |_| {
            clear_entry(&pre_ops, &entry);
            set_value(num_counter.get(), &val1, &val2, 3);
            entry.insert_text("3", &mut -1);
        }));
    button_num4.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong pre_ops, @strong entry =>
        move |_| {
            clear_entry(&pre_ops, &entry);
            set_value(num_counter.get(), &val1, &val2, 4);
            entry.insert_text("4", &mut -1);
        }));
    button_num5.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong pre_ops, @strong entry =>
        move |_| {
            clear_entry(&pre_ops, &entry);
            set_value(num_counter.get(), &val1, &val2, 5);
            entry.insert_text("5", &mut -1);
        }));
    button_num6.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong pre_ops, @strong entry =>
        move |_| {
            clear_entry(&pre_ops, &entry);
            set_value(num_counter.get(), &val1, &val2, 6);
            entry.insert_text("6", &mut -1);
        }));
    button_num7.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong pre_ops, @strong entry =>
        move |_| {
            clear_entry(&pre_ops, &entry);
            set_value(num_counter.get(), &val1, &val2, 7);
            entry.insert_text("7", &mut -1);
        }));
    button_num8.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong pre_ops, @strong entry =>
        move |_| {
            clear_entry(&pre_ops, &entry);
            set_value(num_counter.get(), &val1, &val2, 8);
            entry.insert_text("8", &mut -1);
        }));
    button_num9.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong pre_ops, @strong entry =>
        move |_| {
            clear_entry(&pre_ops, &entry);
            set_value(num_counter.get(), &val1, &val2, 9);
            entry.insert_text("9", &mut -1);
        }));
    
    button_plus.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong cur_ops, @strong pre_ops, @strong entry =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);

            if num_counter.get() == 2 {
                // Set previous and current operation
                pre_ops.set(cur_ops.get());
                cur_ops.set(ADD);

                // Do operation
                operation(pre_ops.get(), &val1, val2.get());

                // Decrease the num counter and reset num2
                num_counter.set(num_counter.get() - 1);
                val2.set(0);
            }
            else {
                cur_ops.set(ADD);
            }

            entry.insert_text("+", &mut -1);

        }));

    button_minus.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong cur_ops, @strong pre_ops, @strong entry =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);

            if num_counter.get() == 2 {
                // Set previous and current operation
                pre_ops.set(cur_ops.get());
                cur_ops.set(SUBTRACT);

                // Do operation
                operation(pre_ops.get(), &val1, val2.get());

                //decrease the num counter and reset num2
                num_counter.set(num_counter.get() - 1);
                val2.set(0);
            }
            else {
                cur_ops.set(SUBTRACT);
            }

            entry.insert_text("-", &mut -1);
    
        }));

    button_multiply.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong cur_ops, @strong pre_ops, @strong entry =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);
    
            if num_counter.get() == 2 {
                // Set previous and current operation
                pre_ops.set(cur_ops.get());
                cur_ops.set(MULTIPLY);
                
                // Do operation
                operation(pre_ops.get(), &val1, val2.get());
    
                //decrease the num counter and reset num2
                num_counter.set(num_counter.get() - 1);
                val2.set(0);
            }
            else {
                cur_ops.set(MULTIPLY);
            }

            entry.insert_text("×", &mut -1);
        
        }));

    button_divide.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong cur_ops, @strong pre_ops, @strong entry =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);
        
            if num_counter.get() == 2 {
                // Set previous and current operation
                pre_ops.set(cur_ops.get());
                cur_ops.set(DIVIDE);
        
                // Do operation
                operation(pre_ops.get(), &val1, val2.get());
        
                // reset variables
                num_counter.set(num_counter.get() - 1);
                val2.set(0);
            }
            else {
                cur_ops.set(DIVIDE);
            }

            entry.insert_text("÷", &mut -1);
            
        }));
    
    button_equals.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong cur_ops, @strong entry =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);

            if num_counter.get() == 2 {
                let result = equation_result(cur_ops.get(), &val1, val2.get());

                entry.set_text(&result.to_string());

                pre_ops.set(EQUALS);

                // reset variables
                num_counter.set(0);
                val1.set(0);
                val2.set(0);
                cur_ops.set(NONE);
            }
        
        }));

    button_clear.connect_clicked(clone!(@strong entry =>
        move |_| {
            num_counter.set(0);
            val1.set(0);
            val2.set(0);
            cur_ops.set(NONE);
            entry.set_text("");
        }));

    let grid = Grid::new(); // gtk4::Grid

    /* Row 0 */
    GridExt::attach(&grid, &entry, 0, 0, 4, 1);

    /* Row 1 */
    GridExt::attach(&grid, &button_clear, 0, 1, 3, 1);
    GridExt::attach(&grid, &button_divide, 3, 1, 1, 1);

    /* Row 2 */
    GridExt::attach(&grid, &button_num7, 0, 2, 1, 1);
    GridExt::attach(&grid, &button_num8, 1, 2, 1, 1);
    GridExt::attach(&grid, &button_num9, 2, 2, 1, 1);
    GridExt::attach(&grid, &button_multiply, 3, 2, 1, 1);

    /* Row 3 */
    GridExt::attach(&grid, &button_num4, 0, 3, 1, 1);
    GridExt::attach(&grid, &button_num5, 1, 3, 1, 1);
    GridExt::attach(&grid, &button_num6, 2, 3, 1, 1);
    GridExt::attach(&grid, &button_minus, 3, 3, 1, 1);

    /* Row 4 */
    GridExt::attach(&grid, &button_num1, 0, 4, 1, 1);
    GridExt::attach(&grid, &button_num2, 1, 4, 1, 1);
    GridExt::attach(&grid, &button_num3, 2, 4, 1, 1);
    GridExt::attach(&grid, &button_plus, 3, 4, 1, 1);

    /* Row 5 */
    GridExt::attach(&grid, &button_num0, 0, 5, 3, 1);
    GridExt::attach(&grid, &button_equals, 3, 5, 1, 1);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Calculator")
        .default_width(250)
        .default_height(70)
        .build();

    // gtk4::prelude::GtkWindowExt
    window.set_child(Some(&grid));

    // Present the window
    window.present();
}