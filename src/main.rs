mod wal;
mod color;
mod telegram_colors;

use crate::telegram_colors::get_telegram_colors;
use crate::wal::import_wal_colors;

fn main() {

    // println!("{}", a.get_contrast(&b));
    let mut colors = import_wal_colors(None);

    color::compute_contrasts(&mut colors);

    println!("{}", get_telegram_colors(&colors));
}
