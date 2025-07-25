use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Styled, Stylize}, text::{Line, Text}, widgets::{Block, Widget}};
use crate::{Rat, RatState};

#[derive(PartialEq, Clone, Copy)]
pub enum EndState {
    Left,
    Right
}

impl Rat {
    pub fn handle_end(&mut self, kv: KeyEvent) {
        match kv.code {
            KeyCode::Tab => {
                if let RatState::End(EndState::Left) = self.state { self.state = RatState::End(EndState::Right); }
                else {  self.state = RatState::End(EndState::Left); }
            }
            _ => {}
        }
    }

    pub fn render_end(&self, area: Rect, buf: &mut Buffer) {
        let [left, right] = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);
        let left_colour = if let RatState::End(EndState::Left) = self.state { Color::LightYellow} else { Color::LightMagenta };
        let right_colour = if let RatState::End(EndState::Right) = self.state { Color::LightYellow } else { Color::LightMagenta };

        let left_text = Text::from(vec![
            Line::from("PEAK LEFT".red().bold()),
            Line::from("yes".blue().italic().into_centered_line())
        ]);

        let left_area = Self::render_center(
            left,
            Some(Constraint::Length(left_text.width() as u16)),
            Some(Constraint::Length(left_text.height() as u16))
        );

        let right_text = Text::from(vec![
            Line::from("PEAK RIGHT".blue().bold()),
            Line::from("no".red().italic().into_centered_line())
        ]);

        let right_area = Self::render_center(
            right,
            Some(Constraint::Length(right_text.width() as u16)),
            Some(Constraint::Length(right_text.height() as u16))
        );

        Block::bordered()
            .title_top("Press tab to".set_style(right_colour).bold().into_centered_line())
            .blue()
            .bg(left_colour)
            .render(left, buf);
        left_text.render(left_area, buf);

        Block::bordered()
            .title_top("switch colour!".set_style(left_colour).bold().into_centered_line())
            .red()
            .bg(right_colour)
            .render(right, buf);
        right_text.render(right_area, buf);
    }
}
