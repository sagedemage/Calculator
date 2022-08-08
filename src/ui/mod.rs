/* User Interface */

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::path::Path;
use std::option::Option;

use gtk::{Application, ApplicationWindow, Builder, PopoverMenu, 
    Grid, HeaderBar, AboutDialog, MenuButton, Picture};
use gtk::prelude::*;
use gio::{Menu, SimpleAction};
use gdk_pixbuf::Pixbuf;

use glib_macros::clone;

use crate::operator_symbols::*;
use crate::widgets::{self, NumberButtons, OperatorButtons, SpecialButtons};
use crate::calculator::{self, Values, Operators};
use crate::grid;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION"); // get package version from Cargo
const LOGO_PATH: &str = "src/resources/images/logo.png"; // path to the logo
const MENU_UI_PATH: &str = "src/resources/ui/menu.ui";

pub fn build_ui(application: &Application) {
    /* build ui of the application */
    // Create Window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Calculator")
        .default_width(250)
        .default_height(70)
        .build();

    // Load menu ui file
    let menu_builder = Builder::from_file(MENU_UI_PATH);

    // Get Menu object
    let menu_object: Option<Menu> = menu_builder.object("menu");

    // Get file path of the logo image
    let image_logo_path = Path::new(LOGO_PATH);

    // Create pixbuf from file path of the app logo image
    let image_logo_pixbuf = Pixbuf::from_file(&image_logo_path);

    // Get the Pixbuf value of file_pixbuf if the file exists
    let image_logo_pixbuf = image_logo_pixbuf.expect("File Not Found: app logo image not found!");

    // Create picture
    let app_logo = Picture::for_pixbuf(&image_logo_pixbuf);

    // Create header bar
    let header_bar = HeaderBar::new();

    // Create menu button
    let menu_button = MenuButton::new();
    menu_button.set_icon_name("view-list"); // set menu button icon

    // Simple Action for showing about dialog
    let about_action = SimpleAction::new("about", None);

    // Get Menu
    let menu = menu_object.unwrap();

    // Create Popover Menu from menu
    let popover_menu = PopoverMenu::from_model(Some(&menu));

    // Create grid
    let grid = Grid::new();
   
    // Create entry
    let entry = widgets::create_entry();

    // Create number buttons
    let number_buttons = NumberButtons::new();

    // Create operator buttons
    let operator_buttons = OperatorButtons::new();

    // Create special buttons
    let special_buttons = SpecialButtons::new();

    // add css class for the numbered buttons
    number_buttons.num0.set_widget_name("number_buttons");
    number_buttons.num1.set_widget_name("number_buttons");
    number_buttons.num2.set_widget_name("number_buttons");
    number_buttons.num3.set_widget_name("number_buttons");
    number_buttons.num4.set_widget_name("number_buttons");
    number_buttons.num5.set_widget_name("number_buttons");
    number_buttons.num6.set_widget_name("number_buttons");
    number_buttons.num7.set_widget_name("number_buttons");
    number_buttons.num8.set_widget_name("number_buttons");
    number_buttons.num9.set_widget_name("number_buttons");

    // add css class for the operator buttons
    operator_buttons.plus.set_widget_name("operator_buttons");
    operator_buttons.minus.set_widget_name("operator_buttons");
    operator_buttons.multiply.set_widget_name("operator_buttons");
    operator_buttons.divide.set_widget_name("operator_buttons");

    // add css class for the special buttons
    special_buttons.clear.set_widget_name("clear-button");
    special_buttons.equals.set_widget_name("equals-button");
    special_buttons.period.set_widget_name("period-button");
    special_buttons.negative.set_widget_name("negative-button");

    /* Mutable values */
    // values
    let vals = Rc::new(RefCell::new(Values::new()));
    // operators
    let ops = Rc::new(RefCell::new(Operators::new()));
    
    // counters
    let num_counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let decimal_counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));

    //conditions
    let decimal_value: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let divide_zero: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let initiate_equals: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    /* Connect callbacks */
    about_action.connect_activate(clone!(@strong window =>
        move |_, _| {
            // create about dialog here
            // About Dialog 
            let about_dialog = AboutDialog::builder()
                .transient_for(&window) // the temporary parent of the window 
                .modal(true) // freezes the rest of the app from user input
                .logo(&app_logo.paintable().unwrap())
                .version(APP_VERSION)
                .comments("GTK4 Calculator App written in Rust")
                .copyright("© 2022 Salmaan Saeed")
                .authors(vec![String::from("Salmaan Saeed")])
                .license("The 3-Clause BSD License")
                .build();
            
            // Show the about dialog
            about_dialog.present();
        }));

    number_buttons.num0.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&decimal_value, num_counter.get(),
                &decimal_counter, &vals, 0.0);
            entry.insert_text("0", &mut -1);
        }));
    number_buttons.num1.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&decimal_value, num_counter.get(),
                &decimal_counter, &vals, 1.0);
            entry.insert_text("1", &mut -1);
        }));
    number_buttons.num2.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&decimal_value, num_counter.get(),
                &decimal_counter, &vals, 2.0);
            entry.insert_text("2", &mut -1);
        }));
    number_buttons.num3.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&decimal_value, num_counter.get(),
                &decimal_counter, &vals, 3.0);
            entry.insert_text("3", &mut -1);
        }));
    number_buttons.num4.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&decimal_value, num_counter.get(),
                &decimal_counter, &vals, 4.0);
            entry.insert_text("4", &mut -1);
        }));
    number_buttons.num5.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&decimal_value, num_counter.get(),
                &decimal_counter, &vals, 5.0);
            entry.insert_text("5", &mut -1);
        }));
    number_buttons.num6.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&decimal_value, num_counter.get(),
                &decimal_counter, &vals, 6.0);
            entry.insert_text("6", &mut -1);
        }));
    number_buttons.num7.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&decimal_value, num_counter.get(),
                &decimal_counter, &vals, 7.0);
            entry.insert_text("7", &mut -1);
        }));
    number_buttons.num8.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&decimal_value, num_counter.get(),
                &decimal_counter, &vals, 8.0);
            entry.insert_text("8", &mut -1);
        }));
    number_buttons.num9.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&decimal_value, num_counter.get(),
                &decimal_counter, &vals, 9.0);
            entry.insert_text("9", &mut -1);
        }));

    special_buttons.period.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            if !decimal_value.get() {
                decimal_value.set(true);
                entry.insert_text(".", &mut -1);
            }
        }));

    special_buttons.negative.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            let entry_length: i32 = entry.text().len() as i32;
            let last_entry_char = entry.text().chars().last();

            /*if entry.text().ends_with('-') {       
                entry.delete_text(entry_length-1, -1);
            }
            else if !entry.text().ends_with('-') && !last_entry_char.unwrap().is_numeric() {
                entry.insert_text("-", &mut -1);
            }*/

            match last_entry_char {
                Some(_) => {
                    if entry.text().ends_with('-') {       
                        entry.delete_text(entry_length-1, -1);
                    }
                    else if !entry.text().ends_with('-') && !last_entry_char.unwrap().is_numeric() {
                        entry.insert_text("-", &mut -1);
                    }
                },
                None => {
                    if entry.text().ends_with('-') {       
                        entry.delete_text(entry_length-1, -1);
                    }
                    else if !entry.text().ends_with('-') {
                        entry.insert_text("-", &mut -1);
                    }
                }
            }

        }));
    
    operator_buttons.plus.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if !entry.text().ends_with('+') {
                        // Increase the counter
                        num_counter.set(num_counter.get() + 1);

                        // Do the operation
                        calculator::operation(ADD, &num_counter, &decimal_counter, &ops, &vals, &decimal_value, 
                                              &divide_zero);

                        // Insert the addition symbol to the entry
                        entry.insert_text("+", &mut -1);
                    }
                },
                None => {}
            }
            
        }));

    operator_buttons.minus.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if !entry.text().ends_with('-') {
                        // Increase the counter
                        num_counter.set(num_counter.get() + 1);

                        // Do the operation
                        calculator::operation(SUBTRACT, &num_counter, &decimal_counter, &ops, &vals, &decimal_value, 
                                              &divide_zero);

                        // Insert the subtraction symbol to the entry
                        entry.insert_text("-", &mut -1);
                    }
                },
                None => {}
            }
        }));

    operator_buttons.multiply.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if !entry.text().ends_with('\u{00D7}') {
                        // Increase the counter
                        num_counter.set(num_counter.get() + 1);

                        // Do the operation
                        calculator::operation(MULTIPLY, &num_counter, &decimal_counter, &ops, &vals, &decimal_value, 
                                              &divide_zero);
    
                        // Insert the multiplication symbol to the entry
                        entry.insert_text("\u{00D7}", &mut -1);
                    }
                },
                None => {}
            }
        }));

    operator_buttons.divide.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if !entry.text().ends_with('\u{00F7}') {
                        // Increase the counter
                        num_counter.set(num_counter.get() + 1);

                        // Do the operation
                        calculator::operation(DIVIDE, &num_counter, &decimal_counter, &ops, &vals, &decimal_value, 
                                              &divide_zero);
        
                        // Insert the division symbol to the entry
                        entry.insert_text("\u{00F7}", &mut -1);
                    }
                },
                None => {}
            }
        }));
    
    special_buttons.equals.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong decimal_value, @strong decimal_counter, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if last_entry_char.unwrap().is_numeric() {
                        // Increase the counter
                        num_counter.set(num_counter.get() + 1);
                        
                        // Equality
                        calculator::equality(&num_counter, &ops, &vals, &divide_zero, &entry, &initiate_equals,
                                             &decimal_counter, &decimal_value);
                    }
                },
                None => {}
            }
        }));

    special_buttons.clear.connect_clicked(clone!(@strong entry =>
        move |_| {
            // reset variables
            calculator::reset_variables(&vals, &ops, &num_counter, &decimal_counter,
                                        &divide_zero, &decimal_value);

            // Clear entry text
            entry.set_text("");
        }));

    // Set popover for menu button
    menu_button.set_popover(Some(&popover_menu));

    // Add about button to the header bar
    header_bar.pack_end(&menu_button);

    /* Attach widgets to the Grid */
    grid::set_grid(&grid, &entry, &special_buttons, &operator_buttons, &number_buttons);

    // Add about action to the application
    application.add_action(&about_action);

    // Set the window title bar
    window.set_titlebar(Some(&header_bar));

    // set grid as a child of window
    window.set_child(Some(&grid));

    // Present the window
    window.present();
}

