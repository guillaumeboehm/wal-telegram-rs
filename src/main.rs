mod palette;
mod color;
mod telegram;

use crate::telegram::colors::get_telegram_colors;
use crate::palette::wal::import_wal_colors;

fn main() {

    let mut colors = import_wal_colors(None);

    color::compute_contrasts(&mut colors);

    println!("{}", get_telegram_colors(&colors));
}
