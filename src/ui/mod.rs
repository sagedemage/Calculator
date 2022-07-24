/* User Interface */

use std::rc::Rc;
use std::cell::{Cell, RefCell};

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Grid, HeaderBar, AboutDialog};

use glib_macros::clone;

pub use crate::symbol_names::*;
pub use crate::widget::*;
pub use crate::calculator::*;

pub fn build_ui(application: &Application) {
    // Header bar
    let header_bar = HeaderBar::new();

    // Create Grid
    let grid = Grid::new();

    let about_button = create_button("About");
    //about_button.set_icon_name("view-list");
    
    let authors = vec![String::from("Salmaan Saeed")];
    
    let about_dialog = AboutDialog::builder()
        .version("0.1.0")
        .comments("GTK4 Calculator App written in Rust")
        .copyright("© 2022 Salmaan Saeed")
        .authors(authors)
        .license("The 3-Clause BSD License")
        .build();
   
    // Create number buttons
    let number_buttons = NumberButtons{
        num0: create_button("0"),
        num1: create_button("1"),
        num2: create_button("2"),
        num3: create_button("3"),
        num4: create_button("4"),
        num5: create_button("5"),
        num6: create_button("6"),
        num7: create_button("7"),
        num8: create_button("8"),
        num9: create_button("9"),
    };

    // create operation and misc widgets
    let plus_button = create_button("+");
    let minus_button = create_button("-");
    let multiply_button = create_button("\u{00D7}");
    let divide_button = create_button("\u{00F7}");
    let equals_button = create_button("=");
    let clear_button = create_button("clear");
    let entry = create_entry();

    // add css class for the button
    clear_button.add_css_class("clear");
    equals_button.add_css_class("equals");

    // A mutable values
    let vals = Rc::new(RefCell::new(
            Values {
                num1: Rc::new(Cell::new(0.0)),
                num2: Rc::new(Cell::new(0.0)),
            }
        ));

    let ops = Rc::new(RefCell::new(
            Operators {
                current: Rc::new(Cell::new(NONE)),
                previous: Rc::new(Cell::new(NONE)),
            }
        ));

    let num_counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let divide_zero: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    // Connect callbacks
    about_button.connect_clicked(clone!(@strong about_dialog =>
        move |_| {
            about_dialog.show();
        }
    ));

    number_buttons.num0.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            clear_entry_before_calculation(&ops.borrow().previous, &entry);
            set_value(num_counter.get(), &vals, 0.0);
            entry.insert_text("0", &mut -1);
        }));
    number_buttons.num1.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            clear_entry_before_calculation(&ops.borrow().previous, &entry);
            set_value(num_counter.get(), &vals, 1.0);
            entry.insert_text("1", &mut -1);
        }));
    number_buttons.num2.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            clear_entry_before_calculation(&ops.borrow().previous, &entry);
            set_value(num_counter.get(), &vals, 2.0);
            entry.insert_text("2", &mut -1);
        }));
    number_buttons.num3.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            clear_entry_before_calculation(&ops.borrow().previous, &entry);
            set_value(num_counter.get(), &vals, 3.0);
            entry.insert_text("3", &mut -1);
        }));
    number_buttons.num4.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            clear_entry_before_calculation(&ops.borrow().previous, &entry);
            set_value(num_counter.get(), &vals, 4.0);
            entry.insert_text("4", &mut -1);
        }));
    number_buttons.num5.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            clear_entry_before_calculation(&ops.borrow().previous, &entry);
            set_value(num_counter.get(), &vals, 5.0);
            entry.insert_text("5", &mut -1);
        }));
    number_buttons.num6.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            clear_entry_before_calculation(&ops.borrow().previous, &entry);
            set_value(num_counter.get(), &vals, 6.0);
            entry.insert_text("6", &mut -1);
        }));
    number_buttons.num7.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            clear_entry_before_calculation(&ops.borrow().previous, &entry);
            set_value(num_counter.get(), &vals, 7.0);
            entry.insert_text("7", &mut -1);
        }));
    number_buttons.num8.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            clear_entry_before_calculation(&ops.borrow().previous, &entry);
            set_value(num_counter.get(), &vals, 8.0);
            entry.insert_text("8", &mut -1);
        }));
    number_buttons.num9.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            clear_entry_before_calculation(&ops.borrow().previous, &entry);
            set_value(num_counter.get(), &vals, 9.0);
            entry.insert_text("9", &mut -1);
        }));
    
    plus_button.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);

            if num_counter.get() == 2 {
                // Set previous and current operation
                ops.borrow().previous.set(ops.borrow().current.get());
                ops.borrow().current.set(ADD);

                // Do operation
                operation(ops.borrow().previous.get(), &vals.borrow().num1, vals.borrow().num2.get());

                // Decrease the num counter and reset num2
                num_counter.set(num_counter.get() - 1);
                vals.borrow().num2.set(0.0);
            }
            else {
                ops.borrow().current.set(ADD);
            }

            entry.insert_text("+", &mut -1);

        }));

    minus_button.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);

            if num_counter.get() == 2 {
                // Set previous and current operation
                ops.borrow().previous.set(ops.borrow().current.get());
                ops.borrow().current.set(SUBTRACT);

                // Do operation
                operation(ops.borrow().previous.get(), &vals.borrow().num1, vals.borrow().num2.get());

                //decrease the num counter and reset num2
                num_counter.set(num_counter.get() - 1);
                vals.borrow().num2.set(0.0);
            }
            else {
                ops.borrow().current.set(SUBTRACT);
            }

            entry.insert_text("-", &mut -1);
    
        }));

    multiply_button.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong entry =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);
    
            if num_counter.get() == 2 {
                // Set previous and current operation
                ops.borrow().previous.set(ops.borrow().current.get());
                ops.borrow().current.set(MULTIPLY);
                
                // Do operation
                operation(ops.borrow().previous.get(), &vals.borrow().num1, vals.borrow().num2.get());
    
                //decrease the num counter and reset num2
                num_counter.set(num_counter.get() - 1);
                vals.borrow().num2.set(0.0);
            }
            else {
                ops.borrow().current.set(MULTIPLY);
            }

            entry.insert_text("\u{00D7}", &mut -1);
        
        }));

    divide_button.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);
        
            if num_counter.get() == 2 {
                // Set previous and current operation
                ops.borrow().previous.set(ops.borrow().current.get());
                ops.borrow().current.set(DIVIDE);
        
                // Do operation
                operation(ops.borrow().previous.get(), &vals.borrow().num1, vals.borrow().num2.get());

                // Check divison by zero
                check_divison_by_zero(ops.borrow().previous.get(), vals.borrow().num2.get(), &divide_zero);
        
                // reset variables
                num_counter.set(num_counter.get() - 1);
                vals.borrow().num2.set(0.0);
            }
            else {
                ops.borrow().current.set(DIVIDE);
            }

            entry.insert_text("\u{00F7}", &mut -1);
            
        }));
    
    equals_button.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);

            if num_counter.get() == 2 {
                let result = equation_result(
                    ops.borrow().current.get(),
                    &vals,
                    &divide_zero,
                    );

                entry.set_text(&result);

                ops.borrow().previous.set(EQUALS);

                // reset variables
                num_counter.set(0);
                vals.borrow().num1.set(0.0);
                vals.borrow().num2.set(0.0);
                ops.borrow().current.set(NONE);
            }
        
        }));

    clear_button.connect_clicked(clone!(@strong entry =>
        move |_| {
            num_counter.set(0);
            vals.borrow().num1.set(0.0);
            vals.borrow().num2.set(0.0);
            ops.borrow().current.set(NONE);
            entry.set_text("");
        }));

    // Add search button to the header bar
    header_bar.pack_end(&about_button);

    /* Row 0 */
    GridExt::attach(&grid, &entry, 0, 0, 4, 1);

    /* Row 1 */
    GridExt::attach(&grid, &clear_button, 0, 1, 3, 1);
    GridExt::attach(&grid, &divide_button, 3, 1, 1, 1);

    /* Row 2 */
    GridExt::attach(&grid, &number_buttons.num7, 0, 2, 1, 1);
    GridExt::attach(&grid, &number_buttons.num8, 1, 2, 1, 1);
    GridExt::attach(&grid, &number_buttons.num9, 2, 2, 1, 1);
    GridExt::attach(&grid, &multiply_button, 3, 2, 1, 1);

    /* Row 3 */
    GridExt::attach(&grid, &number_buttons.num4, 0, 3, 1, 1);
    GridExt::attach(&grid, &number_buttons.num5, 1, 3, 1, 1);
    GridExt::attach(&grid, &number_buttons.num6, 2, 3, 1, 1);
    GridExt::attach(&grid, &minus_button, 3, 3, 1, 1);

    /* Row 4 */
    GridExt::attach(&grid, &number_buttons.num1, 0, 4, 1, 1);
    GridExt::attach(&grid, &number_buttons.num2, 1, 4, 1, 1);
    GridExt::attach(&grid, &number_buttons.num3, 2, 4, 1, 1);
    GridExt::attach(&grid, &plus_button, 3, 4, 1, 1);

    /* Row 5 */
    GridExt::attach(&grid, &number_buttons.num0, 0, 5, 3, 1);
    GridExt::attach(&grid, &equals_button, 3, 5, 1, 1);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Calculator")
        .default_width(250)
        .default_height(70)
        .build();

    
    window.set_titlebar(Some(&header_bar));

    // gtk4::prelude::GtkWindowExt
    window.set_child(Some(&grid));

    // Present the window
    window.present();
}
