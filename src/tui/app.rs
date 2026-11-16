use core::panic;
use std::{collections::HashMap, str::FromStr};

use super::widgets::{DirWidget, PreviewWidget};
use crate::util::{path_manager::PathManager, reader};
use ansi_to_tui::IntoText;
use color_eyre::{Result, eyre::{Error, eyre}};
use crokey::{Combiner, KeyCombination};
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
    keybindings: HashMap<KeyCombination, String>
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
            keybindings: HashMap::new()
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
                && let Ok(k) = KeyCombination::from_str(keyc){
                    self.keybindings.insert(k, command.to_string());
                }
            }

            Ok(())
        };

        reader::read_config().and_then(&mut load)
        .unwrap_or_else( |_|
            load(reader::default_config()).expect("Failed to load default config")
        );
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Ok(Event::Key(key_event)) = read_input()
            && let Some(key_combination) = self.combiner.transform(key_event) {
                let mut do_update = true;

                if self.keybindings.contains_key(&key_combination){
                    match self.keybindings[&key_combination].as_str() {
                        "exit" => {
                            self.exit = true;
                        }
                        "reload" => {
                            panic!("IMPLEMENT RELOAD");
                        }
                        "select_first" => {
                            self.path_manager.select_first();
                        }
                        "select_last" => {
                            self.path_manager.select_last();
                        }

                        "select_previous" => {
                            self.prev_widget.reset_scroll();
                            self.path_manager.select_previous();
                        }
                        "selet_next" => {
                            self.prev_widget.reset_scroll();
                            self.path_manager.select_next();
                        }
                        "go_out" => {
                            self.path_manager.go_out()?;
                        }
                        "go_into" => {
                            self.path_manager.go_into()?;
                        }

                        "select_multiple" => {
                            panic!("IMPLEMENT")
                        }
                        "select_all" => {
                            panic!("IMPLEMENT")
                        }

                        "cut" =>{
                            panic!("IMPLEMENT")
                        }
                        "copy" =>{
                            panic!("IMPLEMENT")
                        }
                        "paste" =>{
                            panic!("IMPLEMENT")
                        }
                        "new" =>{
                            panic!("IMPLEMENT")
                        }
                        "delete" =>{
                            panic!("IMPLEMENT")
                        }

                        "home" => {
                            self.path_manager.load_path(std::env::var("HOME").unwrap())?;
                        }
                        "open" => {
                            self.path_manager.open_selected()?;
                        }
                        "go_into_or_open" => {
                            self.path_manager.go_into_or_open_selected()?;
                        }
                        "open_on_explorer" => {
                            self.path_manager.open_selected_on_explorer()?;
                        }

                        "toggle_preview" => {
                            self.toggle_preview = !self.toggle_preview;
                        }
                        "scroll_down_preview" => {
                            self.prev_widget.scroll_down();
                        }
                        "scroll_up_preview" => {
                            self.prev_widget.scroll_up();
                        }

                        "clear_filter" => {
                            self.path_manager.clear_filter();
                        }
                        _ => {
                            panic!("IMPLEMENT CUSTOM COMMAND")
                        }
                    }
                }else if let Some(letter) = key_combination.as_letter() {
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
