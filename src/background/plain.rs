use std::cell::RefCell;
use std::rc::Rc;

use crate::color::Color;

pub fn use_plain_image(tmp_dir: &std::path::PathBuf, color: Rc<RefCell<Color>>) {
    super::create_bg(&tmp_dir, color);
}
