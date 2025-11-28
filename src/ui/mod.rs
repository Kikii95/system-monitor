mod widgets;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline, Row, Table},
    Frame,
};

use crate::app::App;
use crate::collectors::memory::format_bytes;
use crate::collectors::system::format_uptime;
use crate::collectors::network::{format_speed, format_total};
use crate::collectors::disk::format_space;
use crate::collectors::process::format_proc_memory;

/// Main render function
pub fn render(frame: &mut Frame, app: &App) {
    let theme = &app.theme;

    // Main layout: Header, Content, Footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(20),    // Content
            Constraint::Length(1),  // Footer
        ])
        .split(frame.area());

    // Render header
    render_header(frame, app, main_chunks[0]);

    // Content: Two columns
    let content_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),  // Left column
            Constraint::Percentage(40),  // Right column
        ])
        .split(main_chunks[1]);

    // Left column: CPU, Memory/Swap, GPU
    let left_col = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // CPU
            Constraint::Length(6),  // Memory
            Constraint::Length(6),  // GPU
            Constraint::Min(3),     // Processes
        ])
        .split(content_cols[0]);

    render_cpu(frame, app, left_col[0]);
    render_memory(frame, app, left_col[1]);
    render_gpu(frame, app, left_col[2]);
    render_processes(frame, app, left_col[3]);

    // Right column: Network, Disk, System Info
    let right_col = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Network
            Constraint::Length(6),  // Disk
            Constraint::Min(3),     // System
        ])
        .split(content_cols[1]);

    render_network(frame, app, right_col[0]);
    render_disk(frame, app, right_col[1]);
    render_system_info(frame, app, right_col[2]);

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
            " CPU ─ {} ─ {}C/{}T ",
            truncate_str(&data.model, 30),
            data.physical_cores,
            data.logical_cores
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let cpu_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Global usage
            Constraint::Length(1), // Frequency
            Constraint::Length(3), // Per-core bars
            Constraint::Min(1),    // Sparkline
        ])
        .split(inner);

    // Global usage gauge
    let usage_color = theme.usage_color(data.global_usage);
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(usage_color).bg(theme.bar_empty))
        .ratio((data.global_usage as f64 / 100.0).min(1.0))
        .label(format!("Global: {:.1}%", data.global_usage));
    frame.render_widget(gauge, cpu_layout[0]);

    // Frequency
    let freq_text = Paragraph::new(format!(
        "Freq: {} MHz",
        data.frequency_mhz
    )).style(Style::default().fg(theme.muted));
    frame.render_widget(freq_text, cpu_layout[1]);

    // Per-core usage (compact)
    let cores_per_row = 8;
    let mut core_lines = Vec::new();
    for chunk in data.per_core_usage.chunks(cores_per_row) {
        let spans: Vec<Span> = chunk
            .iter()
            .map(|&usage| {
                let color = theme.usage_color(usage);
                Span::styled(format!("[{:2.0}]", usage), Style::default().fg(color))
            })
            .collect();
        core_lines.push(Line::from(spans));
    }
    let cores_para = Paragraph::new(core_lines);
    frame.render_widget(cores_para, cpu_layout[2]);

    // Sparkline
    let history = cpu.history.as_vec();
    if !history.is_empty() {
        let sparkline = Sparkline::default()
            .data(&history)
            .max(100)
            .style(Style::default().fg(theme.graph_line));
        frame.render_widget(sparkline, cpu_layout[3]);
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

    let mem_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // RAM gauge
            Constraint::Length(1), // Details
            Constraint::Min(1),    // Sparkline
        ])
        .split(inner);

    // RAM gauge
    let ram_color = theme.usage_color(data.usage_percent);
    let ram_gauge = Gauge::default()
        .gauge_style(Style::default().fg(ram_color).bg(theme.bar_empty))
        .ratio((data.usage_percent as f64 / 100.0).min(1.0))
        .label(format!(
            "RAM: {:.1}% ({}/{})",
            data.usage_percent,
            format_bytes(data.used),
            format_bytes(data.total)
        ));
    frame.render_widget(ram_gauge, mem_layout[0]);

    // Details
    let details = Paragraph::new(format!(
        "Avail: {} │ Swap: {:.0}% ({}/{})",
        format_bytes(data.available),
        data.swap_percent,
        format_bytes(data.swap_used),
        format_bytes(data.swap_total)
    )).style(Style::default().fg(theme.muted));
    frame.render_widget(details, mem_layout[1]);

    // Sparkline
    let history = mem.history.as_vec();
    if !history.is_empty() {
        let sparkline = Sparkline::default()
            .data(&history)
            .max(100)
            .style(Style::default().fg(theme.graph_line));
        frame.render_widget(sparkline, mem_layout[2]);
    }
}

fn render_gpu(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let gpu = &app.collectors.gpu;
    let data = &gpu.data;

    let title = if data.available {
        format!(" GPU ─ {} ", truncate_str(&data.name, 25))
    } else {
        " GPU ─ Not Available ".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(title);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if !data.available {
        let msg = Paragraph::new("No NVIDIA GPU detected")
            .style(Style::default().fg(theme.muted));
        frame.render_widget(msg, inner);
        return;
    }

    let gpu_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Usage gauge
            Constraint::Length(1), // VRAM gauge
            Constraint::Length(1), // Details
        ])
        .split(inner);

    // GPU usage
    let usage_color = theme.usage_color(data.usage_percent);
    let usage_gauge = Gauge::default()
        .gauge_style(Style::default().fg(usage_color).bg(theme.bar_empty))
        .ratio((data.usage_percent as f64 / 100.0).min(1.0))
        .label(format!("GPU: {:.0}%", data.usage_percent));
    frame.render_widget(usage_gauge, gpu_layout[0]);

    // VRAM gauge
    let vram_color = theme.usage_color(data.memory_percent);
    let vram_gauge = Gauge::default()
        .gauge_style(Style::default().fg(vram_color).bg(theme.bar_empty))
        .ratio((data.memory_percent as f64 / 100.0).min(1.0))
        .label(format!(
            "VRAM: {:.0}% ({}/{})",
            data.memory_percent,
            format_bytes(data.memory_used),
            format_bytes(data.memory_total)
        ));
    frame.render_widget(vram_gauge, gpu_layout[1]);

    // Details
    let details = Paragraph::new(format!(
        "{}°C │ Fan {}% │ {}W/{}W │ {} MHz",
        data.temperature,
        data.fan_speed,
        data.power_draw,
        data.power_limit,
        data.clock_core
    )).style(Style::default().fg(theme.muted));
    frame.render_widget(details, gpu_layout[2]);
}

fn render_network(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let net = &app.collectors.network;
    let data = &net.data;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(format!(" NETWORK ─ {} ", data.interface));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let net_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Speeds
            Constraint::Length(1), // Totals
            Constraint::Length(1), // Packets
            Constraint::Min(1),    // Sparkline
        ])
        .split(inner);

    // Speeds
    let speeds = Paragraph::new(format!(
        "↓ {}  ↑ {}",
        format_speed(data.rx_speed),
        format_speed(data.tx_speed)
    )).style(Style::default().fg(theme.primary));
    frame.render_widget(speeds, net_layout[0]);

    // Totals
    let totals = Paragraph::new(format!(
        "Total: ↓ {} │ ↑ {}",
        format_total(data.rx_bytes),
        format_total(data.tx_bytes)
    )).style(Style::default().fg(theme.muted));
    frame.render_widget(totals, net_layout[1]);

    // Packets/Errors
    let packets = Paragraph::new(format!(
        "Pkts: ↓{}K ↑{}K │ Err: {}",
        data.rx_packets / 1000,
        data.tx_packets / 1000,
        data.rx_errors + data.tx_errors
    )).style(Style::default().fg(theme.muted));
    frame.render_widget(packets, net_layout[2]);

    // Sparkline (RX speed)
    let history = net.rx_history.as_vec();
    if !history.is_empty() {
        let max_val = history.iter().max().copied().unwrap_or(1).max(1);
        let sparkline = Sparkline::default()
            .data(&history)
            .max(max_val)
            .style(Style::default().fg(theme.graph_line));
        frame.render_widget(sparkline, net_layout[3]);
    }
}

fn render_disk(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let disk = &app.collectors.disk;
    let data = &disk.data;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(format!(" DISK ─ {} ", data.mount_point));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let disk_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Usage gauge
            Constraint::Length(1), // Details
            Constraint::Min(1),    // I/O
        ])
        .split(inner);

    // Usage gauge
    let usage_color = theme.usage_color(data.usage_percent);
    let usage_gauge = Gauge::default()
        .gauge_style(Style::default().fg(usage_color).bg(theme.bar_empty))
        .ratio((data.usage_percent as f64 / 100.0).min(1.0))
        .label(format!(
            "{:.1}% ({}/{})",
            data.usage_percent,
            format_space(data.used_space),
            format_space(data.total_space)
        ));
    frame.render_widget(usage_gauge, disk_layout[0]);

    // Details
    let details = Paragraph::new(format!(
        "Free: {} │ {}",
        format_space(data.free_space),
        data.name
    )).style(Style::default().fg(theme.muted));
    frame.render_widget(details, disk_layout[1]);

    // I/O (placeholder for now)
    let io_text = Paragraph::new("I/O: monitoring...")
        .style(Style::default().fg(theme.muted));
    frame.render_widget(io_text, disk_layout[2]);
}

fn render_processes(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let proc = &app.collectors.process;
    let data = &proc.data;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(format!(
            " PROCESSES ─ {} │ Thr: {} │ Run: {} │ Zomb: {} ",
            data.total_processes,
            data.total_threads,
            data.running,
            data.zombies
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Split for CPU and Memory tables
    let tables_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(inner);

    // Top CPU table
    let cpu_rows: Vec<Row> = data.top_cpu
        .iter()
        .take(5)
        .map(|p| {
            let color = theme.usage_color(p.cpu_percent);
            Row::new(vec![
                Span::styled(truncate_str(&p.name, 12), Style::default().fg(theme.primary)),
                Span::styled(format!("{:5.1}%", p.cpu_percent), Style::default().fg(color)),
            ])
        })
        .collect();

    let cpu_table = Table::new(cpu_rows, [Constraint::Min(12), Constraint::Length(7)])
        .header(Row::new(vec!["TOP CPU", "%"]).style(Style::default().fg(theme.accent)))
        .block(Block::default());
    frame.render_widget(cpu_table, tables_layout[0]);

    // Top Memory table
    let mem_rows: Vec<Row> = data.top_memory
        .iter()
        .take(5)
        .map(|p| {
            Row::new(vec![
                Span::styled(truncate_str(&p.name, 12), Style::default().fg(theme.primary)),
                Span::styled(format_proc_memory(p.memory_bytes), Style::default().fg(theme.muted)),
            ])
        })
        .collect();

    let mem_table = Table::new(mem_rows, [Constraint::Min(12), Constraint::Length(6)])
        .header(Row::new(vec!["TOP MEM", "SIZE"]).style(Style::default().fg(theme.accent)))
        .block(Block::default());
    frame.render_widget(mem_table, tables_layout[1]);
}

fn render_system_info(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let proc = &app.collectors.process.data;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(" SYSTEM ");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Processes: ", Style::default().fg(theme.muted)),
            Span::styled(format!("{}", proc.total_processes), Style::default().fg(theme.primary)),
            Span::styled(" │ Threads: ", Style::default().fg(theme.muted)),
            Span::styled(format!("{}", proc.total_threads), Style::default().fg(theme.primary)),
        ]),
        Line::from(vec![
            Span::styled("Running: ", Style::default().fg(theme.muted)),
            Span::styled(format!("{}", proc.running), Style::default().fg(theme.success)),
            Span::styled(" │ Sleeping: ", Style::default().fg(theme.muted)),
            Span::styled(format!("{}", proc.sleeping), Style::default().fg(theme.muted)),
        ]),
        Line::from(vec![
            Span::styled("Zombies: ", Style::default().fg(theme.muted)),
            Span::styled(
                format!("{}", proc.zombies),
                Style::default().fg(if proc.zombies > 0 { theme.danger } else { theme.success })
            ),
        ]),
    ]).block(Block::default().padding(ratatui::widgets::Padding::horizontal(1)));

    frame.render_widget(info, inner);
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" [Q]", Style::default().fg(theme.accent)),
        Span::styled("uit ", Style::default().fg(theme.muted)),
        Span::styled("[T]", Style::default().fg(theme.accent)),
        Span::styled("heme ", Style::default().fg(theme.muted)),
        Span::styled("[R]", Style::default().fg(theme.accent)),
        Span::styled("efresh ", Style::default().fg(theme.muted)),
        Span::styled("[H]", Style::default().fg(theme.accent)),
        Span::styled("elp ", Style::default().fg(theme.muted)),
        Span::styled(
            format!("│ {} │ v0.1.0", theme.name),
            Style::default().fg(theme.secondary)
        ),
    ]));

    frame.render_widget(footer, area);
}

/// Truncate string to max length
fn truncate_str(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max-1])
    }
}
