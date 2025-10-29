use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

use crate::config::FileConfigRepository;
use crate::error::FugaResult;
use crate::services::StandardFileSystemService;
use crate::traits::{ConfigRepository, FileSystemService};

const HELP_TEXT: &str = "Key Bindings\n\n  q / Ctrl+c    Quit dashboard\n  m / Space     Toggle mark on selection\n  c             Exit and run copy\n  v             Exit and run move\n  s             Exit and run link\n  Arrow keys    Navigate file list\n  j / k         Navigate file list\n  h / Backspace Go to parent directory\n  l / Enter     Enter selected directory\n  . / Ctrl+h    Toggle hidden files\n  /             Start incremental filter\n  Ctrl+l        Clear active filter\n  Ctrl+r / R    Reset mark list (with confirm)\n  ? / F1        Toggle this help";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DashboardExit {
    Quit,
    Copy(PathBuf),
    Move(PathBuf),
    Link(PathBuf),
}

struct TerminalGuard;

impl TerminalGuard {
    fn activate() -> FugaResult<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
        execute!(io::stdout(), cursor::Hide)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), cursor::Show);
        let _ = execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

struct DirEntryData {
    name: String,
    abs_path: String,
    is_dir: bool,
    is_hidden: bool,
}

#[derive(Clone)]
struct StatusMessage {
    text: String,
    is_error: bool,
}

enum Confirmation {
    ResetMarks,
}

struct DashboardApp<'a> {
    config_repo: &'a dyn ConfigRepository,
    fs_service: &'a dyn FileSystemService,
    current_dir: PathBuf,
    entries: Vec<DirEntryData>,
    visible_indices: Vec<usize>,
    list_state: ListState,
    selection: usize,
    show_hidden: bool,
    filter_input: String,
    filter_mode: bool,
    marks: Vec<String>,
    status: Option<StatusMessage>,
    confirmation: Option<Confirmation>,
    help_open: bool,
}

impl<'a> DashboardApp<'a> {
    fn new(
        config_repo: &'a dyn ConfigRepository,
        fs_service: &'a dyn FileSystemService,
    ) -> FugaResult<Self> {
        let current_dir = std::env::current_dir()?;
        let mut app = Self {
            config_repo,
            fs_service,
            current_dir,
            entries: Vec::new(),
            visible_indices: Vec::new(),
            list_state: ListState::default(),
            selection: 0,
            show_hidden: false,
            filter_input: String::new(),
            filter_mode: false,
            marks: Vec::new(),
            status: None,
            confirmation: None,
            help_open: false,
        };
        app.refresh_marks()?;
        app.reload_directory()?;
        Ok(app)
    }

    fn refresh_marks(&mut self) -> FugaResult<()> {
        self.marks = self.config_repo.get_marked_targets()?;
        Ok(())
    }

    fn reload_directory(&mut self) -> FugaResult<()> {
        self.entries = Self::read_directory(&self.current_dir)?;
        self.rebuild_visible();
        Ok(())
    }

    fn read_directory(dir: &PathBuf) -> FugaResult<Vec<DirEntryData>> {
        let mut entries = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let is_dir = file_type.is_dir();
            let file_name = entry.file_name().to_string_lossy().into_owned();
            let abs_path = entry.path().to_string_lossy().into_owned();
            let is_hidden = file_name.starts_with('.');
            entries.push(DirEntryData {
                name: file_name,
                abs_path,
                is_dir,
                is_hidden,
            });
        }

        entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => a
                .name
                .to_ascii_lowercase()
                .cmp(&b.name.to_ascii_lowercase()),
        });

        Ok(entries)
    }

    fn rebuild_visible(&mut self) {
        self.visible_indices.clear();
        let filter = self.filter_input.clone();
        for (idx, entry) in self.entries.iter().enumerate() {
            if !self.show_hidden && entry.is_hidden {
                continue;
            }
            if !filter.is_empty() && !fuzzy_match(&entry.name, &filter) {
                continue;
            }
            self.visible_indices.push(idx);
        }

        if self.visible_indices.is_empty() {
            self.selection = 0;
            self.list_state.select(None);
        } else {
            if self.selection >= self.visible_indices.len() {
                self.selection = self.visible_indices.len() - 1;
            }
            self.list_state.select(Some(self.selection));
        }
    }

    fn draw(&mut self, frame: &mut Frame<'_>) {
        let size = frame.size();
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(1)])
            .split(size);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(vertical[0]);

        let browser_title = format!("File Browser — {}", self.current_dir.to_string_lossy());
        let entries = self.visible_list_items();

        let browser_block = Block::default().title(browser_title).borders(Borders::ALL);
        frame.render_stateful_widget(
            List::new(entries)
                .block(browser_block)
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::Black)),
            horizontal[0],
            &mut self.list_state,
        );

        let marks_block = Block::default()
            .title("Marked Targets")
            .borders(Borders::ALL);
        let marks_lines: Vec<Line> = if self.marks.is_empty() {
            vec![Line::styled(
                "No targets marked",
                Style::default().fg(Color::DarkGray),
            )]
        } else {
            self.marks
                .iter()
                .enumerate()
                .map(|(idx, path)| {
                    let style = if idx % 2 == 0 {
                        Style::default().fg(Color::White).bg(Color::Rgb(28, 36, 52))
                    } else {
                        Style::default().fg(Color::Black).bg(Color::Blue)
                    };
                    Line::styled(path.clone(), style)
                })
                .collect()
        };
        frame.render_widget(
            Paragraph::new(marks_lines)
                .block(marks_block)
                .wrap(Wrap { trim: true }),
            horizontal[1],
        );

        let status_line = self.status_line();
        frame.render_widget(status_line, vertical[1]);

        if self.help_open {
            let area = centered_rect(70, 70, size);
            frame.render_widget(Clear, area);
            frame.render_widget(
                Paragraph::new(HELP_TEXT)
                    .block(Block::default().title("Help").borders(Borders::ALL))
                    .wrap(Wrap { trim: false }),
                area,
            );
        }
    }

    fn visible_list_items(&self) -> Vec<ListItem<'static>> {
        self.visible_indices
            .iter()
            .filter_map(|idx| self.entries.get(*idx))
            .map(|entry| {
                let marker = if self.marks.iter().any(|m| m == &entry.abs_path) {
                    '*'
                } else {
                    ' '
                };
                let type_label = if entry.is_dir { "DIR" } else { "FILE" };
                let line = format!("[{}] {:<4} {}", marker, type_label, entry.name);
                let style = if entry.is_dir {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                };
                ListItem::new(line).style(style)
            })
            .collect()
    }

    fn status_line(&self) -> Paragraph<'_> {
        let base = if self.filter_mode {
            format!("Filter: {}_", self.filter_input)
        } else if matches!(self.confirmation, Some(Confirmation::ResetMarks)) {
            "Reset marks? [y/N]".to_string()
        } else {
            "[q] quit  [m]/[space] mark  [c] copy  [v] move  [s] link  [/] filter  [Ctrl+l] clear filter  [.] hidden  [?] help"
                .to_string()
        };

        let mut spans = vec![Span::styled(base, Style::default().fg(Color::Gray))];
        if let Some(status) = &self.status {
            spans.push(Span::raw("  |  "));
            let style = if status.is_error {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            };
            spans.push(Span::styled(status.text.clone(), style));
        }

        Paragraph::new(Line::from(spans))
    }

    fn visible_selection(&self) -> Option<&DirEntryData> {
        self.visible_indices
            .get(self.selection)
            .and_then(|idx| self.entries.get(*idx))
    }

    fn handle_event(&mut self) -> FugaResult<Option<DashboardExit>> {
        if event::poll(Duration::from_millis(150))? {
            let evt = event::read()?;
            match evt {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    return self.on_key(key);
                }
                _ => {}
            }
        }
        Ok(None)
    }

    fn on_key(&mut self, key: KeyEvent) -> FugaResult<Option<DashboardExit>> {
        if self.help_open {
            match key.code {
                KeyCode::Char('?') | KeyCode::F(1) | KeyCode::Esc => {
                    self.help_open = false;
                }
                _ => {}
            }
            return Ok(None);
        }

        if let Some(Confirmation::ResetMarks) = self.confirmation {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.execute_reset_marks()?;
                    self.confirmation = None;
                }
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                    self.status = Some(StatusMessage {
                        text: "Reset cancelled".to_string(),
                        is_error: false,
                    });
                    self.confirmation = None;
                }
                _ => {}
            }
            return Ok(None);
        }

        if self.filter_mode {
            match key.code {
                KeyCode::Esc => {
                    self.filter_mode = false;
                    self.filter_input.clear();
                    self.rebuild_visible();
                }
                KeyCode::Enter => {
                    self.filter_mode = false;
                }
                KeyCode::Backspace => {
                    self.filter_input.pop();
                    self.rebuild_visible();
                }
                KeyCode::Char(ch) => {
                    if !key.modifiers.contains(KeyModifiers::CONTROL) {
                        self.filter_input.push(ch);
                        self.rebuild_visible();
                    }
                }
                _ => {}
            }
            return Ok(None);
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('c') => return Ok(Some(DashboardExit::Quit)),
                KeyCode::Char('h') => {
                    self.toggle_hidden();
                    return Ok(None);
                }
                KeyCode::Char('r') => {
                    self.request_reset();
                    return Ok(None);
                }
                KeyCode::Char('l') => {
                    self.clear_filter();
                    return Ok(None);
                }
                _ => {}
            }
        }

        match key.code {
            KeyCode::Char('q') => return Ok(Some(DashboardExit::Quit)),
            KeyCode::Char('c') => return Ok(Some(DashboardExit::Copy(self.current_dir.clone()))),
            KeyCode::Char('v') => return Ok(Some(DashboardExit::Move(self.current_dir.clone()))),
            KeyCode::Char('s') => return Ok(Some(DashboardExit::Link(self.current_dir.clone()))),
            KeyCode::Char('?') | KeyCode::F(1) => {
                self.help_open = true;
            }
            KeyCode::Esc => {
                return Ok(Some(DashboardExit::Quit));
            }
            KeyCode::Char('/') => {
                self.filter_mode = true;
                self.filter_input.clear();
            }
            KeyCode::Char('.') => {
                self.toggle_hidden();
            }
            KeyCode::Char('m') | KeyCode::Char(' ') => {
                self.toggle_mark()?;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.move_selection(1);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.move_selection(-1);
            }
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                self.enter_directory()?;
            }
            KeyCode::Char('h') | KeyCode::Left | KeyCode::Backspace => {
                self.go_parent()?;
            }
            KeyCode::Char('R') => {
                self.request_reset();
            }
            _ => {}
        }

        Ok(None)
    }

    fn move_selection(&mut self, delta: isize) {
        if self.visible_indices.is_empty() {
            self.list_state.select(None);
            return;
        }
        let len = self.visible_indices.len() as isize;
        let mut new_sel = self.selection as isize + delta;
        if new_sel < 0 {
            new_sel = 0;
        } else if new_sel >= len {
            new_sel = len - 1;
        }
        self.selection = new_sel as usize;
        self.list_state.select(Some(self.selection));
    }

    fn enter_directory(&mut self) -> FugaResult<()> {
        if let Some(entry) = self.visible_selection() {
            if entry.is_dir {
                let target_path = entry.abs_path.clone();
                self.current_dir = PathBuf::from(&target_path);
                self.filter_input.clear();
                self.filter_mode = false;
                self.reload_directory()?;
                self.status = Some(StatusMessage {
                    text: format!("Entered {}", target_path),
                    is_error: false,
                });
            }
        }
        Ok(())
    }

    fn go_parent(&mut self) -> FugaResult<()> {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.filter_input.clear();
            self.filter_mode = false;
            self.reload_directory()?;
            self.status = Some(StatusMessage {
                text: format!("Entered {}", self.current_dir.to_string_lossy()),
                is_error: false,
            });
        }
        Ok(())
    }

    fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.rebuild_visible();
        self.status = Some(StatusMessage {
            text: if self.show_hidden {
                "Hidden files shown".to_string()
            } else {
                "Hidden files hidden".to_string()
            },
            is_error: false,
        });
    }

    fn clear_filter(&mut self) {
        if self.filter_input.is_empty() {
            self.status = Some(StatusMessage {
                text: "Filter already cleared".to_string(),
                is_error: false,
            });
            return;
        }

        self.filter_mode = false;
        self.filter_input.clear();
        self.rebuild_visible();
        self.status = Some(StatusMessage {
            text: "Filter cleared".to_string(),
            is_error: false,
        });
    }

    fn toggle_mark(&mut self) -> FugaResult<()> {
        let entry = match self.visible_selection() {
            Some(entry) => entry,
            None => return Ok(()),
        };

        if entry.abs_path.is_empty() {
            return Ok(());
        }

        let abs_path = entry.abs_path.clone();

        let target_exists = self.fs_service.get_file_info(&abs_path)?.exists;
        if !target_exists {
            self.status = Some(StatusMessage {
                text: "Cannot mark missing path".to_string(),
                is_error: true,
            });
            return Ok(());
        }

        if self.marks.iter().any(|m| m == &abs_path) {
            self.remove_mark(&abs_path)?;
            self.status = Some(StatusMessage {
                text: format!("Removed mark {}", abs_path),
                is_error: false,
            });
        } else {
            self.add_mark(&abs_path)?;
            self.status = Some(StatusMessage {
                text: format!("Marked {}", abs_path),
                is_error: false,
            });
        }

        self.refresh_marks()?;
        Ok(())
    }

    fn add_mark(&mut self, abs_path: &str) -> FugaResult<()> {
        let mut marks = self.config_repo.get_marked_targets()?;
        if marks.iter().any(|m| m == abs_path) {
            return Ok(());
        }
        marks.push(abs_path.to_string());
        dedupe_preserving_order(&mut marks);
        self.config_repo.set_marked_targets(&marks)?;
        Ok(())
    }

    fn remove_mark(&mut self, abs_path: &str) -> FugaResult<()> {
        let mut marks = self.config_repo.get_marked_targets()?;
        marks.retain(|m| m != abs_path);
        self.config_repo.reset_marks()?;
        if !marks.is_empty() {
            dedupe_preserving_order(&mut marks);
            self.config_repo.set_marked_targets(&marks)?;
        }
        Ok(())
    }

    fn request_reset(&mut self) {
        self.confirmation = Some(Confirmation::ResetMarks);
        self.status = None;
    }

    fn execute_reset_marks(&mut self) -> FugaResult<()> {
        self.config_repo.reset_marks()?;
        self.refresh_marks()?;
        self.status = Some(StatusMessage {
            text: "Marks cleared".to_string(),
            is_error: false,
        });
        Ok(())
    }
}

pub fn run_dashboard(
    config_repo: &FileConfigRepository,
    fs_service: &StandardFileSystemService,
) -> FugaResult<DashboardExit> {
    let _guard = TerminalGuard::activate()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = DashboardApp::new(config_repo, fs_service)?;
    let outcome = loop {
        terminal.draw(|frame| app.draw(frame))?;
        if let Some(exit) = app.handle_event()? {
            break exit;
        }
    };

    terminal.show_cursor()?;
    Ok(outcome)
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}

fn fuzzy_match(candidate: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }
    let mut chars = pattern.chars().map(|c| c.to_ascii_lowercase());
    let mut current = match chars.next() {
        Some(c) => c,
        None => return true,
    };

    for ch in candidate.chars() {
        if ch.to_ascii_lowercase() == current {
            match chars.next() {
                Some(next) => current = next,
                None => return true,
            }
        }
    }

    false
}

fn dedupe_preserving_order(values: &mut Vec<String>) {
    let mut seen = HashSet::new();
    values.retain(|value| seen.insert(value.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::fuga::{FileInfo, TargetType};
    use crate::traits::{ConfigRepository, FileSystemService};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ratatui::widgets::ListState;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    #[derive(Default)]
    struct StubConfigRepository {
        marks: RefCell<Vec<String>>,
    }

    impl StubConfigRepository {
        fn current_marks(&self) -> Vec<String> {
            self.marks.borrow().clone()
        }
    }

    impl ConfigRepository for StubConfigRepository {
        fn load_config(&self) -> FugaResult<AppConfig> {
            Ok(AppConfig::default())
        }

        fn store_config(&self, _config: &AppConfig) -> FugaResult<()> {
            Ok(())
        }

        fn set_marked_targets(&self, targets: &[String]) -> FugaResult<()> {
            *self.marks.borrow_mut() = targets.to_vec();
            Ok(())
        }

        fn get_marked_targets(&self) -> FugaResult<Vec<String>> {
            Ok(self.marks.borrow().clone())
        }

        fn reset_marks(&self) -> FugaResult<()> {
            self.marks.borrow_mut().clear();
            Ok(())
        }
    }

    #[derive(Default)]
    struct StubFileSystemService {
        existing: RefCell<HashSet<String>>,
    }

    impl StubFileSystemService {
        fn register_existing<S: Into<String>>(&self, path: S) {
            self.existing.borrow_mut().insert(path.into());
        }
    }

    impl FileSystemService for StubFileSystemService {
        fn get_file_info(&self, path: &str) -> FugaResult<FileInfo> {
            let exists = self.existing.borrow().contains(path);
            let name = Path::new(path)
                .file_name()
                .map(|value| value.to_string_lossy().into_owned());
            Ok(FileInfo {
                exists,
                is_file: true,
                is_dir: false,
                name,
            })
        }

        fn get_abs_path(&self, path: &str) -> FugaResult<String> {
            Ok(path.to_string())
        }

        fn get_file_type(&self, _path: &str) -> TargetType {
            TargetType::File
        }

        fn copy_items(&self, _src: &str, _dst: &str) -> FugaResult<()> {
            panic!("copy_items should not be invoked in tests");
        }

        fn move_items(&self, _src: &str, _dst: &str) -> FugaResult<()> {
            panic!("move_items should not be invoked in tests");
        }

        fn link_items(&self, _src: &str, _dst: &str) -> FugaResult<()> {
            panic!("link_items should not be invoked in tests");
        }
    }

    fn make_entry(base: &Path, file_name: &str) -> (DirEntryData, String) {
        let abs_path = base.join(file_name);
        let abs_str = abs_path.to_string_lossy().into_owned();
        let entry = DirEntryData {
            name: file_name.to_string(),
            abs_path: abs_str.clone(),
            is_dir: false,
            is_hidden: false,
        };
        (entry, abs_str)
    }

    fn build_app<'a>(
        config: &'a StubConfigRepository,
        fs: &'a StubFileSystemService,
        current_dir: PathBuf,
        entry: DirEntryData,
    ) -> DashboardApp<'a> {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        DashboardApp {
            config_repo: config as &dyn ConfigRepository,
            fs_service: fs as &dyn FileSystemService,
            current_dir,
            entries: vec![entry],
            visible_indices: vec![0],
            list_state,
            selection: 0,
            show_hidden: false,
            filter_input: String::new(),
            filter_mode: false,
            marks: config.current_marks(),
            status: None,
            confirmation: None,
            help_open: false,
        }
    }

    #[test]
    fn fuzzy_match_matches_subsequence_in_order() {
        assert!(fuzzy_match("notes.txt", "nt"));
        assert!(!fuzzy_match("notes.txt", "tn"));
    }

    #[test]
    fn dedupe_preserving_order_removes_duplicate_entries() {
        let mut values = vec![
            "/tmp/a".to_string(),
            "/tmp/b".to_string(),
            "/tmp/a".to_string(),
            "/tmp/c".to_string(),
            "/tmp/b".to_string(),
        ];
        dedupe_preserving_order(&mut values);
        assert_eq!(values, vec!["/tmp/a", "/tmp/b", "/tmp/c"]);
    }

    #[test]
    fn clear_filter_resets_query_and_selection() {
        let config = StubConfigRepository::default();
        let fs = StubFileSystemService::default();
        let temp_dir = tempdir().unwrap();
        let current_dir = temp_dir.path().to_path_buf();
        let (entry, entry_path) = make_entry(temp_dir.path(), "notes.txt");
        fs.register_existing(entry_path);

        let mut app = build_app(&config, &fs, current_dir, entry);
        app.filter_input = "note".to_string();
        app.filter_mode = true;
        app.rebuild_visible();

        app.clear_filter();

        assert!(app.filter_input.is_empty());
        assert!(!app.filter_mode);
        assert_eq!(app.visible_indices, vec![0]);
        assert_eq!(
            app.status.as_ref().map(|status| status.text.as_str()),
            Some("Filter cleared")
        );
    }

    #[test]
    fn toggle_mark_adds_and_removes_targets() {
        let config = StubConfigRepository::default();
        let fs = StubFileSystemService::default();
        let temp_dir = tempdir().unwrap();
        let current_dir = temp_dir.path().to_path_buf();
        let (entry, entry_path) = make_entry(temp_dir.path(), "notes.txt");
        fs.register_existing(entry_path.clone());

        let mut app = build_app(&config, &fs, current_dir, entry);
        app.rebuild_visible();

        app.toggle_mark().expect("mark toggle should succeed");
        assert_eq!(config.current_marks(), vec![entry_path.clone()]);
        assert_eq!(app.marks, vec![entry_path.clone()]);
        assert!(app
            .status
            .as_ref()
            .and_then(|status| status.text.strip_prefix("Marked"))
            .is_some());

        app.toggle_mark().expect("mark removal should succeed");
        assert!(config.current_marks().is_empty());
        assert!(app.marks.is_empty());
        assert!(app
            .status
            .as_ref()
            .and_then(|status| status.text.strip_prefix("Removed mark"))
            .is_some());
    }

    #[test]
    fn copy_key_uses_current_directory_for_exit() {
        let config = StubConfigRepository::default();
        let fs = StubFileSystemService::default();
        let temp_dir = tempdir().unwrap();
        let current_dir = temp_dir.path().to_path_buf();
        let expected_dir = current_dir.clone();
        let (entry, entry_path) = make_entry(temp_dir.path(), "notes.txt");
        fs.register_existing(entry_path);

        let mut app = build_app(&config, &fs, current_dir, entry);
        let exit = app
            .on_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE))
            .expect("on_key should succeed")
            .expect("copy should trigger exit");

        assert_eq!(exit, DashboardExit::Copy(expected_dir));
    }
}
