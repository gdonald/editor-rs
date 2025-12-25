use editor_tui::menu::{Menu, MenuAction, MenuItem, MenuState, MenuType};

#[test]
fn test_menu_item_creation() {
    let item = MenuItem::new("Test")
        .with_shortcut("Ctrl+T")
        .with_action(MenuAction::ShowAbout);

    assert_eq!(item.label, "Test");
    assert_eq!(item.shortcut, Some("Ctrl+T".to_string()));
    assert!(item.action.is_some());
    assert!(item.submenu.is_none());
}

#[test]
fn test_menu_item_separator() {
    let separator = MenuItem::new("-");
    assert!(separator.is_separator());

    let normal = MenuItem::new("Normal");
    assert!(!normal.is_separator());
}

#[test]
fn test_menu_creation() {
    let items = vec![
        MenuItem::new("Item 1"),
        MenuItem::new("Item 2"),
        MenuItem::new("Item 3"),
    ];

    let menu = Menu::new("Test Menu", items.clone());
    assert_eq!(menu.title, "Test Menu");
    assert_eq!(menu.items.len(), 3);
    assert_eq!(menu.selected_index, 0);
}

#[test]
fn test_menu_navigation() {
    let items = vec![
        MenuItem::new("Item 1"),
        MenuItem::new("-"),
        MenuItem::new("Item 2"),
        MenuItem::new("Item 3"),
    ];

    let mut menu = Menu::new("Test Menu", items);
    assert_eq!(menu.selected_index, 0);

    menu.move_selection_down();
    assert_eq!(menu.selected_index, 2);

    menu.move_selection_down();
    assert_eq!(menu.selected_index, 3);

    menu.move_selection_down();
    assert_eq!(menu.selected_index, 3);

    menu.move_selection_up();
    assert_eq!(menu.selected_index, 2);

    menu.move_selection_up();
    assert_eq!(menu.selected_index, 0);

    menu.move_selection_up();
    assert_eq!(menu.selected_index, 0);
}

#[test]
fn test_menu_selected_item() {
    let items = vec![
        MenuItem::new("Item 1"),
        MenuItem::new("Item 2"),
        MenuItem::new("Item 3"),
    ];

    let mut menu = Menu::new("Test Menu", items);
    assert_eq!(menu.selected_item().unwrap().label, "Item 1");

    menu.move_selection_down();
    assert_eq!(menu.selected_item().unwrap().label, "Item 2");

    menu.move_selection_down();
    assert_eq!(menu.selected_item().unwrap().label, "Item 3");
}

#[test]
fn test_menu_type_all() {
    let menu_types = MenuType::all();
    assert_eq!(menu_types.len(), 6);
    assert_eq!(menu_types[0], MenuType::File);
    assert_eq!(menu_types[1], MenuType::Edit);
    assert_eq!(menu_types[2], MenuType::View);
    assert_eq!(menu_types[3], MenuType::Search);
    assert_eq!(menu_types[4], MenuType::Tools);
    assert_eq!(menu_types[5], MenuType::Help);
}

#[test]
fn test_menu_type_title() {
    assert_eq!(MenuType::File.title(), "File");
    assert_eq!(MenuType::Edit.title(), "Edit");
    assert_eq!(MenuType::View.title(), "View");
    assert_eq!(MenuType::Search.title(), "Search");
    assert_eq!(MenuType::Tools.title(), "Tools");
    assert_eq!(MenuType::Help.title(), "Help");
}

#[test]
fn test_menu_type_alt_key() {
    assert_eq!(MenuType::File.alt_key(), 'f');
    assert_eq!(MenuType::Edit.alt_key(), 'e');
    assert_eq!(MenuType::View.alt_key(), 'v');
    assert_eq!(MenuType::Search.alt_key(), 's');
    assert_eq!(MenuType::Tools.alt_key(), 't');
    assert_eq!(MenuType::Help.alt_key(), 'h');
}

#[test]
fn test_menu_state_activation() {
    let mut state = MenuState::new();
    assert!(!state.active);
    assert_eq!(state.selected_menu, 0);
    assert!(!state.is_menu_open());

    state.activate();
    assert!(state.active);

    state.deactivate();
    assert!(!state.active);
    assert!(!state.is_menu_open());
}

#[test]
fn test_menu_state_open_close() {
    let mut state = MenuState::new();

    state.activate();
    state.open_current_menu();
    assert!(state.is_menu_open());

    state.close_menu();
    assert!(!state.is_menu_open());
}

#[test]
fn test_menu_state_navigation() {
    let mut state = MenuState::new();
    assert_eq!(state.selected_menu, 0);

    state.move_menu_right();
    assert_eq!(state.selected_menu, 1);

    state.move_menu_right();
    assert_eq!(state.selected_menu, 2);

    state.move_menu_left();
    assert_eq!(state.selected_menu, 1);

    state.move_menu_left();
    assert_eq!(state.selected_menu, 0);

    state.move_menu_left();
    assert_eq!(state.selected_menu, 5);

    state.move_menu_right();
    assert_eq!(state.selected_menu, 0);
}

#[test]
fn test_menu_state_select_by_alt_key() {
    let mut state = MenuState::new();

    state.select_menu_by_alt_key('e');
    assert_eq!(state.selected_menu, 1);
    assert!(state.active);
    assert!(state.is_menu_open());

    state.deactivate();
    state.select_menu_by_alt_key('v');
    assert_eq!(state.selected_menu, 2);
    assert!(state.active);
    assert!(state.is_menu_open());

    state.deactivate();
    state.select_menu_by_alt_key('h');
    assert_eq!(state.selected_menu, 5);
    assert!(state.active);
    assert!(state.is_menu_open());
}

#[test]
fn test_menu_state_get_selected_action() {
    let mut state = MenuState::new();
    assert!(state.get_selected_action().is_none());

    state.activate();
    state.open_current_menu();
    let action = state.get_selected_action();
    assert!(action.is_some());
}
