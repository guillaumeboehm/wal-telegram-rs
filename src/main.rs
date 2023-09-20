mod background;
mod color;
mod palette;
mod telegram;
mod tmp_dir;

fn main() {
    // env variable for script always yes
    std::env::set_var("WAL_TELEGRAM_YES_ALL", "TRUE");
    // Create tmp dir
    let tmp_dir = tmp_dir::create_tmp_dir();

    // If wal then wal else custom blabla
    let palette = palette::generate_palette(palette::Palette::Wal, None);

    // Lets say it's wal background
    background::generate_background(&tmp_dir, background::Background::Wal, 0.1, None, None);
    // background::generate_background(&tmp_dir, background::Background::Plain, 0.0, Some(&palette), None);

    telegram::theme::package_theme(&tmp_dir, None, &palette);
}
