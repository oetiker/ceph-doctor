use crate::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

pub struct TerminalManager {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalManager {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(TerminalManager { terminal })
    }

    pub fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<io::Stdout>> {
        &mut self.terminal
    }

    pub fn cleanup(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn poll_event(&self, timeout: Duration) -> Result<bool> {
        Ok(event::poll(timeout)?)
    }

    pub fn read_event(&self) -> Result<Event> {
        Ok(event::read()?)
    }

    pub fn should_quit(&self, event: &Event) -> bool {
        match event {
            Event::Key(key) => self.is_quit_key(key),
            _ => false,
        }
    }

    pub fn is_quit_key(&self, key: &KeyEvent) -> bool {
        matches!(
            key,
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } | KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } | KeyEvent {
                code: KeyCode::Esc,
                ..
            }
        )
    }
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

pub async fn sleep_with_event_check(
    duration_secs: u64,
    terminal_manager: &TerminalManager,
) -> Result<SleepResult> {
    let mut remaining_time = duration_secs as f32;
    while remaining_time > 0.0 {
        let sleep_duration = remaining_time.min(0.25);
        tokio::time::sleep(Duration::from_secs_f32(sleep_duration)).await;
        remaining_time -= sleep_duration;

        if terminal_manager.poll_event(Duration::from_millis(0))? {
            let event = terminal_manager.read_event()?;
            if terminal_manager.should_quit(&event) {
                return Ok(SleepResult::Quit);
            }
            if matches!(event, Event::Resize(_, _)) {
                return Ok(SleepResult::Resize);
            }
        }
    }
    Ok(SleepResult::Continue)
}

#[derive(Debug, PartialEq)]
pub enum SleepResult {
    Continue,
    Quit,
    Resize,
}
