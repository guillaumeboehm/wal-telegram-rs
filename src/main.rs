mod palette;
mod color;
mod telegram;
mod background;

use crate::palette::wal;

fn main() {

    // If wal then wal else custom blabla
    let palette = wal::import_wal_palette("");
    color::compute_contrasts(&palette);

    // Lets say it's wal background
    let bg = background::wal::fetch_wal_background(None);

    // env variable for script always yes
    std::env::set_var("WAL_TELEGRAM_YES_ALL", "TRUE");

    telegram::theme::package_theme(None, bg.as_str(), false, &palette);
}
