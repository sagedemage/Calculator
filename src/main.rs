/*
 * gtk4
 *
 * Memory Management
 *
 * Got my Working Example from here:
 * https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_memory_management.html
 */

 /* Main program */

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application};

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

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

