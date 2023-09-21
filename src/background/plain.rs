use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::color::Color;
use crate::color::average_color;

// - Set the palette parameter to use an average of the palette as the color
// - Set the color_hex to use a color
pub fn use_plain_image(tmp_dir: &std::path::PathBuf, palette: Option<&HashMap<String, Rc<RefCell<Color>>>>, color_hex: Option<&str>) {
    let color;

    if palette.is_some() {
        let colors = palette.unwrap().values().map(|col| col.borrow().clone()).clone().collect();
        color = average_color(colors);
    }
    else if color_hex.is_some() {
        color = Color::from_hex(color_hex.unwrap()).expect(format!("Could not parse plain image color '{}'", color_hex.unwrap()).as_str());
    }
    else {
        panic!("Use either a palette or a color hex for plain image generation");
    }

    super::create_bg(&tmp_dir, &color);
}
