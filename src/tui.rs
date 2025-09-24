use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{io, path::Path};
use crate::discovery::{DirAnalyzer, LargeFileFinder, DevArtifactFinder, FileItem};
use crate::utils::format_size;

#[derive(Debug, Clone)]
enum MenuOption {
    ListDirectories,
    FindLargeFiles,
    FindDevArtifacts,
    DockerCleanup,
    TempCleanup,
    Exit,
}

impl MenuOption {
    fn as_str(&self) -> &str {
        match self {
            MenuOption::ListDirectories => "📁 List directories by size",
            MenuOption::FindLargeFiles => "🔍 Find large files",
            MenuOption::FindDevArtifacts => "🛠️  Find development artifacts",
            MenuOption::DockerCleanup => "🐳 Docker cleanup",
            MenuOption::TempCleanup => "🗂️  Temporary files cleanup",
            MenuOption::Exit => "❌ Exit",
        }
    }
}

struct App {
    menu_state: ListState,
    menu_options: Vec<MenuOption>,
    current_view: AppView,
    items: Vec<FileItem>,
    items_state: ListState,
    message: Option<String>,
    show_help: bool,
}

#[derive(Debug, Clone)]
enum AppView {
    Menu,
    DirectoryList,
    LargeFiles,
    DevArtifacts,
    Loading,
}

impl App {
    fn new() -> App {
        let mut app = App {
            menu_state: ListState::default(),
            menu_options: vec![
                MenuOption::ListDirectories,
                MenuOption::FindLargeFiles,
                MenuOption::FindDevArtifacts,
                MenuOption::DockerCleanup,
                MenuOption::TempCleanup,
                MenuOption::Exit,
            ],
            current_view: AppView::Menu,
            items: Vec::new(),
            items_state: ListState::default(),
            message: None,
            show_help: false,
        };
        app.menu_state.select(Some(0));
        app
    }

    fn next_menu_item(&mut self) {
        let selected = match self.menu_state.selected() {
            Some(i) => {
                if i >= self.menu_options.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(selected));
    }

    fn previous_menu_item(&mut self) {
        let selected = match self.menu_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.menu_options.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(selected));
    }

    fn next_item(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let selected = match self.items_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.items_state.select(Some(selected));
    }

    fn previous_item(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let selected = match self.items_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.items_state.select(Some(selected));
    }

    async fn execute_menu_action(&mut self) -> Result<bool> {
        if let Some(selected) = self.menu_state.selected() {
            match &self.menu_options[selected] {
                MenuOption::ListDirectories => {
                    self.current_view = AppView::Loading;
                    self.load_directories().await?;
                    self.current_view = AppView::DirectoryList;
                    self.items_state.select(Some(0));
                }
                MenuOption::FindLargeFiles => {
                    self.current_view = AppView::Loading;
                    self.load_large_files().await?;
                    self.current_view = AppView::LargeFiles;
                    self.items_state.select(Some(0));
                }
                MenuOption::FindDevArtifacts => {
                    self.current_view = AppView::Loading;
                    self.load_dev_artifacts().await?;
                    self.current_view = AppView::DevArtifacts;
                    self.items_state.select(Some(0));
                }
                MenuOption::DockerCleanup => {
                    self.message = Some("Docker cleanup functionality requires CLI mode. Use: safe-clean docker".to_string());
                }
                MenuOption::TempCleanup => {
                    self.message = Some("Temp cleanup functionality requires CLI mode. Use: safe-clean temp".to_string());
                }
                MenuOption::Exit => {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    async fn load_directories(&mut self) -> Result<()> {
        let analyzer = DirAnalyzer::new();
        self.items = analyzer.analyze_directory(Path::new("."), true).await?;
        Ok(())
    }

    async fn load_large_files(&mut self) -> Result<()> {
        let finder = LargeFileFinder::new();
        self.items = finder.find_large_files(Path::new("."), 100 * 1024 * 1024).await?; // 100MB threshold
        Ok(())
    }

    async fn load_dev_artifacts(&mut self) -> Result<()> {
        let finder = DevArtifactFinder::new();
        self.items = finder.find_artifacts(Path::new(".")).await?;
        Ok(())
    }

    fn back_to_menu(&mut self) {
        self.current_view = AppView::Menu;
        self.items.clear();
        self.items_state = ListState::default();
        self.message = None;
    }

    fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}

pub async fn run() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let result = run_app(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => app.toggle_help(),
                    KeyCode::Esc => {
                        if matches!(app.current_view, AppView::Menu) {
                            break;
                        } else {
                            app.back_to_menu();
                        }
                    }
                    KeyCode::Enter => {
                        match app.current_view {
                            AppView::Menu => {
                                if app.execute_menu_action().await? {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Up => {
                        match app.current_view {
                            AppView::Menu => app.previous_menu_item(),
                            _ => app.previous_item(),
                        }
                    }
                    KeyCode::Down => {
                        match app.current_view {
                            AppView::Menu => app.next_menu_item(),
                            _ => app.next_item(),
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Header
    let header = Paragraph::new("Safe Clean - Disk Cleanup Tool")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Main content
    match app.current_view {
        AppView::Menu => render_menu(f, app, chunks[1]),
        AppView::DirectoryList => render_items_list(f, app, chunks[1], "Directories by Size"),
        AppView::LargeFiles => render_items_list(f, app, chunks[1], "Large Files"),
        AppView::DevArtifacts => render_items_list(f, app, chunks[1], "Development Artifacts"),
        AppView::Loading => render_loading(f, chunks[1]),
    }

    // Footer
    let footer_text = if app.show_help {
        "ESC: Back/Exit | ↑↓: Navigate | Enter: Select | h: Toggle Help | q: Quit"
    } else {
        "h: Help | q: Quit"
    };
    
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);

    // Show message popup if any
    if let Some(message) = &app.message {
        render_message_popup(f, message);
    }
}

fn render_menu(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .menu_options
        .iter()
        .map(|option| ListItem::new(option.as_str()))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Main Menu"))
        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("► ");

    f.render_stateful_widget(list, area, &mut app.menu_state);
}

fn render_items_list(f: &mut Frame, app: &mut App, area: Rect, title: &str) {
    if app.items.is_empty() {
        let paragraph = Paragraph::new("No items found.")
            .block(Block::default().borders(Borders::ALL).title(title))
            .alignment(Alignment::Center);
        f.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|item| {
            let path_str = item.path.to_string_lossy();
            let display_path = if path_str.len() > 60 {
                format!("...{}", &path_str[path_str.len()-57..])
            } else {
                path_str.to_string()
            };
            
            let size_str = format_size(item.size);
            let line = if let Some(count) = item.item_count {
                format!("{:<60} {:>10} {:>8} items", display_path, size_str, count)
            } else {
                format!("{:<60} {:>10}", display_path, size_str)
            };
            
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("► ");

    f.render_stateful_widget(list, area, &mut app.items_state);
}

fn render_loading(f: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new("Loading... Please wait.")
        .block(Block::default().borders(Borders::ALL).title("Processing"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_message_popup(f: &mut Frame, message: &str) {
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);
    
    let paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Information"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}