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

/// Dracula theme (purple/pink)
pub const THEME_DRACULA: Theme = Theme {
    name: "dracula",
    background: Color::Rgb(40, 42, 54),
    foreground: Color::Rgb(248, 248, 242),
    primary: Color::Rgb(189, 147, 249),
    secondary: Color::Rgb(255, 121, 198),
    accent: Color::Rgb(139, 233, 253),
    success: Color::Rgb(80, 250, 123),
    warning: Color::Rgb(255, 184, 108),
    danger: Color::Rgb(255, 85, 85),
    muted: Color::Rgb(98, 114, 164),
    border: Color::Rgb(68, 71, 90),
    bar_filled: Color::Rgb(189, 147, 249),
    bar_empty: Color::Rgb(68, 71, 90),
    graph_line: Color::Rgb(255, 121, 198),
};

/// Nord theme (arctic blue)
pub const THEME_NORD: Theme = Theme {
    name: "nord",
    background: Color::Rgb(46, 52, 64),
    foreground: Color::Rgb(216, 222, 233),
    primary: Color::Rgb(136, 192, 208),
    secondary: Color::Rgb(129, 161, 193),
    accent: Color::Rgb(163, 190, 140),
    success: Color::Rgb(163, 190, 140),
    warning: Color::Rgb(235, 203, 139),
    danger: Color::Rgb(191, 97, 106),
    muted: Color::Rgb(76, 86, 106),
    border: Color::Rgb(67, 76, 94),
    bar_filled: Color::Rgb(136, 192, 208),
    bar_empty: Color::Rgb(59, 66, 82),
    graph_line: Color::Rgb(129, 161, 193),
};

/// Gruvbox theme (retro warm)
pub const THEME_GRUVBOX: Theme = Theme {
    name: "gruvbox",
    background: Color::Rgb(40, 40, 40),
    foreground: Color::Rgb(235, 219, 178),
    primary: Color::Rgb(250, 189, 47),
    secondary: Color::Rgb(215, 153, 33),
    accent: Color::Rgb(184, 187, 38),
    success: Color::Rgb(152, 151, 26),
    warning: Color::Rgb(254, 128, 25),
    danger: Color::Rgb(251, 73, 52),
    muted: Color::Rgb(146, 131, 116),
    border: Color::Rgb(80, 73, 69),
    bar_filled: Color::Rgb(250, 189, 47),
    bar_empty: Color::Rgb(60, 56, 54),
    graph_line: Color::Rgb(215, 153, 33),
};

/// Tokyo Night theme (deep blue/purple)
pub const THEME_TOKYO: Theme = Theme {
    name: "tokyo",
    background: Color::Rgb(26, 27, 38),
    foreground: Color::Rgb(169, 177, 214),
    primary: Color::Rgb(122, 162, 247),
    secondary: Color::Rgb(187, 154, 247),
    accent: Color::Rgb(125, 207, 255),
    success: Color::Rgb(158, 206, 106),
    warning: Color::Rgb(224, 175, 104),
    danger: Color::Rgb(247, 118, 142),
    muted: Color::Rgb(86, 95, 137),
    border: Color::Rgb(41, 46, 66),
    bar_filled: Color::Rgb(122, 162, 247),
    bar_empty: Color::Rgb(41, 46, 66),
    graph_line: Color::Rgb(187, 154, 247),
};

/// Ocean theme (deep sea blue)
pub const THEME_OCEAN: Theme = Theme {
    name: "ocean",
    background: Color::Rgb(15, 25, 35),
    foreground: Color::Rgb(200, 220, 240),
    primary: Color::Rgb(0, 180, 230),
    secondary: Color::Rgb(0, 140, 190),
    accent: Color::Rgb(100, 220, 255),
    success: Color::Rgb(0, 220, 180),
    warning: Color::Rgb(255, 200, 80),
    danger: Color::Rgb(255, 80, 100),
    muted: Color::Rgb(80, 110, 140),
    border: Color::Rgb(40, 70, 100),
    bar_filled: Color::Rgb(0, 180, 230),
    bar_empty: Color::Rgb(25, 45, 65),
    graph_line: Color::Rgb(100, 220, 255),
};

/// All available themes
pub const THEMES: &[Theme] = &[
    THEME_HACKER,
    THEME_MATRIX,
    THEME_MINIMAL,
    THEME_CYBERPUNK,
    THEME_DRACULA,
    THEME_NORD,
    THEME_GRUVBOX,
    THEME_TOKYO,
    THEME_OCEAN,
];
