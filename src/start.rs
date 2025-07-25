use std::time::{Duration, Instant};
use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, text::{Line, Text}, widgets::{Block, List, StatefulWidget, Widget}};
use crate::Rat;

const NEXT_ANIM: Duration = Duration::from_millis(100);

impl Rat {
    pub fn render_start(&mut self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .title("This is the start!".blue())
            .title_bottom("Press left/right to advance screen".italic().red().into_right_aligned_line())
            .render(area, buf)
        ;

        let middle_text = Text::from(vec![
            Line::from("Hello my friend".bold().yellow().into_centered_line()),
            Line::from("Press Esc to quit/escape".italic().white().into_centered_line()),
            Line::from("Also here is a random list for no reason at all!".italic().magenta()),
        ]);
        let center_rect = Rat::render_center(
            area,
            Some(Constraint::Length(middle_text.width() as u16)),
            Some(Constraint::Length(middle_text.height() as u16))
        );

        let mut list_rect = Rat::render_center(area, Some(Constraint::Length(6)), Some(Constraint::Length(10)));
        list_rect = Layout::vertical([Constraint::Percentage(65), Constraint::Percentage(35)]).split(list_rect)[1];
        let list = List::new(vec![
            Line::from("Start").centered(),
            Line::from("Mid").centered(),
            Line::from("End").centered()
        ]).magenta().highlight_symbol("-").highlight_style(Color::Green);

        if self.last_tick.elapsed() >= NEXT_ANIM {
            if self.sublist_state.selected().unwrap_or_default() == list.len() - 1 { self.sublist_state.select(Some(0)); }
            else { self.sublist_state.select_next(); }
            self.last_tick = Instant::now();
        }

        middle_text.render(center_rect, buf);
        StatefulWidget::render(list, list_rect, buf, &mut self.sublist_state);
    }
}
