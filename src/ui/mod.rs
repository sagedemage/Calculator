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
use crate::calculator::{self, Values, Operators, NumberTypeStatuses, Counters};
use crate::grid;
use crate::editable_entry_text;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION"); // get package version from Cargo
const LICENSE: &str = env!("CARGO_PKG_LICENSE"); // get license of the project
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION"); // get the description of the project
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS"); // get the authors of the project
const COPYRIGHT_FORMAT: &str = "\u{00A9} 2022 "; // copyright format
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
    //let num_counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    //let decimal_counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    
    let counters = Rc::new(RefCell::new(Counters::new()));

    // conditions
    let divide_zero: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let initiate_equals: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    let number_type_statuses = Rc::new(RefCell::new(NumberTypeStatuses::new()));

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
                .comments(DESCRIPTION)
                .copyright(format!("{}{}", COPYRIGHT_FORMAT, AUTHORS).as_str())
                .authors(vec![String::from(AUTHORS)])
                .license(LICENSE)
                .build();
            
            // Show the about dialog
            about_dialog.present();
        }));

    number_buttons.num0.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(&number_type_statuses.borrow().decimal, &counters, &vals, 0.0);
            entry.insert_text("0", &mut -1);
        }));
    number_buttons.num1.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            let value = calculator::set_sign_of_value(&number_type_statuses.borrow().negative, 1.0);
            calculator::set_value(&number_type_statuses.borrow().decimal, &counters, &vals, value);
            entry.insert_text("1", &mut -1);
        }));
    number_buttons.num2.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, 
        @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            let value = calculator::set_sign_of_value(&number_type_statuses.borrow().negative, 2.0);
            calculator::set_value(&number_type_statuses.borrow().decimal, &counters, &vals, value);
            entry.insert_text("2", &mut -1);
        }));
    number_buttons.num3.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, 
        @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            let value = calculator::set_sign_of_value(&number_type_statuses.borrow().negative, 3.0);
            calculator::set_value(&number_type_statuses.borrow().decimal, &counters, &vals, value);
            entry.insert_text("3", &mut -1);
        }));
    number_buttons.num4.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses,@strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            let value = calculator::set_sign_of_value(&number_type_statuses.borrow().negative, 4.0);
            calculator::set_value(&number_type_statuses.borrow().decimal, &counters, &vals, value);
            entry.insert_text("4", &mut -1);
        }));
    number_buttons.num5.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            let value = calculator::set_sign_of_value(&number_type_statuses.borrow().negative, 5.0);
            calculator::set_value(&number_type_statuses.borrow().decimal, &counters, &vals, value);
            entry.insert_text("5", &mut -1);
        }));
    number_buttons.num6.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            let value = calculator::set_sign_of_value(&number_type_statuses.borrow().negative, 6.0);
            calculator::set_value(&number_type_statuses.borrow().decimal, &counters, &vals, value);
            entry.insert_text("6", &mut -1);
        }));
    number_buttons.num7.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            let value = calculator::set_sign_of_value(&number_type_statuses.borrow().negative, 7.0);
            calculator::set_value(&number_type_statuses.borrow().decimal, &counters, &vals, value);
            entry.insert_text("7", &mut -1);
        }));
    number_buttons.num8.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            let value = calculator::set_sign_of_value(&number_type_statuses.borrow().negative, 8.0);
            calculator::set_value(&number_type_statuses.borrow().decimal, &counters, &vals, value);
            entry.insert_text("8", &mut -1);
        }));
    number_buttons.num9.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            let value = calculator::set_sign_of_value(&number_type_statuses.borrow().negative, 9.0);
            calculator::set_value(&number_type_statuses.borrow().decimal, &counters, &vals, value);
            entry.insert_text("9", &mut -1);
        }));

    special_buttons.period.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, @strong entry =>
        move |_| {
            if !number_type_statuses.borrow().decimal.get() {
                number_type_statuses.borrow().decimal.set(true);
                entry.insert_text(".", &mut -1);
            }
        }));

    special_buttons.negative.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong initiate_equals, @strong number_type_statuses, @strong entry =>
        move |_| {

            calculator::clear_entry_before_calculation(&initiate_equals, &entry);

            let entry_text: gtk::glib::GString = entry.text();
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    let last_entry_char = last_entry_char.unwrap();

                    if entry_text.ends_with('-') {
                        let new_text = editable_entry_text::remove_last_character_of_entry(&entry_text);
                        entry.set_text(new_text.as_str());
                        number_type_statuses.borrow().negative.set(false);
                    }
                    else if !entry_text.ends_with('-') && !last_entry_char.is_numeric() {
                        entry.insert_text("(-", &mut -1);
                        number_type_statuses.borrow().negative.set(true);
                    }
                },
                None => {
                    if !entry_text.ends_with('-') {
                        entry.insert_text("(-", &mut -1);
                        number_type_statuses.borrow().negative.set(true);
                    }
                }
            }

        }));
    
    operator_buttons.plus.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong divide_zero, @strong number_type_statuses, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if !entry.text().ends_with('\u{002B}') {
                        // Increase the counter
                        counters.borrow().number.set(counters.borrow().number.get() + 1);

                        // reset distinct numeral types
                        calculator::reset_distinct_numerical_types(
                            &number_type_statuses, 
                            &counters.borrow().decimal
                        );

                        // Do the operation
                        calculator::operation(ADD, &counters.borrow().number, &ops, &vals, &divide_zero);

                        // Insert the addition symbol to the entry
                        entry.insert_text("\u{002B}", &mut -1);
                    }
                },
                None => {}
            }
            
        }));

    operator_buttons.minus.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong divide_zero, @strong number_type_statuses, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if !entry.text().ends_with('\u{2212}') {
                        // Increase the counter
                        counters.borrow().number.set(counters.borrow().number.get() + 1);

                        // reset distinct numeral types
                        calculator::reset_distinct_numerical_types(
                            &number_type_statuses,
                            &counters.borrow().decimal
                        );

                        // Do the operation
                        calculator::operation(SUBTRACT, &counters.borrow().number, &ops, &vals, &divide_zero);

                        // Insert the subtraction symbol to the entry
                        entry.insert_text("\u{2212}", &mut -1);
                    }
                },
                None => {}
            }
        }));

    operator_buttons.multiply.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong divide_zero, @strong number_type_statuses, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if !entry.text().ends_with('\u{00D7}') {
                        // Increase the counter
                        counters.borrow().number.set(counters.borrow().number.get() + 1);

                        // reset distinct numeral types
                        calculator::reset_distinct_numerical_types(
                            &number_type_statuses,
                            &counters.borrow().decimal
                        );

                        // Do the operation
                        calculator::operation(MULTIPLY, &counters.borrow().number, &ops, &vals, &divide_zero);
    
                        // Insert the multiplication symbol to the entry
                        entry.insert_text("\u{00D7}", &mut -1);
                    }
                },
                None => {}
            }
        }));

    operator_buttons.divide.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong divide_zero, @strong number_type_statuses, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if !entry.text().ends_with('\u{00F7}') {
                        // Increase the counter
                        counters.borrow().number.set(counters.borrow().number.get() + 1);

                        // reset distinct numeral types
                        calculator::reset_distinct_numerical_types(
                            &number_type_statuses,
                            &counters.borrow().decimal
                        );

                        // Do the operation
                        calculator::operation(DIVIDE, &counters.borrow().number, &ops, &vals, &divide_zero);
        
                        // Insert the division symbol to the entry
                        entry.insert_text("\u{00F7}", &mut -1);
                    }
                },
                None => {}
            }
        }));
    
    special_buttons.equals.connect_clicked(clone!(@strong vals, @strong counters, @strong ops, 
        @strong divide_zero, @strong number_type_statuses, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if last_entry_char.unwrap().is_numeric() {
                        // Increase the counter
                        counters.borrow().number.set(counters.borrow().number.get() + 1);
                        
                        // Equality
                        calculator::equality(&counters.borrow().number, 
                                             &ops, &vals, &divide_zero, &entry, &initiate_equals);
                        
                        // reset variables
                        calculator::reset_to_default(&vals, &ops, &counters, &divide_zero, 
                                                     &number_type_statuses);
                    }
                },
                None => {}
            }
        }));

    special_buttons.clear.connect_clicked(clone!(@strong entry =>
        move |_| {
            // reset variables
            calculator::reset_to_default(&vals, &ops, &counters, &divide_zero, 
                                         &number_type_statuses);

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

