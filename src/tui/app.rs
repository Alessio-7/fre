use super::widgets::{DirWidget, PreviewWidget, TextInputWidget};
use crate::util::{path_manager::PathManager, reader};
use color_eyre::{
    Result,
    eyre::{Error, eyre},
};
use crokey::{Combiner, KeyCombination, key};
use crossterm::event::{self, Event, read as read_input};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Position},
    style::Stylize,
    widgets::{Block, Paragraph},
};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct App {
    combiner: Combiner,
    exit: bool,
    path_manager: PathManager,
    layout: Layout,
    dir_widget: DirWidget,
    prev_widget: PreviewWidget,
    toggle_preview: bool,
    keybindings: HashMap<KeyCombination, String>,
    in_text_input_mode: bool,
    text_input_widget: TextInputWidget,
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
            keybindings: HashMap::new(),
            in_text_input_mode: false,
            text_input_widget: TextInputWidget::default(),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, path: String) -> Result<()> {
        self.load_configs();
        self.path_manager.load_path(path)?;
        self.update_widgets();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Err(error) = self.handle_events() {
                terminal.draw(|frame| self.draw_error(frame, error))?;
                while !event::read()?.as_key_press_event().is_some() {}
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

        let par = Paragraph::new(message);
        frame.render_widget(par, block.inner(popup_area));
        frame.render_widget(block, popup_area);
    }

    fn draw(&mut self, frame: &mut Frame) {
        if !self.toggle_preview {
            let l = self.layout.split(frame.area());
            frame.render_widget(&self.dir_widget, l[0]);
            frame.render_widget(&self.prev_widget, l[1]);
        } else {
            frame.render_widget(&self.dir_widget, frame.area());
        }

        if self.in_text_input_mode {
            frame.render_widget(&mut self.text_input_widget, frame.area());
            frame.set_cursor_position(Position {
                x: self.text_input_widget.cursor_x,
                y: self.text_input_widget.cursor_y,
            });
        }
    }

    fn update_widgets(&mut self) {
        self.dir_widget.update(&self.path_manager);
        if !self.toggle_preview {
            self.prev_widget.update(&self.path_manager);
        }
    }

    fn load_configs(&mut self) {
        let mut load = |conf: serde_json::Value| -> Result<()> {
            //TODO appeareance settings
            let Some(_a) = conf["appearence"].as_object() else {
                return Err(eyre!("Failed to load appearence config"));
            };
            let Some(k) = conf["keybindings"].as_object() else {
                return Err(eyre!("Failed to load keybindings config"));
            };

            for (keyc, command) in k {
                if let Some(command) = command.as_str()
                    && let Ok(k) = KeyCombination::from_str(keyc)
                {
                    self.keybindings.insert(k, command.to_string());
                }
            }

            Ok(())
        };

        reader::read_config()
            .and_then(&mut load)
            .unwrap_or_else(|_| {
                load(reader::default_config()).expect("Failed to load default config")
            });
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Ok(Event::Key(key_event)) = read_input()
            && let Some(key_combination) = self.combiner.transform(key_event)
        {
            let mut do_update = true;
            if self.in_text_input_mode {
                if key_combination == key!(esc) {
                    self.in_text_input_mode = false;
                } else if self.text_input_widget.key_event(key_combination) {
                    self.in_text_input_mode = false;
                    self.path_manager
                        .rename_selected(self.text_input_widget.text.clone())?;
                }
            } else if self.keybindings.contains_key(&key_combination) {
                match self.keybindings[&key_combination].as_str() {
                    "exit" => self.exit = true,
                    "reload" => self.load_configs(),
                    "select_first" => self.path_manager.select_first(),
                    "select_last" => self.path_manager.select_last(),

                    "select_previous" => {
                        self.prev_widget.reset_scroll();
                        self.path_manager.select_previous();
                    }
                    "select_next" => {
                        self.prev_widget.reset_scroll();
                        self.path_manager.select_next();
                    }
                    "go_out" => self.path_manager.go_out()?,
                    "go_into" => self.path_manager.go_into()?,

                    "select_multiple" => return Err(eyre!("IMPLEMENT")),
                    "select_all" => return Err(eyre!("IMPLEMENT")),

                    "cut" => self.path_manager.cut_selected()?,
                    "copy" => self.path_manager.copy_selected()?,
                    "paste" => self.path_manager.paste_selected()?,
                    "new" => return Err(eyre!("IMPLEMENT")),
                    "delete" => self.path_manager.delete_selected()?,
                    "rename" => {
                        self.in_text_input_mode = true;
                        self.text_input_widget.update(
                            "Rename File:".to_string(),
                            self.path_manager.get_selected_name()?,
                        );
                    }
                    "home" => self
                        .path_manager
                        .load_path(std::env::var("HOME").unwrap())?,
                    "open" => self.path_manager.open_selected()?,
                    "go_into_or_open" => self.path_manager.go_into_or_open_selected()?,
                    "open_on_explorer" => self.path_manager.open_selected_on_explorer()?,

                    "toggle_preview" => self.toggle_preview = !self.toggle_preview,
                    "scroll_down_preview" => self.prev_widget.scroll_down(),
                    "scroll_up_preview" => self.prev_widget.scroll_up(),

                    "clear_filter" => self.path_manager.clear_filter(),
                    _ => return Err(eyre!("IMPLEMENT CUSTOM COMMAND")),
                }
            } else if let Some(letter) = key_combination.as_letter() {
                self.path_manager.filter_dir_list_word(letter);
            } else {
                do_update = false;
            }

            if do_update {
                self.update_widgets();
            }
        }
        Ok(())
    }
}
