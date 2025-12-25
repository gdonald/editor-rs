use editor_core::Command;

#[derive(Debug, Clone)]
pub enum MenuAction {
    ExecuteCommand(Command),
    OpenFile,
    SaveAs,
    Quit,
    Search,
    Replace,
    GotoLine,
    SelectAll,
    ShowHelp,
    ShowAbout,
    ToggleLineNumbers,
    ToggleStatusBar,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub shortcut: Option<String>,
    pub action: Option<MenuAction>,
    pub submenu: Option<Vec<MenuItem>>,
}

impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            action: None,
            submenu: None,
        }
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn with_action(mut self, action: MenuAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_submenu(mut self, submenu: Vec<MenuItem>) -> Self {
        self.submenu = Some(submenu);
        self
    }

    pub fn is_separator(&self) -> bool {
        self.label == "-"
    }
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub title: String,
    pub items: Vec<MenuItem>,
    pub selected_index: usize,
}

impl Menu {
    pub fn new(title: impl Into<String>, items: Vec<MenuItem>) -> Self {
        Self {
            title: title.into(),
            items,
            selected_index: 0,
        }
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            while self.selected_index > 0 && self.items[self.selected_index].is_separator() {
                self.selected_index -= 1;
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.items.len().saturating_sub(1) {
            self.selected_index += 1;
            while self.selected_index < self.items.len() - 1
                && self.items[self.selected_index].is_separator()
            {
                self.selected_index += 1;
            }
        }
    }

    pub fn selected_item(&self) -> Option<&MenuItem> {
        self.items.get(self.selected_index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuType {
    File,
    Edit,
    View,
    Search,
    Tools,
    Help,
}

impl MenuType {
    pub fn all() -> Vec<MenuType> {
        vec![
            MenuType::File,
            MenuType::Edit,
            MenuType::View,
            MenuType::Search,
            MenuType::Tools,
            MenuType::Help,
        ]
    }

    pub fn title(&self) -> &str {
        match self {
            MenuType::File => "File",
            MenuType::Edit => "Edit",
            MenuType::View => "View",
            MenuType::Search => "Search",
            MenuType::Tools => "Tools",
            MenuType::Help => "Help",
        }
    }

    pub fn alt_key(&self) -> char {
        match self {
            MenuType::File => 'f',
            MenuType::Edit => 'e',
            MenuType::View => 'v',
            MenuType::Search => 's',
            MenuType::Tools => 't',
            MenuType::Help => 'h',
        }
    }
}

#[derive(Debug, Clone)]
pub struct MenuState {
    pub active: bool,
    pub selected_menu: usize,
    pub open_menu: Option<Menu>,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            active: false,
            selected_menu: 0,
            open_menu: None,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.selected_menu = 0;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.open_menu = None;
    }

    pub fn is_menu_open(&self) -> bool {
        self.open_menu.is_some()
    }

    pub fn open_current_menu(&mut self) {
        let menu_types = MenuType::all();
        if let Some(menu_type) = menu_types.get(self.selected_menu) {
            self.open_menu = Some(create_menu(*menu_type));
        }
    }

    pub fn close_menu(&mut self) {
        self.open_menu = None;
    }

    pub fn move_menu_left(&mut self) {
        if self.selected_menu > 0 {
            self.selected_menu -= 1;
        } else {
            self.selected_menu = MenuType::all().len() - 1;
        }
        if self.is_menu_open() {
            self.open_current_menu();
        }
    }

    pub fn move_menu_right(&mut self) {
        let menu_count = MenuType::all().len();
        if self.selected_menu < menu_count - 1 {
            self.selected_menu += 1;
        } else {
            self.selected_menu = 0;
        }
        if self.is_menu_open() {
            self.open_current_menu();
        }
    }

    pub fn select_menu_by_alt_key(&mut self, key: char) {
        let menu_types = MenuType::all();
        for (idx, menu_type) in menu_types.iter().enumerate() {
            if menu_type.alt_key() == key {
                self.active = true;
                self.selected_menu = idx;
                self.open_current_menu();
                break;
            }
        }
    }

    pub fn get_selected_action(&self) -> Option<MenuAction> {
        self.open_menu
            .as_ref()
            .and_then(|menu| menu.selected_item())
            .and_then(|item| item.action.clone())
    }
}

impl Default for MenuState {
    fn default() -> Self {
        Self::new()
    }
}

fn create_menu(menu_type: MenuType) -> Menu {
    match menu_type {
        MenuType::File => Menu::new(
            "File",
            vec![
                MenuItem::new("New")
                    .with_shortcut("Ctrl+N")
                    .with_action(MenuAction::ExecuteCommand(Command::New)),
                MenuItem::new("Open")
                    .with_shortcut("Ctrl+O")
                    .with_action(MenuAction::OpenFile),
                MenuItem::new("-"),
                MenuItem::new("Save")
                    .with_shortcut("Ctrl+S")
                    .with_action(MenuAction::ExecuteCommand(Command::Save)),
                MenuItem::new("Save As").with_action(MenuAction::SaveAs),
                MenuItem::new("-"),
                MenuItem::new("Close")
                    .with_shortcut("Ctrl+W")
                    .with_action(MenuAction::ExecuteCommand(Command::Close)),
                MenuItem::new("Quit")
                    .with_shortcut("Ctrl+Q")
                    .with_action(MenuAction::Quit),
            ],
        ),
        MenuType::Edit => Menu::new(
            "Edit",
            vec![
                MenuItem::new("Undo")
                    .with_shortcut("Ctrl+Z")
                    .with_action(MenuAction::ExecuteCommand(Command::Undo)),
                MenuItem::new("Redo")
                    .with_shortcut("Ctrl+Y")
                    .with_action(MenuAction::ExecuteCommand(Command::Redo)),
                MenuItem::new("-"),
                MenuItem::new("Cut")
                    .with_shortcut("Ctrl+X")
                    .with_action(MenuAction::ExecuteCommand(Command::Cut)),
                MenuItem::new("Copy")
                    .with_shortcut("Ctrl+C")
                    .with_action(MenuAction::ExecuteCommand(Command::Copy)),
                MenuItem::new("Paste")
                    .with_shortcut("Ctrl+V")
                    .with_action(MenuAction::ExecuteCommand(Command::Paste)),
                MenuItem::new("-"),
                MenuItem::new("Select All")
                    .with_shortcut("Ctrl+A")
                    .with_action(MenuAction::SelectAll),
            ],
        ),
        MenuType::View => Menu::new(
            "View",
            vec![
                MenuItem::new("Toggle Line Numbers").with_action(MenuAction::ToggleLineNumbers),
                MenuItem::new("Toggle Status Bar").with_action(MenuAction::ToggleStatusBar),
            ],
        ),
        MenuType::Search => Menu::new(
            "Search",
            vec![
                MenuItem::new("Find")
                    .with_shortcut("Ctrl+F")
                    .with_action(MenuAction::Search),
                MenuItem::new("Replace")
                    .with_shortcut("Ctrl+H")
                    .with_action(MenuAction::Replace),
                MenuItem::new("-"),
                MenuItem::new("Go to Line")
                    .with_shortcut("Ctrl+G")
                    .with_action(MenuAction::GotoLine),
                MenuItem::new("Next Match")
                    .with_shortcut("F3")
                    .with_action(MenuAction::ExecuteCommand(Command::NextMatch)),
                MenuItem::new("Previous Match")
                    .with_shortcut("Shift+F3")
                    .with_action(MenuAction::ExecuteCommand(Command::PreviousMatch)),
            ],
        ),
        MenuType::Tools => Menu::new(
            "Tools",
            vec![
                MenuItem::new("Toggle Comment")
                    .with_shortcut("Ctrl+/")
                    .with_action(MenuAction::ExecuteCommand(Command::ToggleLineComment)),
                MenuItem::new("Toggle Block Comment")
                    .with_shortcut("Ctrl+Shift+/")
                    .with_action(MenuAction::ExecuteCommand(Command::ToggleBlockComment)),
                MenuItem::new("-"),
                MenuItem::new("Sort Lines").with_action(MenuAction::ExecuteCommand(
                    Command::SortLines { numerical: false },
                )),
                MenuItem::new("Duplicate Line")
                    .with_shortcut("Ctrl+D")
                    .with_action(MenuAction::ExecuteCommand(Command::DuplicateLine)),
                MenuItem::new("Delete Line")
                    .with_shortcut("Ctrl+K")
                    .with_action(MenuAction::ExecuteCommand(Command::DeleteLine)),
                MenuItem::new("Join Lines")
                    .with_shortcut("Ctrl+J")
                    .with_action(MenuAction::ExecuteCommand(Command::JoinLines)),
            ],
        ),
        MenuType::Help => Menu::new(
            "Help",
            vec![
                MenuItem::new("Keyboard Shortcuts").with_action(MenuAction::ShowHelp),
                MenuItem::new("-"),
                MenuItem::new("About").with_action(MenuAction::ShowAbout),
            ],
        ),
    }
}
