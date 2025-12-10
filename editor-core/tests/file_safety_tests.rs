use editor_core::buffer::{Buffer, LineEnding};
use editor_core::error::EditorError;
use std::fs;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_auto_save_enabled() {
    let mut buffer = Buffer::new();
    assert!(!buffer.auto_save_enabled());

    buffer.set_auto_save(true);
    assert!(buffer.auto_save_enabled());

    buffer.set_auto_save(false);
    assert!(!buffer.auto_save_enabled());
}

#[test]
fn test_last_saved_tracking() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "Hello, world!").unwrap();
    let buffer = Buffer::from_file(file_path.clone()).unwrap();

    assert!(buffer.last_saved().is_some());
    let initial_time = buffer.last_saved().unwrap();

    sleep(Duration::from_millis(10));

    let mut buffer = buffer;
    buffer.insert_char(0, 0, 'X').unwrap();
    buffer.save().unwrap();

    assert!(buffer.last_saved().is_some());
    let new_time = buffer.last_saved().unwrap();
    assert!(new_time > initial_time);
}

#[test]
fn test_create_backup() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "Hello, world!").unwrap();
    let buffer = Buffer::from_file(file_path.clone()).unwrap();

    let backup_path = buffer.create_backup().unwrap();
    assert!(backup_path.exists());
    assert_eq!(
        backup_path.extension().and_then(|e| e.to_str()),
        Some("backup")
    );

    let backup_content = fs::read_to_string(&backup_path).unwrap();
    assert_eq!(backup_content, "Hello, world!");
}

#[test]
fn test_create_backup_no_file_path() {
    let buffer = Buffer::new();
    let result = buffer.create_backup();
    assert!(result.is_err());
    match result.unwrap_err() {
        EditorError::InvalidOperation(_) => {}
        _ => panic!("Expected InvalidOperation error"),
    }
}

#[test]
fn test_save_recovery_data() {
    let temp_dir = TempDir::new().unwrap();
    let recovery_dir = temp_dir.path().join("recovery");
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "Hello, world!").unwrap();
    let buffer = Buffer::from_file(file_path.clone()).unwrap();

    let recovery_path = buffer.save_recovery_data(&recovery_dir).unwrap();
    assert!(recovery_path.exists());
    assert!(recovery_path.to_string_lossy().contains(".recovery"));

    let recovery_content = fs::read_to_string(&recovery_path).unwrap();
    assert_eq!(recovery_content, "Hello, world!");
}

#[test]
fn test_save_recovery_data_no_file_path() {
    let temp_dir = TempDir::new().unwrap();
    let recovery_dir = temp_dir.path().join("recovery");

    let mut buffer = Buffer::new();
    buffer.insert_char(0, 0, 'H').unwrap();
    buffer.insert_char(0, 1, 'i').unwrap();

    let recovery_path = buffer.save_recovery_data(&recovery_dir).unwrap();
    assert!(recovery_path.exists());
    assert!(recovery_path.to_string_lossy().contains("untitled"));

    let recovery_content = fs::read_to_string(&recovery_path).unwrap();
    assert_eq!(recovery_content, "Hi");
}

#[test]
fn test_load_recovery_data() {
    let temp_dir = TempDir::new().unwrap();
    let recovery_dir = temp_dir.path().join("recovery");
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "Original content").unwrap();
    let mut buffer = Buffer::from_file(file_path.clone()).unwrap();
    buffer.insert_char(0, 0, 'X').unwrap();

    let recovery_path = buffer.save_recovery_data(&recovery_dir).unwrap();

    let recovered = Buffer::load_recovery_data(recovery_path).unwrap();
    assert_eq!(recovered.content(), "XOriginal content");
}

#[test]
fn test_check_external_modification_no_change() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "Hello, world!").unwrap();
    let buffer = Buffer::from_file(file_path.clone()).unwrap();

    let is_modified = buffer.check_external_modification().unwrap();
    assert!(!is_modified);
}

#[test]
fn test_check_external_modification_with_change() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "Hello, world!").unwrap();
    let buffer = Buffer::from_file(file_path.clone()).unwrap();

    sleep(Duration::from_millis(100));
    fs::write(&file_path, "Modified content").unwrap();

    let is_modified = buffer.check_external_modification().unwrap();
    assert!(is_modified);
}

#[test]
fn test_check_external_modification_no_file() {
    let buffer = Buffer::new();
    let is_modified = buffer.check_external_modification().unwrap();
    assert!(!is_modified);
}

#[test]
fn test_reload_from_disk() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "Original content\n").unwrap();
    let mut buffer = Buffer::from_file(file_path.clone()).unwrap();

    assert_eq!(buffer.content(), "Original content\n");

    sleep(Duration::from_millis(10));
    fs::write(&file_path, "New content\n").unwrap();

    buffer.reload_from_disk().unwrap();
    assert_eq!(buffer.content(), "New content\n");
    assert!(!buffer.is_modified());
}

#[test]
fn test_reload_from_disk_no_file() {
    let mut buffer = Buffer::new();
    let result = buffer.reload_from_disk();
    assert!(result.is_err());
    match result.unwrap_err() {
        EditorError::InvalidOperation(_) => {}
        _ => panic!("Expected InvalidOperation error"),
    }
}

#[test]
fn test_has_unsaved_changes() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "Hello, world!").unwrap();
    let mut buffer = Buffer::from_file(file_path.clone()).unwrap();

    assert!(!buffer.has_unsaved_changes());

    buffer.insert_char(0, 0, 'X').unwrap();
    assert!(buffer.has_unsaved_changes());

    buffer.save().unwrap();
    assert!(!buffer.has_unsaved_changes());
}

#[test]
fn test_corrupted_file_invalid_utf8() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("invalid.txt");

    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(&[0xFF, 0xFE, 0xFD]).unwrap();

    let result = Buffer::from_file(file_path);
    assert!(result.is_err());
    match result.unwrap_err() {
        EditorError::CorruptedFile(_) => {}
        _ => panic!("Expected CorruptedFile error"),
    }
}

#[test]
fn test_save_preserves_line_ending() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "line1\r\nline2\r\n").unwrap();
    let mut buffer = Buffer::from_file(file_path.clone()).unwrap();

    assert_eq!(buffer.line_ending(), LineEnding::Crlf);

    buffer.insert_char(0, 5, 'X').unwrap();
    buffer.save().unwrap();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("\r\n"));
    assert_eq!(content, "line1X\r\nline2\r\n");
}

#[test]
fn test_backup_with_different_extensions() {
    let temp_dir = TempDir::new().unwrap();

    let test_files = vec![
        ("test.rs", "backup"),
        ("test.py", "backup"),
        ("test", "backup"),
    ];

    for (filename, expected_ext) in test_files {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, "content").unwrap();
        let buffer = Buffer::from_file(file_path).unwrap();

        let backup_path = buffer.create_backup().unwrap();
        assert!(backup_path.exists());
        assert_eq!(
            backup_path.extension().and_then(|e| e.to_str()),
            Some(expected_ext)
        );
        assert!(backup_path.to_string_lossy().ends_with(".backup"));
    }
}

#[test]
fn test_recovery_dir_creation() {
    let temp_dir = TempDir::new().unwrap();
    let recovery_dir = temp_dir.path().join("nonexistent").join("recovery");

    let mut buffer = Buffer::new();
    buffer.insert_char(0, 0, 'H').unwrap();

    let recovery_path = buffer.save_recovery_data(&recovery_dir).unwrap();
    assert!(recovery_dir.exists());
    assert!(recovery_path.exists());
}

#[test]
fn test_last_saved_none_for_new_buffer() {
    let buffer = Buffer::new();
    assert!(buffer.last_saved().is_none());
}

#[test]
fn test_save_as_updates_last_saved() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    let mut buffer = Buffer::new();
    buffer.insert_char(0, 0, 'H').unwrap();
    assert!(buffer.last_saved().is_none());

    buffer.save_as(file_path.clone()).unwrap();
    assert!(buffer.last_saved().is_some());
}

#[test]
fn test_reload_updates_last_saved() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "Original").unwrap();
    let mut buffer = Buffer::from_file(file_path.clone()).unwrap();
    let initial_time = buffer.last_saved().unwrap();

    sleep(Duration::from_millis(10));
    fs::write(&file_path, "Modified").unwrap();

    buffer.reload_from_disk().unwrap();
    let new_time = buffer.last_saved().unwrap();
    assert!(new_time > initial_time);
}
