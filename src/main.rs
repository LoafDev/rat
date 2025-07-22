use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{buffer::Buffer, layout::{Constraint, Flex, Layout, Rect}, style::{Color, Styled, Stylize}, text::{Line, Span, Text}, widgets::{Block, Paragraph, Widget, Wrap}, DefaultTerminal, Frame};

#[derive(PartialEq)]
enum EndState {
    Left,
    Right
}

#[derive(PartialEq)]
enum RatState {
    Start,
    Mid,
    End(EndState)
}

struct Rat {
    value: i32,
    state: RatState,
    quit: bool
}

impl Rat {
    fn default() -> Self {
        Rat {
            value: 0,
            state: RatState::Start,
            quit: false
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.quit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_key()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key(&mut self) -> Result<()> {
        if let Event::Key(kv) = event::read()? && let KeyEventKind::Press = kv.kind {
            match kv.code {
                KeyCode::Esc => { self.quit = true; }
                KeyCode::Right => {
                    match self.state {
                        RatState::Start => self.state = RatState::Mid,
                        RatState::Mid => self.state = RatState::End(EndState::Left),
                        RatState::End(_) => self.state = RatState::Start,
                    }
                }
                KeyCode::Left => {
                    match self.state {
                        RatState::Start => self.state = RatState::End(EndState::Left),
                        RatState::Mid => self.state = RatState::Start,
                        RatState::End(_) => self.state = RatState::Mid,
                    }
                }
                KeyCode::Up if self.state == RatState::Mid => self.value += 1,
                KeyCode::Down if self.state == RatState::Mid => self.value -= 1,
                KeyCode::Tab if self.state == RatState::End(EndState::Right) || self.state == RatState::End(EndState::Left) => {
                    if let RatState::End(EndState::Left) = self.state { self.state = RatState::End(EndState::Right); }
                    else {  self.state = RatState::End(EndState::Left); }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn render_center(mut area: Rect, hori: Option<Constraint>, vert: Option<Constraint>) -> Rect {
        if let Some(h) = hori {
            [area] = Layout::horizontal([h])
                .flex(Flex::Center)
                .areas(area)
            ;
        }

        if let Some(v) = vert {
            [area] = Layout::vertical([v])
                .flex(Flex::Center)
                .areas(area)
            ;
        }
        area
    }

    fn render_start(&self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .title("This is the start!".blue())
            .title_bottom("Press left/right to advance screen".italic().red().into_right_aligned_line())
            .render(area, buf)
        ;

        let middle_text = Text::from(vec![
            Line::from("Hello my friend".bold().yellow().into_centered_line()),
            Line::from("Press Esc to quit/escape".italic().white()),
        ]);
        let center_rect = Rat::render_center(
            area,
            Some(Constraint::Length(middle_text.width() as u16)),
            Some(Constraint::Length(middle_text.height() as u16))
        );
        middle_text.render(center_rect, buf);
    }

    fn render_mid(&self, area: Rect, buf: &mut Buffer) {
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

    fn render_end(&self, area: Rect, buf: &mut Buffer) {
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

impl Widget for &Rat {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            RatState::Start => self.render_start(area, buf),
            RatState::Mid => self.render_mid(area, buf),
            RatState::End(_) => self.render_end(area, buf),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut rat = ratatui::init();
    let result = Rat::default().run(&mut rat);
    ratatui::restore();

    result
}
