#![allow(warnings)]

// Small abstraction to make things a little easier
use std::{ io, ops::Add, time::{Duration, Instant}};use crossterm::{ event::{self, Event as CrosstermEvent, KeyEvent}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},};use ratatui::{Terminal, backend::CrosstermBackend};pub trait TuiApp { fn draw(&mut self, frame: &mut ratatui::Frame); fn event(&mut self, event: AppEvent) -> bool;}pub enum AppEvent { Key(KeyEvent), Tick,}pub struct TuiConfig { pub tick_rate: Duration, pub debounce: Option<Duration>,}impl Default for TuiConfig { fn default() -> Self { Self { tick_rate: Duration::from_millis(250), debounce: None, } }}pub struct Tui { terminal: Terminal<CrosstermBackend<io::Stdout>>, config: TuiConfig,}impl Tui { pub fn new(config: TuiConfig) -> io::Result<Self> { enable_raw_mode()?; let mut stdout = io::stdout(); execute!(stdout, EnterAlternateScreen)?; let backend = CrosstermBackend::new(stdout); let terminal = Terminal::new(backend)?; Ok(Self { terminal, config }) } pub fn run<A: TuiApp>(&mut self, mut app: A) -> io::Result<()> { let mut last_tick = Instant::now(); let mut last_key = Instant::now(); loop { self.terminal.draw(|f| app.draw(f))?; let timeout = self .config .tick_rate .saturating_sub(last_tick.elapsed()); if event::poll(timeout)? { if let CrosstermEvent::Key(key) = event::read()? { if let Some(debounce) = self.config.debounce { if last_key.elapsed() < debounce { continue; } last_key = Instant::now(); } if !app.event(AppEvent::Key(key)) { break; } } } if last_tick.elapsed() >= self.config.tick_rate { last_tick = Instant::now(); if !app.event(AppEvent::Tick) { break; } } } Ok(()) }}impl Drop for Tui { fn drop(&mut self) { let _ = disable_raw_mode(); let _ = execute!( self.terminal.backend_mut(), LeaveAlternateScreen ); let _ = self.terminal.show_cursor(); }}





use crossterm::event::{KeyCode}; // KeyEvent is already defined in the abstraction layer
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style, Color},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    text::{Line, Span}
};

pub mod powerline;


struct App {
    step_list: Vec<&'static str>,
    focus_selected: usize,
    focus_cursor: usize,
}

impl App {
    fn new() -> Self {
        Self {
            step_list: vec!["Topic", "Script", "Speech", "Align", "Render", "Thumbnail"],
            focus_selected: 0,
            focus_cursor: 0,
        }
    }
}

impl TuiApp for App {
    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let sepstyle = powerline::SeparatorStyle::CUSTOM { enter: '', enter_is_reversed: true, exit: '', exit_is_reversed: false };
        let leftsep = powerline::Separator::new(sepstyle, 0);
        let my_segment = powerline::Segment::new(Color::Black, Color::Green, "Lorem", 1, 1, &leftsep, &leftsep);
        let another_segment = powerline::Segment::new(Color::Black, Color::Red, "ipsum", 1, 1, &leftsep, &leftsep);
        let le_text = Line::from(powerline::compute(vec![my_segment, another_segment]));





        let root = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(6),
                Constraint::Length(1),
            ])
            .split(frame.area());

        let top_info = Paragraph::new(" powerline-prompt-generato-rs v0.1.0 by SoggySupernova").style(Style::new().blue().add_modifier(Modifier::REVERSED));
        frame.render_widget(top_info, root[0]);
        // Main content (list + info)
        let content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(17), Constraint::Fill(1)])
            .split(root[1]);

        
        let main_content_block = Block::default()
        .title(self.step_list[self.focus_selected])
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::new().blue())
        ;

        let main_content = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1),Constraint::Length(4),Constraint::Length(1)])
                .split(content[1]);
        
        // -------- Left: List --------
        

        // -------- Right: Info --------
        let main_log = Paragraph::new(le_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(format!(" {} ",self.step_list[self.focus_selected]))
                .border_style(Style::new().blue())
            );
        frame.render_widget(main_log, main_content[0]);




        // -------- Help Bar --------
        let help = Paragraph::new(
            Line::from(vec![
                Span::styled("Arrow keys", Style::new().blue().add_modifier(Modifier::BOLD)),
                Span::styled(" to navigate, ", Style::default()),
                Span::styled("Enter", Style::new().blue().add_modifier(Modifier::BOLD)),
                Span::styled(" to select, ", Style::default()),
                Span::styled("Q", Style::new().blue().add_modifier(Modifier::BOLD)),
                Span::styled(" to quit", Style::default()),
            ])
        )
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(help, root[2]);

        // -------- Footer --------
    }

    fn event(&mut self, event: AppEvent) -> bool {
        match event {
            AppEvent::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char('q') => return false, // regardless of focus
                KeyCode::Up => {
                        if self.focus_cursor > 0 {
                            self.focus_cursor -= 1;
                        }
                    
            
                },
                KeyCode::Down =>  {
                    
                        if self.focus_cursor + 1 < self.step_list.len() {
                            self.focus_cursor += 1;
                        }
                    
                    
                },
                KeyCode::Enter => {
                    // Selection hook (no-op for now)
                        

                            self.focus_selected = self.focus_cursor;
                        
                    
                }
                KeyCode::Left | KeyCode::Right => {
                    
                }
                _ => {}
            },
            _ => {}
        }
        true
    }
}



fn main() -> io::Result<()> {
    let mut tui = Tui::new(TuiConfig {
        debounce: Some(std::time::Duration::from_millis(50)),
        ..Default::default()
    })?;
    println!("powerline time");

    tui.run(App::new())
}
