/* User Interface */

use std::rc::Rc;
use std::cell::{Cell, RefCell};

use gtk::{Application, ApplicationWindow, Box, Orientation, GestureClick,
Popover, Grid, HeaderBar, AboutDialog, MenuButton, Label};
use gtk::prelude::*;

use glib_macros::clone;

pub use crate::operator_symbols::*;
pub use crate::widgets::*;
pub use crate::calculator::*;
pub use crate::grid::set_grid;

// Get package version from Cargo
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn build_ui(application: &Application) {
    /* build ui of the application */

    // Header bar
    let header_bar = HeaderBar::new();

    // Menu Button
    let menu_button = MenuButton::new();
    menu_button.set_icon_name("view-list"); // set menu button icon

    // Vertical Box
    let vbox = Box::new(Orientation::Vertical, 0);

    // Popover for labels
    let popover = Popover::new();

    // About label
    let about_label = Label::new(Some("About"));

    // About gesture
    let about_gesture = GestureClick::new();
    
    // Grid
    let grid = Grid::new();
   
    // entry
    let entry = create_entry();

    // number buttons
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

    // operator buttons
    let operator_buttons = OperatorButtons{
        plus: create_button("+"),
        minus: create_button("-"),
        multiply: create_button("\u{00D7}"),
        divide: create_button("\u{00F7}"),
    };

    // special buttons
    let special_buttons = SpecialButtons{
        clear: create_button("clear"),
        equals: create_button("="),
    };

    // add css class for the special button
    special_buttons.clear.add_css_class("clear");
    special_buttons.equals.add_css_class("equals");

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
    let initiate_equals: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    /* Connect callbacks */
    about_gesture.connect_pressed(move |about_gesture, _, _, _| {
        about_gesture.set_state(gtk::EventSequenceState::Claimed);

        // create about dialog here
        // About Dialog 
        let about_dialog = AboutDialog::builder()
            .version(VERSION)
            .comments("GTK4 Calculator App written in Rust")
            .copyright("© 2022 Salmaan Saeed")
            .authors(vec![String::from("Salmaan Saeed")])
            .license("The 3-Clause BSD License")
            .build();

        about_dialog.show();
    });

    number_buttons.num0.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            clear_entry_before_calculation(&initiate_equals, &entry);
            set_value(num_counter.get(), &vals, 0.0);
            entry.insert_text("0", &mut -1);
        }));
    number_buttons.num1.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            clear_entry_before_calculation(&initiate_equals, &entry);
            set_value(num_counter.get(), &vals, 1.0);
            entry.insert_text("1", &mut -1);
        }));
    number_buttons.num2.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            clear_entry_before_calculation(&initiate_equals, &entry);
            set_value(num_counter.get(), &vals, 2.0);
            entry.insert_text("2", &mut -1);
        }));
    number_buttons.num3.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            clear_entry_before_calculation(&initiate_equals, &entry);
            set_value(num_counter.get(), &vals, 3.0);
            entry.insert_text("3", &mut -1);
        }));
    number_buttons.num4.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            clear_entry_before_calculation(&initiate_equals, &entry);
            set_value(num_counter.get(), &vals, 4.0);
            entry.insert_text("4", &mut -1);
        }));
    number_buttons.num5.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            clear_entry_before_calculation(&initiate_equals, &entry);
            set_value(num_counter.get(), &vals, 5.0);
            entry.insert_text("5", &mut -1);
        }));
    number_buttons.num6.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            clear_entry_before_calculation(&initiate_equals, &entry);
            set_value(num_counter.get(), &vals, 6.0);
            entry.insert_text("6", &mut -1);
        }));
    number_buttons.num7.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            clear_entry_before_calculation(&initiate_equals, &entry);
            set_value(num_counter.get(), &vals, 7.0);
            entry.insert_text("7", &mut -1);
        }));
    number_buttons.num8.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            clear_entry_before_calculation(&initiate_equals, &entry);
            set_value(num_counter.get(), &vals, 8.0);
            entry.insert_text("8", &mut -1);
        }));
    number_buttons.num9.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            clear_entry_before_calculation(&initiate_equals, &entry);
            set_value(num_counter.get(), &vals, 9.0);
            entry.insert_text("9", &mut -1);
        }));
    
    operator_buttons.plus.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            if entry.text().chars().last().unwrap() != '+' {
                // Increase the counter
                num_counter.set(num_counter.get() + 1);

                // Do the operation
                operation(ADD, &num_counter, &ops, &vals, &divide_zero);

                // Insert the addition symbol to the entry
                entry.insert_text("+", &mut -1);
            }
        }));

    operator_buttons.minus.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            if entry.text().chars().last().unwrap() != '-' {
                // Increase the counter
                num_counter.set(num_counter.get() + 1);

                // Do the operation
                operation(SUBTRACT, &num_counter, &ops, &vals, &divide_zero);

                // Insert the subtraction symbol to the entry
                entry.insert_text("-", &mut -1);
            }
        }));

    operator_buttons.multiply.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            if entry.text().chars().last().unwrap() != '\u{00D7}' {
                // Increase the counter
                num_counter.set(num_counter.get() + 1);

                // Do the operation
                operation(MULTIPLY, &num_counter, &ops, &vals, &divide_zero);
    
                // Insert the multiplication symbol to the entry
                entry.insert_text("\u{00D7}", &mut -1);
            }
        }));

    operator_buttons.divide.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            if entry.text().chars().last().unwrap() != '\u{00F7}' {
            // Increase the counter
            num_counter.set(num_counter.get() + 1);

            // Do the operation
            operation(DIVIDE, &num_counter, &ops, &vals, &divide_zero);
        
            // Insert the division symbol to the entry
            entry.insert_text("\u{00F7}", &mut -1);
            }
        }));
    
    special_buttons.equals.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if last_entry_char.unwrap().is_numeric() {
                        // Increase the counter
                        num_counter.set(num_counter.get() + 1);

                        if num_counter.get() == 2 {
                            let result = equation_result(
                                &ops,
                                &vals,
                                &divide_zero,
                                );

                            // Show the result on the entry
                            entry.set_text(&result);

                            // Notify the progam the user initated the equals button
                            initiate_equals.set(true);

                            // reset variables
                            reset_variables(&vals, &ops, &num_counter, &divide_zero);
                        }        
                    }
                },
                None => {}
            }
        }));

    special_buttons.clear.connect_clicked(clone!(@strong entry =>
        move |_| {
            // reset variables
            reset_variables(&vals, &ops, &num_counter, &divide_zero);

            // Clear entry text
            entry.set_text("");
        }));

    // Add label to box
    about_label.add_controller(&about_gesture);

    // Add about label to vertical box
    vbox.append(&about_label);
    
    // Set vertical box as the child of popover
    popover.set_child(Some(&vbox));

    // Set popover for menu button
    menu_button.set_popover(Some(&popover));

    // Add about button to the header bar
    header_bar.pack_end(&menu_button);

    /* Attach widgets to the Grid */
    set_grid(&grid, &entry, &special_buttons, &operator_buttons, &number_buttons);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Calculator")
        .default_width(250)
        .default_height(70)
        .build();

    // Set the window title bar
    window.set_titlebar(Some(&header_bar));

    // set grid as a child of window
    window.set_child(Some(&grid));

    // Present the window
    window.present();
}

