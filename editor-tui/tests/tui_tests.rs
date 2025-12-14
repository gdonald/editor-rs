use ratatui::{backend::TestBackend, Terminal};

#[test]
fn test_terminal_initialization() {
    let backend = TestBackend::new(80, 24);
    let terminal = Terminal::new(backend);
    assert!(terminal.is_ok());
}

#[test]
fn test_terminal_draw() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let result = terminal.draw(|frame| {
        let area = frame.size();
        assert_eq!(area.width, 80);
        assert_eq!(area.height, 24);
    });

    assert!(result.is_ok());
}

#[test]
fn test_terminal_size() {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            let area = frame.size();
            assert_eq!(area.width, 120);
            assert_eq!(area.height, 30);
        })
        .unwrap();
}
