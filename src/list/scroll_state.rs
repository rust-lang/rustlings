pub struct ScrollState {
    n_rows: usize,
    max_n_rows_to_display: usize,
    selected: Option<usize>,
    offset: usize,
    scroll_padding: usize,
    max_scroll_padding: usize,
}

impl ScrollState {
    pub fn new(n_rows: usize, selected: Option<usize>, max_scroll_padding: usize) -> Self {
        Self {
            n_rows,
            max_n_rows_to_display: 0,
            selected,
            offset: selected.map_or(0, |selected| selected.saturating_sub(max_scroll_padding)),
            scroll_padding: 0,
            max_scroll_padding,
        }
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    fn update_offset(&mut self) {
        let Some(selected) = self.selected else {
            return;
        };

        let min_offset = (selected + self.scroll_padding)
            .saturating_sub(self.max_n_rows_to_display.saturating_sub(1));
        let max_offset = selected.saturating_sub(self.scroll_padding);
        let global_max_offset = self.n_rows.saturating_sub(self.max_n_rows_to_display);

        self.offset = self
            .offset
            .max(min_offset)
            .min(max_offset)
            .min(global_max_offset);
    }

    #[inline]
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn set_selected(&mut self, selected: usize) {
        self.selected = Some(selected);
        self.update_offset();
    }

    pub fn select_next(&mut self) {
        if let Some(selected) = self.selected {
            self.set_selected((selected + 1).min(self.n_rows - 1));
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(selected) = self.selected {
            self.set_selected(selected.saturating_sub(1));
        }
    }

    pub fn select_first(&mut self) {
        if self.n_rows > 0 {
            self.set_selected(0);
        }
    }

    pub fn select_last(&mut self) {
        if self.n_rows > 0 {
            self.set_selected(self.n_rows - 1);
        }
    }

    pub fn set_n_rows(&mut self, n_rows: usize) {
        self.n_rows = n_rows;

        if self.n_rows == 0 {
            self.selected = None;
            return;
        }

        self.set_selected(self.selected.map_or(0, |selected| selected.min(n_rows - 1)));
    }

    #[inline]
    fn update_scroll_padding(&mut self) {
        self.scroll_padding = (self.max_n_rows_to_display / 4).min(self.max_scroll_padding);
    }

    #[inline]
    pub fn max_n_rows_to_display(&self) -> usize {
        self.max_n_rows_to_display
    }

    pub fn set_max_n_rows_to_display(&mut self, max_n_rows_to_display: usize) {
        self.max_n_rows_to_display = max_n_rows_to_display;
        self.update_scroll_padding();
        self.update_offset();
    }
}
