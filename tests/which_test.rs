use cmdplus::commands::which::which_command;

#[test]
fn test_which_finds_existing_command() {
    let result = which_command("cargo");
    assert!(result.is_some());
    assert!(result.unwrap().to_str().unwrap().contains("cargo"));
}

#[test]
fn test_which_returns_none_for_nonexistent_command() {
    let result = which_command("clearly_nonexistent_command_xyz");
    assert!(result.is_none());
}
