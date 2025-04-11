use audio_visualizer::audio::capture_audio;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

fn main() -> Result<()> {
    color_eyre::install().expect("failed to install color_eyre");
    capture_audio();
    /* let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore(); */

    Ok(())
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}