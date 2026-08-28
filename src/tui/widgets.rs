use crate::util::fileinfo::FileInfo;
use crate::util::path_manager::PathManager;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
    },
};

const FOREGROUND: Color = Color::Rgb(226, 228, 220);
const HIGHLIGHT_FOREGROUND: Color = Color::Rgb(245, 89, 5);
const DIR_WIDGET_STYLE: Style = Style::new().fg(Color::Rgb(245, 112, 69));
const PREVIEW_WIDGET_STYLE: Style = Style::new().fg(Color::Rgb(124, 96, 166));

const SCROLL_FORCE: u16 = 1;

#[derive(Debug, Default)]

pub(super) struct PathWidget {
    path: String,
    filter_letter: Option<char>,
}

impl Widget for &PathWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .borders(Borders::BOTTOM)
            .style(DIR_WIDGET_STYLE)
            .title_bottom(Line::from(" Home: Ctrl - H ").right_aligned());

        if let Some(l) = self.filter_letter {
            block = block.title_bottom(Line::from(format!(" Filter: '{l}' ")).centered());
        }

        let p = Line::from(self.path.to_string());

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
        self.path_widget.path = pm.path.clone();
        self.path_widget.filter_letter = pm.get_filter_letter();
        self.list_items = pm
            .get_dir_list()
            .iter()
            .map(|info| ListItem::new(format!("{}  {}", info.file_type.icon(), info.name)))
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
            .highlight_style(Style::new().fg(HIGHLIGHT_FOREGROUND));

        let l = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .split(area);
        self.path_widget.render(l[1], buf);
        StatefulWidget::render(list, l[2], buf, &mut state);
    }
}

#[derive(Debug)]
pub(super) struct PreviewWidget {
    file_info: FileInfo,
    scroll: u16,
    layout_file_preview: Layout,
}

impl Default for PreviewWidget {
    fn default() -> Self {
        PreviewWidget {
            file_info: FileInfo::default(),
            scroll: 0,
            layout_file_preview: Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]),
        }
    }
}

impl PreviewWidget {
    pub(super) fn update(&mut self, pm: &PathManager) {
        self.file_info = pm.get_file_preview();
    }

    pub(super) fn reset_scroll(&mut self) {
        self.scroll = 0;
    }

    pub(super) fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(SCROLL_FORCE);
    }

    pub(super) fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(SCROLL_FORCE);
    }
}

impl Widget for &PreviewWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(PREVIEW_WIDGET_STYLE)
            .title(Line::from(" Toggle: Ctrl - T ").right_aligned())
            .title_bottom(Line::from(" Scroll: PagUp, PagDw ").right_aligned());

        let name_line = Line::from(self.file_info.name.clone());

        let content_par = Paragraph::new(self.file_info.content.clone())
            .block(
                Block::new()
                    .borders(Borders::TOP)
                    .style(PREVIEW_WIDGET_STYLE),
            )
            .style(Style::new().fg(FOREGROUND))
            .scroll((self.scroll, 0));

        let _area = block.inner(area);
        let l = self.layout_file_preview.split(_area);

        block.render(area, buf);
        name_line.render(l[0], buf);
        content_par.render(l[1], buf);
    }
}
