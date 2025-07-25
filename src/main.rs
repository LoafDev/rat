use std::time::{Duration, Instant};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use donut::Donut;
use ratatui::{buffer::Buffer, layout::{Constraint, Flex, Layout, Rect}, widgets::{ListState, Widget}, DefaultTerminal, Frame};
mod donut; mod start; mod end; mod mid; mod canva;
use crate::end::EndState;

const READ_BLOCK: Duration = Duration::from_millis(50);

#[derive(PartialEq, Clone, Copy)]
enum RatState {
    Start,
    Mid,
    End(EndState),
    Donut,
    Canva
}

struct Rat {
    value: i32,
    state: RatState,
    list: Vec<RatState>,
    list_state: ListState, 
    sublist_state: ListState,
    last_tick: Instant,
    donut: Donut,
    canva_offset: [[f64; 2]; 4],
    toggle_offset: bool,
    offset_direction: bool,
    quit: bool
}

impl Default for Rat {
    fn default() -> Self {
        Rat {
            value: 0,
            state: RatState::Start,
            list: vec![
                RatState::Start,
                RatState::Mid,
                RatState::End(EndState::Left),
                RatState::Donut,
                RatState::Canva
            ],
            list_state: ListState::default().with_selected(Some(0)),
            sublist_state: ListState::default().with_selected(Some(0)),
            last_tick: Instant::now(),
            donut: Donut::default(),
            canva_offset: [[0.; 2]; 4],
            toggle_offset: true,
            offset_direction: false,
            quit: false
        }
    }
}

impl Rat {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.quit {
            terminal.draw(|f| self.draw(f))?;
            if event::poll(READ_BLOCK)? { self.handle_key()?; }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key(&mut self) -> Result<()> {
        if let Event::Key(kv) = event::read()? && let KeyEventKind::Press = kv.kind {
            match kv.code {
                KeyCode::Esc => { self.quit = true; }
                KeyCode::Right => {
                    self.list_state.select_next();
                    if self.list_state.selected().unwrap_or_default() >= self.list.len() { self.list_state.select(Some(0)); }
                    self.state = self.list[self.list_state.selected().unwrap_or_default()];
                }
                KeyCode::Left => {
                    if self.list_state.selected().unwrap_or_default() == 0 { self.list_state.select(Some(self.list.len() - 1)); } 
                    else { self.list_state.select_previous(); }
                    self.state = self.list[self.list_state.selected().unwrap_or_default()];
                }
                _ => {}
            }

            match self.state {
                RatState::Start => {},
                RatState::Mid => self.handle_mid(kv),
                RatState::End(_) => self.handle_end(kv),
                RatState::Donut => self.handle_donut(kv),
                RatState::Canva => self.handle_canva(kv),
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
}

impl Widget for &mut Rat {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            RatState::Start => self.render_start(area, buf),
            RatState::Mid => self.render_mid(area, buf),
            RatState::End(_) => self.render_end(area, buf),
            RatState::Donut => self.render_donut(area, buf),
            RatState::Canva => self.render_canva(area, buf)
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
