use std::{io, vec};

use anyhow::Result;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use midiedit_core::Tui;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, ListItem, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    tempo: u16,
    exit: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Tui::parse();
    ratatui::run(|terminal| App::default().run(terminal))?;

    Ok(())
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [top, center] =
            Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).areas(frame.area());
        let [left, right] =
            Layout::horizontal(vec![Constraint::Percentage(15), Constraint::Fill(1)]).areas(center);

        let block = Block::bordered()
            .title(Line::from(" MidiEdit 0.1.0 ".bold()).left_aligned())
            .border_set(border::THICK);

        let tempo_text = Text::from(vec![Line::from(vec![
            "Tempo: ".into(),
            self.tempo.to_string().yellow(),
        ])]);

        let top_widget = Paragraph::new(tempo_text).left_aligned().block(block);

        frame.render_widget(top_widget, top);

        let temp_items = [ListItem::new("Track 1"), ListItem::new("Track 2")];

        let track_block = Block::bordered().title("Tracks");

        let track_list = List::new(temp_items).block(track_block);

        frame.render_widget(track_list, left);

        let piano_roll_widget = Block::bordered().title("Piano Roll");

        frame.render_widget(piano_roll_widget, right);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.tempo += 1;
    }

    fn decrement_counter(&mut self) {
        self.tempo -= 1;
    }
}
