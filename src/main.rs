/*
 * gtk4
 *
 * Memory Management
 *
 * Got my Working Example from here:
 * https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_memory_management.html
 */

use std::rc::Rc;
use std::cell::Cell;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, Button, ApplicationWindow, Orientation, Entry};
use glib_macros::clone;

/* Operation constants for it to be used */
const ADD: char = 'a';
const SUBTRACT: char = 's';
const MULTIPLY: char = 'm';
const DIVIDE: char = 'd';
const EMPTY: char = 'e';

const APP_ID: &str = "org.gtk_rs.GObjectSubclassing1";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    let margin: i32 = 12;

    // Create two buttons
    let button_num1 = Button::builder()
        .label("1")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let button_num2 = Button::builder()
        .label("2")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let button_plus = Button::builder()
        .label("+")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let button_minus = Button::builder()
        .label("-")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let button_multiply = Button::builder()
        .label("*")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let button_divide = Button::builder()
        .label("/")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let button_equals = Button::builder()
        .label("=")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let button_clear = Button::builder()
        .label("clear")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let button_show = Button::builder()
        .label("None")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();

    //let entry = Entry::new()

    // A mutable integer
    let val1: Rc<Cell<i64>> = Rc::new(Cell::new(0));
    let val2: Rc<Cell<i64>> = Rc::new(Cell::new(0));
    let num_counter = Rc::new(Cell::new(0));
    let cur_ops = Rc::new(Cell::new(EMPTY));
    let pre_ops = Rc::new(Cell::new(EMPTY));

    // Connect callbacks
    // When a button is clicked, `number` should be changed
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
                operation(num_counter.get(), pre_ops.get(), &val1, val2.get());

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
                operation(num_counter.get(), pre_ops.get(), &val1, val2.get());

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
                operation(num_counter.get(), pre_ops.get(), &val1, val2.get());
    
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
                operation(num_counter.get(), pre_ops.get(), &val1, val2.get());
        
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

            let mut result = 0;

            if num_counter.get() == 2 {
                match cur_ops.get() {
                    ADD => {val1.set(val1.get() + val2.get()); result = val1.get();},
                    SUBTRACT => {val1.set(val1.get() - val2.get()); result = val1.get();},
                    MULTIPLY => {val1.set(val1.get() * val2.get()); result = val1.get();},
                    _=> ()
                }
                if cur_ops.get() == DIVIDE && val2.get() == 0 {
                    println!("Divide by zero error");
                }
                else if cur_ops.get() == DIVIDE && val2.get() != 0 {
                    val1.set(val1.get() / val2.get());
                    result = val1.get();
                }

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

fn set_value(num_counter: i32, val1: &Rc<Cell<i64>>, val2: &Rc<Cell<i64>>, num: i64) {
    if num_counter == 0 {
        val1.set(val1.get() * 10 + num);
    }
    if num_counter == 1 {
        val2.set(val2.get() * 10 + num);
    }
}

fn display_value(num_counter: i32, val1: i64, val2: i64) -> i64 {
    if num_counter == 0 {
        return val1;
    }
    if num_counter == 1 {
        return val2;
    }
    return 0;
}

fn operation(num_counter: i32, pre_ops: char, val1: &Rc<Cell<i64>>, val2: i64) {
    match pre_ops {
        ADD => val1.set(val1.get() + val2),
        SUBTRACT => val1.set(val1.get() - val2),
        MULTIPLY => val1.set(val1.get() * val2),
        _=> ()
    }
    if pre_ops == DIVIDE && val2 == 0 {
        println!("Divide by zero error");
    }
    else if pre_ops == DIVIDE && val2 != 0 {
        val1.set(val1.get() / val2);
    }
}


