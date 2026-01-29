use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use sysinfo::System;

mod process;
mod ui;

use process::{collect_processes, Categorizer, GlobalStats, ProcessCategory, ProcessData};

enum AppEvent {
    Input(Event),
    Tick,
    ProcessesUpdated(Vec<ProcessData>, GlobalStats),
}

struct App {
    processes: Vec<ProcessData>,
    global_stats: GlobalStats,
    state: ratatui::widgets::ListState,
    should_quit: bool,
    status_message: Option<(String, Instant)>,
}

impl App {
    fn new() -> Self {
        let mut state = ratatui::widgets::ListState::default();
        state.select(Some(0));
        Self {
            processes: Vec::new(),
            global_stats: GlobalStats {
                total_memory: 0,
                used_memory: 0,
                cpu_usage: 0.0,
                uptime: 0,
            },
            state,
            should_quit: false,
            status_message: None,
        }
    }

    fn on_tick(&mut self) {
        if let Some((_, time)) = self.status_message {
            if time.elapsed() > Duration::from_secs(3) {
                self.status_message = None;
            }
        }
    }

    fn next(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.processes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.processes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn kill_selected_process(&mut self) -> Result<()> {
        if let Some(index) = self.state.selected() {
            if let Some(proc) = self.processes.get(index) {
                match proc.category {
                    ProcessCategory::System => {
                        self.set_status("⚠️  Cannot kill System process! Blocked.");
                    }
                    ProcessCategory::Service => {
                        self.set_status(
                            "⚠️  Service process needs confirmation (Not implemented).",
                        );
                    }
                    ProcessCategory::User => {
                        #[cfg(target_os = "windows")]
                        {
                            let _ = std::process::Command::new("taskkill")
                                .args(["/F", "/PID", &proc.pid.to_string()])
                                .output();
                            self.set_status(format!(
                                "💀 Killed process {} ({})",
                                proc.name, proc.pid
                            ));
                        }
                        #[cfg(not(target_os = "windows"))]
                        {
                            self.set_status("Kill implementation pending for non-windows.");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn set_status<S: Into<String>>(&mut self, msg: S) {
        self.status_message = Some((msg.into(), Instant::now()));
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();
    let event_tx = tx.clone();

    thread::spawn(move || {
        let mut sys = System::new_all();
        let categorizer = Categorizer::new();
        loop {
            let (procs, stats) = collect_processes(&mut sys, &categorizer);
            if tx.send(AppEvent::ProcessesUpdated(procs, stats)).is_err() {
                break;
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });

    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(200)).expect("Event poll failed") {
            if let Ok(evt) = event::read() {
                if event_tx.send(AppEvent::Input(evt)).is_err() {
                    break;
                }
            }
        } else {
            if event_tx.send(AppEvent::Tick).is_err() {
                break;
            }
        }
    });

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            ui::draw(f, &app.processes, &app.global_stats, &mut app.state);
            if let Some((msg, _)) = &app.status_message {
                let area = f.area();
                use ratatui::style::{Color, Style};
                use ratatui::widgets::{Block, Borders, Paragraph};
                let p = Paragraph::new(msg.as_str())
                    .style(Style::default().fg(Color::Red).bg(Color::Black))
                    .block(Block::default().borders(Borders::ALL));
                let rect =
                    ratatui::layout::Rect::new(area.x + 2, area.height - 5, area.width - 4, 3);
                f.render_widget(p, rect);
            }
        })?;

        match rx.recv()? {
            AppEvent::Input(event) => match event {
                Event::Key(key) => {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                app.should_quit = true;
                            }
                            KeyCode::Char('k') => {
                                app.kill_selected_process()?;
                            }
                            KeyCode::Down => app.next(),
                            KeyCode::Up => app.previous(),
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            AppEvent::ProcessesUpdated(data, stats) => {
                app.processes = data;
                app.global_stats = stats;
            }
            AppEvent::Tick => {
                app.on_tick();
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
