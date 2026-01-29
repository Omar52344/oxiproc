use crate::process::{GlobalStats, ProcessCategory, ProcessData};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn draw(
    f: &mut Frame,
    processes: &[ProcessData],
    global_stats: &GlobalStats,
    state: &mut ListState,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Header + Stats
            Constraint::Min(0),    // Process List
            Constraint::Length(3), // Footer / Status
        ])
        .split(f.area());

    // Header Area
    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Stats
            Constraint::Min(0),
        ])
        .split(chunks[0]);

    let title = Paragraph::new("Oxiproc - Lightweight Process Guardian")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, header_chunks[0]);

    // Stats Area (Gauges)
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Memory
            Constraint::Percentage(50), // CPU
        ])
        .split(header_chunks[1]);

    // Memory Gauge
    let mem_used = global_stats.used_memory as f64;
    let mem_total = global_stats.total_memory as f64;
    let mem_percent = if mem_total > 0.0 {
        (mem_used / mem_total * 100.0) as u16
    } else {
        0
    };
    let mem_label = format!(
        "Mem: {} / {} ({}%)",
        format_memory(global_stats.used_memory),
        format_memory(global_stats.total_memory),
        mem_percent
    );

    let mem_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Memory"))
        .gauge_style(Style::default().fg(Color::Magenta))
        .percent(mem_percent)
        .label(mem_label);

    f.render_widget(mem_gauge, stats_chunks[0]);

    // CPU Gauge
    let cpu_val = global_stats.cpu_usage; // 0-100
    let cpu_percent = cpu_val.clamp(0.0, 100.0) as u16;
    let cpu_label = format!("CPU: {:.1}%", cpu_val);

    let cpu_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("CPU"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(cpu_percent)
        .label(cpu_label);

    f.render_widget(cpu_gauge, stats_chunks[1]);

    // Process List
    let items: Vec<ListItem> = processes
        .iter()
        .map(|p| {
            let color = match p.category {
                ProcessCategory::System => Color::Red,
                ProcessCategory::Service => Color::Yellow,
                ProcessCategory::User => Color::Green,
            };

            let content = Line::from(vec![
                Span::styled(format!("{:<8}", p.pid), Style::default().fg(Color::Gray)),
                Span::styled(format!("{:<30}", p.name), Style::default().fg(color)),
                Span::styled(
                    format!("{:<10.1}%", p.cpu_usage),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!("{:<15}", format_memory(p.memory)),
                    Style::default().fg(Color::White),
                ),
                Span::styled(format!("{:?}", p.category), Style::default().fg(color)),
            ]);

            ListItem::new(content)
        })
        .collect();

    let selected_index = state.selected().unwrap_or(0);
    let total_count = processes.len();
    let list_title = format!(
        " Processes ({}) - Selected: {} ",
        total_count, selected_index
    );

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(list_title))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[1], state);

    // Footer with controls
    let uptime_desc = format_duration(global_stats.uptime);
    let footer_text = vec![
        Span::raw("Press "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to quit, "),
        Span::styled("k", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to kill, "),
        Span::styled("Up/Down", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" nav. "),
        Span::styled(
            format!("Uptime: {}", uptime_desc),
            Style::default().fg(Color::Blue),
        ),
    ];
    let footer =
        Paragraph::new(Line::from(footer_text)).block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn format_memory(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else {
        format!("{} KB", bytes / KB)
    }
}

fn format_duration(secs: u64) -> String {
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, s)
}
