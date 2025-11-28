mod widgets;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
    Frame,
};

use crate::app::App;
use crate::collectors::memory::format_bytes;
use crate::collectors::system::format_uptime;

/// Main render function
pub fn render(frame: &mut Frame, app: &App) {
    let theme = &app.theme;

    // Main layout: Header, Content, Footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Content
            Constraint::Length(1),  // Footer
        ])
        .split(frame.area());

    // Render header
    render_header(frame, app, main_chunks[0]);

    // Content layout: CPU/Memory side by side
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12), // CPU
            Constraint::Length(8),  // Memory
            Constraint::Min(3),     // Placeholder for more
        ])
        .split(main_chunks[1]);

    // Render CPU widget
    render_cpu(frame, app, content_chunks[0]);

    // Render Memory widget
    render_memory(frame, app, content_chunks[1]);

    // Placeholder
    let placeholder = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(" More metrics coming soon... ");
    frame.render_widget(placeholder, content_chunks[2]);

    // Render footer
    render_footer(frame, app, main_chunks[2]);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let sys = &app.collectors.system.data;

    let now = chrono::Local::now();
    let time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let header_text = vec![
        Line::from(vec![
            Span::styled("  ⚡ SYSTEM MONITOR ", Style::default().fg(theme.primary).bold()),
            Span::styled(
                format!("│ {} {} │ Kernel {} │ Uptime: {} │ ",
                    sys.os_name,
                    sys.os_version,
                    sys.kernel_version,
                    format_uptime(sys.uptime_secs)
                ),
                Style::default().fg(theme.muted)
            ),
            Span::styled(format!("🕐 {}", time_str), Style::default().fg(theme.accent)),
        ]),
    ];

    let header = Paragraph::new(header_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border))
        );

    frame.render_widget(header, area);
}

fn render_cpu(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let cpu = &app.collectors.cpu;
    let data = &cpu.data;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(format!(
            " CPU ─ {} ─ {} Cores / {} Threads ",
            data.model,
            data.physical_cores,
            data.logical_cores
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Layout inside CPU block
    let cpu_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Global usage
            Constraint::Length(1), // Frequency
            Constraint::Length(1), // Spacer
            Constraint::Length(3), // Per-core bars
            Constraint::Min(2),    // Sparkline
        ])
        .split(inner);

    // Global usage gauge
    let usage_color = theme.usage_color(data.global_usage);
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(usage_color).bg(theme.bar_empty))
        .ratio(data.global_usage as f64 / 100.0)
        .label(format!("Global: {:.1}%", data.global_usage));
    frame.render_widget(gauge, cpu_layout[0]);

    // Frequency
    let freq_text = Paragraph::new(format!(
        "Frequency: {} MHz",
        data.frequency_mhz
    )).style(Style::default().fg(theme.muted));
    frame.render_widget(freq_text, cpu_layout[1]);

    // Per-core usage (compact display)
    let cores_per_row = 8;
    let mut core_lines = Vec::new();
    for chunk in data.per_core_usage.chunks(cores_per_row) {
        let spans: Vec<Span> = chunk
            .iter()
            .enumerate()
            .map(|(i, &usage)| {
                let color = theme.usage_color(usage);
                Span::styled(format!("[{:3.0}%]", usage), Style::default().fg(color))
            })
            .collect();
        core_lines.push(Line::from(spans));
    }
    let cores_para = Paragraph::new(core_lines);
    frame.render_widget(cores_para, cpu_layout[3]);

    // Sparkline
    let history = cpu.history.as_vec();
    if !history.is_empty() {
        let sparkline = Sparkline::default()
            .data(&history)
            .max(100)
            .style(Style::default().fg(theme.graph_line));
        frame.render_widget(sparkline, cpu_layout[4]);
    }
}

fn render_memory(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let mem = &app.collectors.memory;
    let data = &mem.data;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(" MEMORY ");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Layout inside Memory block
    let mem_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // RAM gauge
            Constraint::Length(1), // RAM details
            Constraint::Length(1), // Swap
            Constraint::Min(1),    // Sparkline
        ])
        .split(inner);

    // RAM usage gauge
    let ram_color = theme.usage_color(data.usage_percent);
    let ram_gauge = Gauge::default()
        .gauge_style(Style::default().fg(ram_color).bg(theme.bar_empty))
        .ratio(data.usage_percent as f64 / 100.0)
        .label(format!(
            "RAM: {:.1}% ({} / {})",
            data.usage_percent,
            format_bytes(data.used),
            format_bytes(data.total)
        ));
    frame.render_widget(ram_gauge, mem_layout[0]);

    // RAM details
    let ram_details = Paragraph::new(format!(
        "Available: {}",
        format_bytes(data.available)
    )).style(Style::default().fg(theme.muted));
    frame.render_widget(ram_details, mem_layout[1]);

    // Swap
    let swap_text = if data.swap_total > 0 {
        format!(
            "Swap: {:.1}% ({} / {})",
            data.swap_percent,
            format_bytes(data.swap_used),
            format_bytes(data.swap_total)
        )
    } else {
        "Swap: N/A".to_string()
    };
    let swap_para = Paragraph::new(swap_text).style(Style::default().fg(theme.muted));
    frame.render_widget(swap_para, mem_layout[2]);

    // Sparkline
    let history = mem.history.as_vec();
    if !history.is_empty() {
        let sparkline = Sparkline::default()
            .data(&history)
            .max(100)
            .style(Style::default().fg(theme.graph_line));
        frame.render_widget(sparkline, mem_layout[3]);
    }
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" [Q]", Style::default().fg(theme.accent)),
        Span::styled("uit  ", Style::default().fg(theme.muted)),
        Span::styled("[T]", Style::default().fg(theme.accent)),
        Span::styled("heme  ", Style::default().fg(theme.muted)),
        Span::styled("[R]", Style::default().fg(theme.accent)),
        Span::styled("efresh  ", Style::default().fg(theme.muted)),
        Span::styled("[H]", Style::default().fg(theme.accent)),
        Span::styled("elp  ", Style::default().fg(theme.muted)),
        Span::styled(
            format!("│ Theme: {} │ v0.1.0", theme.name),
            Style::default().fg(theme.secondary)
        ),
    ]));

    frame.render_widget(footer, area);
}
