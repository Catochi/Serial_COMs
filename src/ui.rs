use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Wrap},
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::app::{App, CHANNELS, Focus};

const BG: Color = Color::Rgb(20, 23, 34);
const PANEL: Color = Color::Rgb(27, 31, 46);
const TEXT: Color = Color::Rgb(218, 224, 240);
const MUTED: Color = Color::Rgb(126, 139, 166);
const ACCENT: Color = Color::Rgb(125, 211, 192);

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    frame.render_widget(
        Block::default().style(Style::default().bg(BG).fg(TEXT)),
        area,
    );
    if area.width < 44 || area.height < 12 {
        frame.render_widget(
            Paragraph::new("Please resize the terminal to at least 44 x 12.")
                .wrap(Wrap { trim: true }),
            area,
        );
        return;
    }

    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .margin(1)
    .areas(area);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                " SERIAL / COMS ",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  a terminal workspace", Style::default().fg(MUTED)),
        ])),
        header,
    );

    let [sidebar, main] = Layout::horizontal([
        Constraint::Length(if area.width < 70 { 18 } else { 24 }),
        Constraint::Min(0),
    ])
    .spacing(1)
    .areas(body);
    draw_sidebar(frame, app, sidebar);
    draw_chat(frame, app, main);
    frame.render_widget(
        Paragraph::new(" Tab focus  ↑/↓ channel  Enter send  Esc sidebar  Ctrl+C quit")
            .style(Style::default().fg(MUTED)),
        footer,
    );
}

fn panel(title: &str, focused: bool) -> Block<'_> {
    Block::bordered()
        .title(title)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if focused {
            ACCENT
        } else {
            Color::Rgb(61, 70, 94)
        }))
        .style(Style::default().bg(PANEL).fg(TEXT))
}

fn draw_sidebar(frame: &mut Frame, app: &App, area: Rect) {
    let [channels, note] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(4)]).areas(area);
    let items = CHANNELS
        .iter()
        .map(|name| ListItem::new(format!(" # {name}")));
    let list = List::new(items)
        .block(panel(" Workspace ", app.focus == Focus::Sidebar))
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(43, 65, 72))
                .fg(ACCENT)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▎");
    frame.render_stateful_widget(
        list,
        channels,
        &mut ListState::default().with_selected(Some(app.selected)),
    );
    frame.render_widget(
        Paragraph::new(" LOCAL DEMO\n Messages live in memory.\n No device connected.")
            .style(Style::default().fg(MUTED))
            .wrap(Wrap { trim: false }),
        note,
    );
}

fn draw_chat(frame: &mut Frame, app: &App, area: Rect) {
    let [history, input] = Layout::vertical([Constraint::Min(0), Constraint::Length(3)])
        .spacing(1)
        .areas(area);
    let mut lines = Vec::new();
    for message in &app.conversations[app.selected] {
        let color = match message.author {
            "you" => ACCENT,
            "system" => MUTED,
            _ => Color::Rgb(184, 166, 244),
        };
        lines.push(Line::from(Span::styled(
            format!(" {}", message.author),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(format!(" {}", message.text)));
        lines.push(Line::default());
    }
    let title = format!(" # {} ", CHANNELS[app.selected]);
    let block = panel(&title, false);
    let inner = block.inner(history);
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
    let scroll = paragraph
        .line_count(inner.width)
        .saturating_sub(inner.height as usize);
    frame.render_widget(
        paragraph
            .block(block)
            .scroll((scroll.min(u16::MAX as usize) as u16, 0)),
        history,
    );

    let focused = app.focus == Focus::Input;
    let block = panel(
        if focused {
            " Message · Enter to send "
        } else {
            " Message · Tab to compose "
        },
        focused,
    );
    let inner = block.inner(input);
    // Keep the end of the input visible, reserving a cell for the cursor.
    let available = inner.width.saturating_sub(1) as usize;
    let mut width = 0;
    let mut start = app.input.len();
    for (index, c) in app.input.char_indices().rev() {
        let char_width = c.width().unwrap_or(0);
        if width + char_width > available {
            break;
        }
        width += char_width;
        start = index;
    }
    let visible = &app.input[start..];
    let (content, color) = if app.input.is_empty() && !focused {
        ("Write something…", MUTED)
    } else {
        (visible, TEXT)
    };
    frame.render_widget(
        Paragraph::new(content)
            .style(Style::default().fg(color))
            .block(block),
        input,
    );
    if focused && inner.width > 0 && inner.height > 0 {
        frame.set_cursor_position((inner.x + visible.width() as u16, inner.y));
    }
}
