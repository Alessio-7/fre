use crate::util::path_manager::PathManager;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};
use std::fs::FileType;

#[derive(Debug, Default)]
pub(super) struct DirWidget {
    dir_list: *const Vec<(FileType, String)>,
}

impl DirWidget {
    pub(super) fn update(&mut self, pm: &PathManager) {
        self.dir_list = pm.get_dir_list()
    }
}

impl Widget for &DirWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Dir widget").render(area, buf);
    }
}

#[derive(Debug, Default)]
pub(super) struct PreviewWidget {
    file_preview: String,
}

impl PreviewWidget {
    pub(super) fn update(&mut self, pm: &PathManager) {
        self.file_preview = pm.get_file_preview().1
    }
}

impl Widget for &PreviewWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Preview widget").render(area, buf);
    }
}
