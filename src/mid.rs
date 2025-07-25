use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, style::{Color, Styled, Stylize}, text::{Line, Span, Text}, widgets::{Block, Paragraph, Widget, Wrap}};
use crate::Rat;

impl Rat {
    pub fn handle_mid(&mut self, kv: KeyEvent) {
        match kv.code {
            KeyCode::Up => self.value += 1,
            KeyCode::Down => self.value -= 1,
            _ => {}
        }
    }

    pub fn render_mid(&self, area: Rect, buf: &mut Buffer) {
        let text = Text::from(vec![
            Line::from("This is mid render! Isn't it peak? I'm going to wrap this text haha lol!").bold().blue(),
            Line::from(vec![
                Span::from("value: ").magenta(),
                Span::from(self.value.to_string()).set_style(
                    if self.value > 0 { Color::Green }
                    else if self.value < 0 { Color::Red }
                    else { Color::White }
                )
            ]).italic().centered(),
        ]);
        let rarea = Self::render_center(
            area,
            Some(Constraint::Length(text.width() as u16)),
            Some(Constraint::Length(text.height() as u16))
        );

        Block::bordered().yellow()
            .title_bottom("Press up/down to increase/decrease!".magenta().into_right_aligned_line())
            .render(
            Self::render_center(
                area,
                Some(Constraint::Max(text.width() as u16 + 4)),
                Some(Constraint::Max(text.height() as u16 * 7))
            ),
            buf
        );
            
        Paragraph::new(text)
            .wrap( Wrap { trim: true } )
            .render(rarea, buf)
        ;
    }
}
