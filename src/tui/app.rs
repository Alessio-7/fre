use super::widgets::{DirWidget, PreviewWidget};
use crate::util::path_manager::PathManager;
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{DefaultTerminal, Frame, layout::Constraint, layout::Layout};

#[derive(Debug)]
pub struct App {
    exit: bool,

    path_manager: PathManager,

    layout: Layout,
    dir_widget: DirWidget,
    prev_widget: PreviewWidget,
}

impl Default for App {
    fn default() -> Self {
        App {
            exit: false,
            path_manager: PathManager::default(),
            layout: Layout::horizontal([Constraint::Fill(2), Constraint::Fill(1)]),
            dir_widget: DirWidget::default(),
            prev_widget: PreviewWidget::default(),
        }
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal, path: String) -> Result<()> {
        self.path_manager.load_path(path);

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let l = self.layout.split(frame.area());
        frame.render_widget(&self.dir_widget, l[0]);
        frame.render_widget(&self.prev_widget, l[1]);
    }

    fn update_widgets(&mut self) {
        self.dir_widget.update(&self.path_manager);
        self.prev_widget.update(&self.path_manager);
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Up => {
                    self.path_manager.up();
                    self.update_widgets();
                }
                KeyCode::Down => {
                    self.path_manager.down();
                    self.update_widgets();
                }
                KeyCode::Left => {
                    self.path_manager.left();
                    self.update_widgets();
                }
                KeyCode::Right => {
                    self.path_manager.right();
                    self.update_widgets();
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.exit = true;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
