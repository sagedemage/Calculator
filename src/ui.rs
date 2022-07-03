/* User Interface */

use std::rc::Rc;
use std::cell::Cell;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, Button, ApplicationWindow, Orientation, Entry};
use glib_macros::clone;

pub use crate::defs::*;
pub use crate::button::*;
pub use crate::calculator::*;

pub fn build_ui(application: &Application) {
    // Create buttons
    let button_num0 = create_button("0");
    let button_num1 = create_button("1");
    let button_num2 = create_button("2");
    let button_plus = create_button("+");
    let button_minus = create_button("-");
    let button_multiply = create_button("*");
    let button_divide = create_button("/");
    let button_equals = create_button("=");
    let button_clear = create_button("clear");
    let button_show = create_button("none");

    //let entry = Entry::new()

    // A mutable integer
    let val1: Rc<Cell<i64>> = Rc::new(Cell::new(0));
    let val2: Rc<Cell<i64>> = Rc::new(Cell::new(0));
    let num_counter = Rc::new(Cell::new(0));
    let cur_ops = Rc::new(Cell::new(EMPTY));
    let pre_ops = Rc::new(Cell::new(EMPTY));

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_num0.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong button_show =>
        move |_| {
            set_value(num_counter.get(), &val1, &val2, 0);
            let val = display_value(num_counter.get(), val1.get(), val2.get());
            button_show.set_label(&val.to_string());
        }));

    button_num1.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong button_show =>
        move |_| {
            set_value(num_counter.get(), &val1, &val2, 1);
            let val = display_value(num_counter.get(), val1.get(), val2.get());
            button_show.set_label(&val.to_string());
        }));

    button_num2.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong button_show =>
        move |_| {
            set_value(num_counter.get(), &val1, &val2, 2);
            let val = display_value(num_counter.get(), val1.get(), val2.get());
            button_show.set_label(&val.to_string());
        }));

    button_plus.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong cur_ops, @strong pre_ops, @strong button_show =>
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

            button_show.set_label("+");

        }));

    button_minus.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong cur_ops, @strong pre_ops, @strong button_show =>
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

            button_show.set_label("-");
    
        }));

    button_multiply.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong cur_ops, @strong pre_ops, @strong button_show =>
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

            button_show.set_label("*");
        
        }));

    button_divide.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong cur_ops, @strong pre_ops, @strong button_show =>
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

            button_show.set_label("/");
            
        }));
    
    button_equals.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, @strong cur_ops, @strong button_show =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);

            let mut result = String::from("");

            if num_counter.get() == 2 {
                result = equation_result(cur_ops.get(), &val1, val2.get());

                button_show.set_label(&result.to_string());

                // reset variables
                num_counter.set(0);
                val1.set(0);
                val2.set(0);
                cur_ops.set(EMPTY);
            }
        
        }));

    button_clear.connect_clicked(clone!(@strong button_show =>
        move |_| {
            num_counter.set(0);
            val1.set(0);
            val2.set(0);
            cur_ops.set(EMPTY);
            button_show.set_label("");
        }));

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&button_show);
    gtk_box.append(&button_num0);
    gtk_box.append(&button_num1);
    gtk_box.append(&button_num2);
    gtk_box.append(&button_plus);
    gtk_box.append(&button_minus);
    gtk_box.append(&button_multiply);
    gtk_box.append(&button_divide);
    gtk_box.append(&button_equals);
    gtk_box.append(&button_clear);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}