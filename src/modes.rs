pub struct StateObject {
    pub menu_popup: bool,
    pub theme_popup: bool,
    pub menu_selected: usize,
    pub location_selected: usize,
    pub menu_items: Vec<String>,
    pub theme_items: Vec<String>,
}

impl StateObject {
    pub fn new() -> Self {
        Self {
            menu_popup: false,
            theme_popup: false,
            menu_selected: 0,
            location_selected: 0,
            menu_items: vec![
                "Settings".to_string(),
                "About".to_string(),
                "Help".to_string(),
            ],
            theme_items: vec![
                "Dayfox".to_string(),
                "Tokyonight night".to_string(),
                "Catpuccin".to_string(),
                "Everforest".to_string(),
            ],
        }
    }

    pub fn toggle_menu(&mut self) {
        self.menu_popup = !self.menu_popup
    }

    pub fn toggle_location(&mut self) {
        self.theme_popup = !self.theme_popup
    }

    pub fn move_selection_up(&mut self) {
        if self.menu_popup && self.menu_selected > 0 {
            self.menu_selected -= 1;
        } else if self.theme_popup && self.location_selected > 0 {
            self.location_selected -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.menu_popup && self.menu_selected < self.menu_items.len() - 1 {
            self.menu_selected += 1;
        } else if self.theme_popup && self.location_selected < self.theme_items.len() - 1 {
            self.location_selected += 1;
        }
    }

    pub fn get_selected_item(&self) -> Option<&str> {
        if self.menu_popup {
            self.menu_items.get(self.menu_selected).map(|s| s.as_str())
        } else if self.theme_popup {
            self.theme_items
                .get(self.location_selected)
                .map(|s| s.as_str())
        } else {
            None
        }
    }
}
