use super::widgets::{DirWidget, PreviewWidget};
use crate::util::path_manager::PathManager;
use ansi_to_tui::IntoText;
use color_eyre::{Result, eyre::Error};
use crokey::{Combiner, key};
use crossterm::event::{self, Event, read as read_input};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Paragraph},
};

#[derive(Debug)]
pub struct App {
    combiner: Combiner,
    exit: bool,
    path_manager: PathManager,
    layout: Layout,
    dir_widget: DirWidget,
    prev_widget: PreviewWidget,
    toggle_preview: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            combiner: Combiner::default(),
            exit: false,
            path_manager: PathManager::default(),
            layout: Layout::horizontal([Constraint::Ratio(2, 5), Constraint::Fill(1)]),
            dir_widget: DirWidget::default(),
            prev_widget: PreviewWidget::default(),
            toggle_preview: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, path: String) -> Result<()> {
        self.path_manager.load_path(path)?;
        self.update_widgets();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match self.handle_events() {
                Err(error) => {
                    terminal.draw(|frame| self.draw_error(frame, error))?;
                    while !event::read()?.as_key_press_event().is_some() {}
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn draw_error(&self, frame: &mut Frame, e: Error) {
        let popup_area = frame
            .area()
            .centered(Constraint::Ratio(1, 2), Constraint::Ratio(1, 2));
        let block = Block::bordered().on_black().title("Error");

        let message = format!("Attempted Path: {}\n{}", self.path_manager.path, e);

        let par = Paragraph::new(message.as_bytes().into_text().unwrap());
        frame.render_widget(par, block.inner(popup_area));
        frame.render_widget(block, popup_area);
    }

    fn draw(&self, frame: &mut Frame) {
        if !self.toggle_preview {
            let l = self.layout.split(frame.area());
            frame.render_widget(&self.dir_widget, l[0]);
            frame.render_widget(&self.prev_widget, l[1]);
        } else {
            frame.render_widget(&self.dir_widget, frame.area());
        }
    }

    fn update_widgets(&mut self) {
        self.dir_widget.update(&self.path_manager);
        if !self.toggle_preview {
            self.prev_widget.update(&self.path_manager);
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match read_input() {
            Ok(Event::Key(key_event)) => {
                if let Some(key_combination) = self.combiner.transform(key_event) {
                    let mut do_update = true;
                    match key_combination {
                        key!(ctrl - q) => {
                            self.exit = true;
                        }
                        key!(home) => {
                            self.path_manager.select_first();
                        }
                        key!(end) => {
                            self.path_manager.select_last();
                        }
                        key!(up) => {
                            self.prev_widget.reset_scroll();
                            self.path_manager.select_previous();
                        }
                        key!(down) => {
                            self.prev_widget.reset_scroll();
                            self.path_manager.select_next();
                        }
                        key!(left) => {
                            self.path_manager.go_out()?;
                        }
                        key!(right) => {
                            self.path_manager.go_into()?;
                        }
                        key!(ctrl - h) => {
                            self.path_manager
                                .load_path(std::env::var("HOME").unwrap())?;
                        }
                        key!(enter) => {
                            self.path_manager.open_selected()?;
                        }
                        key!(ctrl - t) => {
                            self.toggle_preview = !self.toggle_preview;
                        }
                        key!(pagedown) => {
                            self.prev_widget.scroll_down();
                        }
                        key!(pageup) => {
                            self.prev_widget.scroll_up();
                        }
                        key!(esc) | key!(backspace) => {
                            self.path_manager.clear_filter();
                        }
                        _ => {
                            if let Some(letter) = key_combination.as_letter() {
                                self.path_manager.filter_dir_list(letter);
                            } else {
                                do_update = false;
                            }
                        }
                    }
                    if do_update {
                        self.update_widgets();
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
