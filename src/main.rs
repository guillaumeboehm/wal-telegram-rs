mod palette;
mod color;
mod telegram;
mod background;

use crate::telegram::colors::get_telegram_colors;
use crate::palette::wal::import_wal_palette;

fn main() {

    // If wal then wal else custom blabla
    let palette = import_wal_palette("");

    color::compute_contrasts(&palette);

    // use palette for telegram colors
    let telegram_colors = get_telegram_colors(&palette);

    // Lets say it's wal background
    let bg = background::wal::fetch_wal_background(None);

    // env variable for script always yes
    std::env::set_var("WAL_TELEGRAM_YES_ALL", "TRUE");

    telegram::theme::package_theme(None, bg.as_str(), false, &palette);
}
