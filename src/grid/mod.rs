/* Set up the grid */

use gtk4 as gtk;
use gtk::{Grid, Entry};
use gtk::prelude::*;
pub use crate::widgets::*;

pub fn set_grid(grid: &Grid, entry: &Entry, special_buttons: &SpecialButtons, 
                operator_buttons: &OperatorButtons, number_buttons: &NumberButtons) {
    /* Attach widgets to the Grid */
    // Row 0
    GridExt::attach(grid, entry, 0, 0, 4, 1);

    // Row 1
    GridExt::attach(grid, &special_buttons.clear, 0, 1, 3, 1);
    GridExt::attach(grid, &operator_buttons.divide, 3, 1, 1, 1);

    // Row 2
    GridExt::attach(grid, &number_buttons.num7, 0, 2, 1, 1);
    GridExt::attach(grid, &number_buttons.num8, 1, 2, 1, 1);
    GridExt::attach(grid, &number_buttons.num9, 2, 2, 1, 1);
    GridExt::attach(grid, &operator_buttons.multiply, 3, 2, 1, 1);

    // Row 3
    GridExt::attach(grid, &number_buttons.num4, 0, 3, 1, 1);
    GridExt::attach(grid, &number_buttons.num5, 1, 3, 1, 1);
    GridExt::attach(grid, &number_buttons.num6, 2, 3, 1, 1);
    GridExt::attach(grid, &operator_buttons.minus, 3, 3, 1, 1);

    // Row 4
    GridExt::attach(grid, &number_buttons.num1, 0, 4, 1, 1);
    GridExt::attach(grid, &number_buttons.num2, 1, 4, 1, 1);
    GridExt::attach(grid, &number_buttons.num3, 2, 4, 1, 1);
    GridExt::attach(grid, &operator_buttons.plus, 3, 4, 1, 1);

    // Row 5
    GridExt::attach(grid, &number_buttons.num0, 0, 5, 3, 1);
    GridExt::attach(grid, &special_buttons.equals, 3, 5, 1, 1);
}

