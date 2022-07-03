/*
 * gtk4
 *
 * Memory Management
 *
 * Got my Working Example from here:
 * https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_memory_management.html
 */

 /* Main program */

use gdk4::Display;
use gtk4 as gtk;
use gtk::{Application, CssProvider, StyleContext};

mod defs;
mod button;
mod calculator;
mod ui;

pub use crate::defs::*;
pub use crate::button::*;
pub use crate::calculator::*;
pub use crate::ui::*;

const APP_ID: &str = "org.gtk_rs.GObjectSubclassing1";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    //IsA

    // Add the provider to the default screen
    //IsA<gtk4::gdk4::Display>` is not satisfied
    //the following other types implement trait `IsA<T>`:
    //<gdk::Display as IsA<gdk::Display>>
    //<gdk::Display as IsA<gdk::glib::Object>>rustcE0277
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

