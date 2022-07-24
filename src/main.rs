/*
 * gtk4
 *
 * Memory Management
 *
 * Got my Working Example from here:
 * https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_memory_management.html
 */

 /* Main program */

pub use gtk4 as gtk;
pub use gtk::Application;

pub use gdk4::Display;
pub use gtk::{CssProvider, StyleContext};

mod symbol_names;
mod widget;
mod calculator;
mod ui;
mod css;

pub use crate::ui::*;
pub use crate::css::load_css_file;

const APP_ID: &str = "org.gtk_rs.GObjectSubclassing1";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css_file());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
