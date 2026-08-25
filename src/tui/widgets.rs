use crate::util::path_manager::PathManager;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

#[derive(Debug, Default)]
pub(super) struct DirWidget {
    list_items: Vec<ListItem<'static>>,
    list_state: ListState,
}

impl DirWidget {
    pub(super) fn update(&mut self, pm: &PathManager) {
        self.list_items = pm
            .get_dir_list()
            .iter()
            .map(|(_, s)| ListItem::new(s.to_string()))
            .collect();
        self.list_state.select(Some(pm.selected_index));
        //TODO levare quel clone
    }
}

impl Widget for &DirWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.list_state.clone();
        let list = List::new(self.list_items.iter().cloned()).highlight_symbol(">");
        StatefulWidget::render(list, area, buf, &mut state);
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
