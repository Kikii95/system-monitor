use ratatui::style::Color;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Theme {
    pub name: &'static str,
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub muted: Color,
    pub border: Color,
    pub bar_filled: Color,
    pub bar_empty: Color,
    pub graph_line: Color,
}

impl Theme {
    /// Get color based on usage percentage
    pub fn usage_color(&self, percent: f32) -> Color {
        match percent as u8 {
            0..=60 => self.success,
            61..=85 => self.warning,
            _ => self.danger,
        }
    }
}

/// Hacker Green theme (default)
pub const THEME_HACKER: Theme = Theme {
    name: "hacker",
    background: Color::Rgb(10, 10, 10),
    foreground: Color::Rgb(0, 255, 65),
    primary: Color::Rgb(0, 255, 65),
    secondary: Color::Rgb(0, 180, 45),
    accent: Color::Rgb(50, 255, 100),
    success: Color::Rgb(0, 255, 136),
    warning: Color::Rgb(255, 200, 0),
    danger: Color::Rgb(255, 50, 50),
    muted: Color::Rgb(80, 100, 80),
    border: Color::Rgb(0, 180, 45),
    bar_filled: Color::Rgb(0, 255, 65),
    bar_empty: Color::Rgb(30, 50, 30),
    graph_line: Color::Rgb(0, 255, 65),
};

/// Matrix theme (darker green)
pub const THEME_MATRIX: Theme = Theme {
    name: "matrix",
    background: Color::Rgb(0, 0, 0),
    foreground: Color::Rgb(0, 200, 0),
    primary: Color::Rgb(0, 200, 0),
    secondary: Color::Rgb(0, 150, 0),
    accent: Color::Rgb(0, 255, 0),
    success: Color::Rgb(0, 200, 100),
    warning: Color::Rgb(200, 200, 0),
    danger: Color::Rgb(200, 0, 0),
    muted: Color::Rgb(0, 80, 0),
    border: Color::Rgb(0, 150, 0),
    bar_filled: Color::Rgb(0, 200, 0),
    bar_empty: Color::Rgb(0, 40, 0),
    graph_line: Color::Rgb(0, 255, 0),
};

/// Minimal theme (monochrome)
pub const THEME_MINIMAL: Theme = Theme {
    name: "minimal",
    background: Color::Rgb(15, 15, 15),
    foreground: Color::Rgb(200, 200, 200),
    primary: Color::Rgb(255, 255, 255),
    secondary: Color::Rgb(150, 150, 150),
    accent: Color::Rgb(255, 255, 255),
    success: Color::Rgb(200, 200, 200),
    warning: Color::Rgb(200, 200, 100),
    danger: Color::Rgb(200, 100, 100),
    muted: Color::Rgb(80, 80, 80),
    border: Color::Rgb(100, 100, 100),
    bar_filled: Color::Rgb(200, 200, 200),
    bar_empty: Color::Rgb(40, 40, 40),
    graph_line: Color::Rgb(200, 200, 200),
};

/// Cyberpunk theme (red/cyan)
pub const THEME_CYBERPUNK: Theme = Theme {
    name: "cyberpunk",
    background: Color::Rgb(10, 10, 15),
    foreground: Color::Rgb(255, 0, 64),
    primary: Color::Rgb(255, 0, 64),
    secondary: Color::Rgb(0, 255, 255),
    accent: Color::Rgb(255, 100, 150),
    success: Color::Rgb(0, 255, 200),
    warning: Color::Rgb(255, 200, 0),
    danger: Color::Rgb(255, 0, 0),
    muted: Color::Rgb(100, 50, 70),
    border: Color::Rgb(255, 0, 64),
    bar_filled: Color::Rgb(255, 0, 64),
    bar_empty: Color::Rgb(50, 20, 30),
    graph_line: Color::Rgb(0, 255, 255),
};

/// All available themes
pub const THEMES: &[Theme] = &[
    THEME_HACKER,
    THEME_MATRIX,
    THEME_MINIMAL,
    THEME_CYBERPUNK,
];
