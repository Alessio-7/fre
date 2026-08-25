use crate::util::path_manager::PathManager;
use crate::util::reader::FileType;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

const FOREGROUND : Color = Color::Rgb(226, 228, 220);
const HIGHLIGHT_FOREGROUND : Color = Color::Rgb(245, 89, 5);
const DIR_WIDGET_STYLE : Style = Style::new().bold().fg(Color::Rgb(245, 112, 69));
const PREVIEW_WIDGET_STYLE : Style = Style::new().bold().fg(Color::Rgb(124, 96, 166));

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
            .map(|(ftype, fname)| 
            ListItem::new(format!("{}  {}", ftype.icon(), fname)))
            .collect();
        self.list_state.select(Some(pm.selected_index));
        //TODO levare quel clone
    }
}

impl Widget for &DirWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.list_state.clone();
        let list = List::new(self.list_items.iter().cloned())
            .style(Style::new().fg(FOREGROUND))
            .highlight_symbol("|>")
            .highlight_style(Style::new().fg(   HIGHLIGHT_FOREGROUND));

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(DIR_WIDGET_STYLE);

        let _area = block.inner(area);
        block.render(area, buf);

        StatefulWidget::render(list, _area, buf, &mut state);
    }
}

#[derive(Debug, Default)]
pub(super) struct PreviewWidget {
    file_preview: (FileType, String),
}

impl PreviewWidget {
    pub(super) fn update(&mut self, pm: &PathManager) {
        self.file_preview = pm.get_file_preview()
    }
}

impl Widget for &PreviewWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(PREVIEW_WIDGET_STYLE);

        let _area = block.inner(area);
        block.render(area, buf);
        
        Paragraph::new(self.file_preview.1.to_string())
        .style(Style::new().fg(FOREGROUND))
        .render(_area, buf);
    }
}
