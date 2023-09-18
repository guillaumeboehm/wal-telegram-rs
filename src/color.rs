use std::cell::RefCell;
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::fmt::{self, format};
use std::rc::Rc;

// Get the value of the color point fixed for relative luminance calculation
fn fixed_for_rel_luma(value: f32) -> f32 {
    if value <= 0.04045 {
        return value/12.92;
    }
    else {
        return ((value+0.055)/1.055).powf(2.4);
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    contrasts: Vec<Rc<RefCell<Color>>>,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r:{} g:{} b:{} a:{}", self.red, self.green, self.blue, self.alpha)
    }
}

impl Color {
    // Constructors
    pub fn new() -> Color {
        return Color {red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0, contrasts: Vec::new()};
    }

    pub fn from_rgb_int(r: u8, g: u8, b: u8) -> Option<Color> {
        let mut col: Color = Color::new();

        col.red = r as f32 / 255.0;
        col.green = g as f32 / 255.0;
        col.blue = b as f32 / 255.0;

        return Some(col);
    }

    pub fn from_rgba_int(r: u8, g: u8, b: u8, a: u8) -> Option<Color> {
        let mut col = Color::from_rgb_int(r, g, b);

        if col.as_ref().is_some() {
            col.as_mut().unwrap().alpha = a as f32 / 255.0;
        }

        return col;
    }

    pub fn from_rgb_float(r: f32, g: f32, b: f32) -> Option<Color> {
        return Some(Color{red: r, green: g, blue: b, alpha: 1.0, contrasts: Vec::new()});
    }

    pub fn from_rgba_float(r: f32, g: f32, b: f32, a: f32) -> Option<Color> {
        let mut col = Color::from_rgb_float(r, g, b);

        if col.as_ref().is_some() {
            col.as_mut().unwrap().alpha = a;
        }

        return col;
    }

    pub fn from_hex(hex: &str) -> Option<Color> {
        let mut col: Color = Color::new();

        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            if let Ok(color) = u32::from_str_radix(&hex[0..2], 16) {
                col.red = color as f32 / 255.0;
            }
            if let Ok(color) = u32::from_str_radix(&hex[2..4], 16) {
                col.green = color as f32 / 255.0;
            }
            if let Ok(color) = u32::from_str_radix(&hex[4..6], 16) {
                col.blue = color as f32 / 255.0;
            }
            return Some(col);
        }
        else if hex.len() == 9 {
            if let Ok(color) = u32::from_str_radix(&hex[0..2], 16) {
                col.red = color as f32 / 255.0;
            }
            if let Ok(color) = u32::from_str_radix(&hex[2..4], 16) {
                col.green = color as f32 / 255.0;
            }
            if let Ok(color) = u32::from_str_radix(&hex[4..6], 16) {
                col.blue = color as f32 / 255.0;
            }
            if let Ok(color) = u32::from_str_radix(&hex[6..8], 16) {
                col.alpha = color as f32 / 255.0;
            }
            return Some(col);
        }

        return None;
    }

    // Getters
    pub fn u8_red(&self) -> u8 {
        return (self.red * 255.0).round() as u8;
    }
    pub fn u8_green(&self) -> u8 {
        return (self.green * 255.0).round() as u8;
    }
    pub fn u8_blue(&self) -> u8 {
        return (self.blue * 255.0).round() as u8;
    }
    pub fn u8_alpha(&self) -> u8 {
        return (self.alpha * 255.0).round() as u8;
    }

    pub fn to_hex(&self) -> String {
        return format!( "{:02x}{:02x}{:02x}{:02x}", self.u8_red(), self.u8_green(), self.u8_blue(), self.u8_alpha());
    }

    // Setters
    fn set_contrasts(&mut self, contrasts: Vec<Rc<RefCell<Color>>>) {
        self.contrasts = contrasts;
    }

    // Compute the relative luminance of the color
    pub fn get_relative_luminance(&self) -> f32 {
        return (0.2126*fixed_for_rel_luma(self.red) +
                0.7152*fixed_for_rel_luma(self.green) +
                0.0722*fixed_for_rel_luma(self.blue)) * self.alpha;
    }

    // Get the contrast between 'self' color and 'other' color
    pub fn get_contrast(&self, other: &Color) -> f32 {
        let our_luma = self.get_relative_luminance();
        let other_luma = other.get_relative_luminance();

        if our_luma > other_luma {
            return (our_luma + 0.05) / (other_luma + 0.05);
        }
        else {
            return (other_luma + 0.05) / (our_luma + 0.05);
        }
    }

    // TODO: Use proper luminance calculation ?
    pub fn darker(&self, percentage: u8) -> Rc<RefCell<Color>> {
        let perc = (percentage as f32) / 100.0;

        let darker_color_point = |c: f32| {
            return (c - c * perc).clamp(0.0, 1.0);
        };

        let mut col = self.clone();
        col.red = darker_color_point(col.red);
        col.green = darker_color_point(col.green);
        col.blue = darker_color_point(col.blue);

        return Rc::new(RefCell::new(col));
    }

    pub fn lighter(&self, percentage: u8) -> Rc<RefCell<Color>> {
        let perc = (percentage as f32) / 100.0;

        let lighter_color_point = |c: f32| {
            return (c + c * perc).clamp(0.0, 1.0);
        };

        let mut col = self.clone();
        col.red = lighter_color_point(col.red);
        col.green = lighter_color_point(col.green);
        col.blue = lighter_color_point(col.blue);

        return Rc::new(RefCell::new(col));
    }

    pub fn alpha(&self, percentage: u8) -> Rc<RefCell<Color>> {
        let perc = (percentage.clamp(0, 100) as f32) / 100.0;

        let mut col = self.clone();
        col.alpha = perc;

        return Rc::new(RefCell::new(col));
    }

    // Get the indexth contrast of the color starting from 0
    // Contrast called on a darkened or lightened will return the original
    // color's contrast
    pub fn contrast(&self, index: usize) -> Rc<RefCell<Color>> {
        let c = self.contrasts.get(index);
        if c.is_some() {
            return c.unwrap().clone();
        }
        else {
            eprintln!("WARNING! Contrast index out of scope, returning the same color");
            return Rc::new(RefCell::new(self.clone()));
        }
    }
}

pub fn compute_contrasts(colors: &HashMap<String, Rc<RefCell<Color>>>) {
    for index in 0..(colors.len() - 1) {
        let mut contrasts_vec: VecDeque<Rc<RefCell<Color>>> = VecDeque::new();

        let mut ordered_contrasts: BTreeMap<u32, Rc<RefCell<Color>>> = BTreeMap::new();

        for (name_sec, col_sec) in colors.iter() {
            if name_sec != format!("color{index}").as_str() {
                let c = colors[format!("color{index}").as_str()].borrow().get_contrast(&(*col_sec.borrow()));
                ordered_contrasts.insert((c * 1000000.0).floor() as u32, col_sec.clone());
            }
        }

        for (_, ord_col) in ordered_contrasts {
            contrasts_vec.push_front(ord_col);
        }

        colors[format!("color{index}").as_str()].borrow_mut().set_contrasts(Vec::from(contrasts_vec));
    }
}
