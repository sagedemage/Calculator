/* User Interface */

use std::rc::Rc;
use std::cell::{Cell, RefCell};

use gtk4 as gtk;
use gtk::{Application, ApplicationWindow, Grid, HeaderBar};
//use gtk::ToggleButton;
use glib_macros::clone;

pub use crate::defs::*;
pub use crate::widget::*;
pub use crate::calculator::*;

pub fn build_ui(application: &Application) {
    // Header bar
    let header_bar = HeaderBar::new();

    // Search Button (work in progress)
    //let search_button = ToggleButton::new();

    //search_button.set_icon_name("system-search-symbolic");

    // Add search button to the header bar
    //header_bar.pack_end(&search_button);
    
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

    // create operation and miscs widgets
    let button_plus = create_button("+");
    let button_minus = create_button("-");
    let button_multiply = create_button("\u{00D7}");
    let button_divide = create_button("\u{00F7}");
    let button_equals = create_button("=");
    let button_clear = create_button("clear");
    let entry = create_entry();

    button_clear.add_css_class("destructive-action");
    button_equals.add_css_class("suggested-action");

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
    // When a button is clicked, `number` should be changed
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
    
    button_plus.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
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

    button_minus.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
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

    button_multiply.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
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

    button_divide.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
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
    
    button_equals.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
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

    button_clear.connect_clicked(clone!(@strong entry =>
        move |_| {
            num_counter.set(0);
            vals.borrow().num1.set(0.0);
            vals.borrow().num2.set(0.0);
            ops.borrow().current.set(NONE);
            entry.set_text("");
        }));

    let grid = Grid::new(); // gtk4::Grid

    /* Row 0 */
    GridExt::attach(&grid, &entry, 0, 0, 4, 1);

    /* Row 1 */
    GridExt::attach(&grid, &button_clear, 0, 1, 3, 1);
    GridExt::attach(&grid, &button_divide, 3, 1, 1, 1);

    /* Row 2 */
    GridExt::attach(&grid, &number_buttons.num7, 0, 2, 1, 1);
    GridExt::attach(&grid, &number_buttons.num8, 1, 2, 1, 1);
    GridExt::attach(&grid, &number_buttons.num9, 2, 2, 1, 1);
    GridExt::attach(&grid, &button_multiply, 3, 2, 1, 1);

    /* Row 3 */
    GridExt::attach(&grid, &number_buttons.num4, 0, 3, 1, 1);
    GridExt::attach(&grid, &number_buttons.num5, 1, 3, 1, 1);
    GridExt::attach(&grid, &number_buttons.num6, 2, 3, 1, 1);
    GridExt::attach(&grid, &button_minus, 3, 3, 1, 1);

    /* Row 4 */
    GridExt::attach(&grid, &number_buttons.num1, 0, 4, 1, 1);
    GridExt::attach(&grid, &number_buttons.num2, 1, 4, 1, 1);
    GridExt::attach(&grid, &number_buttons.num3, 2, 4, 1, 1);
    GridExt::attach(&grid, &button_plus, 3, 4, 1, 1);

    /* Row 5 */
    GridExt::attach(&grid, &number_buttons.num0, 0, 5, 3, 1);
    GridExt::attach(&grid, &button_equals, 3, 5, 1, 1);

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
