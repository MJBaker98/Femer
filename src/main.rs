mod file_utils;
mod state;
mod tui;

fn main() -> color_eyre::Result<()> {
    tui::run()
}
