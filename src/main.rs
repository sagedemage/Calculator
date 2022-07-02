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
use glib;
use glib_macros::clone;

const APP_ID: &str = "org.gtk_rs.GObjectSubclassing1";

struct Nums {
    num1: i32,
}

impl Nums {
    fn get(&self) -> i32 {
        return self.num1;
    }
}

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
    let num1: Rc<Cell<i64>> = Rc::new(Cell::new(0));
    let num2: Rc<Cell<i64>> = Rc::new(Cell::new(0));
    let ops = Rc::new(Cell::new('n'));
    let num_counter = Rc::new(Cell::new(0));

    //let rnums = Box::new(&mut nums);


    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_num1.connect_clicked(clone!(@weak num1, @weak num2, @weak num_counter, @weak button_show =>
        move |_| {
            if num_counter.get() == 0 {
                num1.set(num1.get() * 10 + 1);
            }
            if num_counter.get() == 0 {
                num2.set(num2.get() * 10 + 1);
            }
            button_show.set_label(&num1.get().to_string());

        }));

    button_num2.connect_clicked(clone!(@weak num1, @weak num2, @weak num_counter, @weak button_show =>
        move |_| {
            if num_counter.get() == 0 {
                num1.set(num1.get() * 10 + 2);
            }
            if num_counter.get() == 0 {
                num2.set(num2.get() * 10 + 2);
            }
            button_show.set_label(&num2.get().to_string());

        }));

    button_plus.connect_clicked(clone!(@weak num1, @weak num2, @weak ops, @weak button_show, @weak num_counter=>
        move |_| {
            if num_counter.get() == 0 {
                ops.set('+');
            }
            if num_counter.get() == 0 {
                ops.set('+');
            }
            button_show.set_label(&ops.get().to_string());

        }));
    button_minus.connect_clicked(clone!(@weak num1, @weak num2, @weak ops, @weak button_show, @weak num_counter =>
        move |_| {
            if num_counter.get() == 0 {
                ops.set('-');
            }
            if num_counter.get() == 0 {
                ops.set('-');
            }
            button_show.set_label(&ops.get().to_string());
    
        }));
    
    button_equals.connect_clicked(clone!(@weak ops, @weak button_show =>
        move |_| {
            if num_counter.get() == 0 {
                ops.set('=');
            }
            if num_counter.get() == 0 {
                ops.set('=');
            }
            button_show.set_label(&ops.get().to_string());
        
        }));

    button_clear.connect_clicked(clone!(@weak button_show =>
        move |_| {
            num1.set(0);
            num2.set(0);
            ops.set(' ');
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
