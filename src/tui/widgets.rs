use crate::util::path_manager::PathManager;
use crate::util::reader::FileType;
use ratatui::{
    buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Style}, widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, StatefulWidget, Widget, canvas::Label},
};

const FOREGROUND : Color = Color::Rgb(226, 228, 220);
const HIGHLIGHT_FOREGROUND : Color = Color::Rgb(245, 89, 5);
const DIR_WIDGET_STYLE : Style = Style::new().bold().fg(Color::Rgb(245, 112, 69));
//const PREVIEW_WIDGET_STYLE : Style = Style::new().bold().fg(Color::Rgb(124, 96, 166));

#[derive(Debug, Default)]

pub(super) struct PathWidget{
    path: String
}

impl PathWidget{
    fn update(&mut self, path: &String){
        self.path=path.to_string();
    }
}

impl Widget for &PathWidget{
    fn render(self, area: Rect, buf: &mut Buffer){
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(DIR_WIDGET_STYLE);
        let p = Paragraph::new(self.path.to_string());

        let _area = block.inner(area);
        block.render(area, buf);
        p.render(_area, buf);
        
    }
}

#[derive(Debug, Default)]
pub(super) struct DirWidget {
    list_items: Vec<ListItem<'static>>,
    list_state: ListState,
    path_widget: PathWidget,
}

impl DirWidget {
    pub(super) fn update(&mut self, pm: &PathManager) {
        self.path_widget.update(&pm.path);
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

        let l = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)])
                            .split(area);

        /*
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(DIR_WIDGET_STYLE);

        let _area = block.inner(area);
        block.render(area, buf);
        StatefulWidget::render(list, _area, buf, &mut state);
        */
        StatefulWidget::render(list, l[0], buf, &mut state);
        self.path_widget.render(l[1], buf);
    }
}

#[derive(Debug, Default)]
pub(super) struct PreviewWidget {
    file_preview: (FileType, String),
}

impl PreviewWidget {
    pub(super) fn update(&mut self, pm: &PathManager) {
        self.file_preview = pm.get_file_preview()
        //self.file_preview = (FileType::Dir, pm.path.clone());
    }
}

impl Widget for &PreviewWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        /*
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(PREVIEW_WIDGET_STYLE);

        let _area = block.inner(area);
        block.render(area, buf);
        
        Paragraph::new(self.file_preview.1.to_string())
        .style(Style::new().fg(FOREGROUND))
        .render(_area, buf);
         */
        Paragraph::new(self.file_preview.1.to_string())
        .style(Style::new().fg(FOREGROUND))
        .render(area, buf);
    }
}
