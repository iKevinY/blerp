extern crate blerp;
use blerp::Blerp;

#[test]
fn test_blerp() {
    let blerp = Blerp::new(vec!["blerp"]);
    assert!(blerp.unwrap().run().is_ok());
}

#[test]
fn test_help_option() {
    let blerp = Blerp::new(vec!["blerp", "--help"]);
    assert_eq!(blerp.unwrap_err().fatal(), false);
}

#[test]
fn test_version_option() {
    let blerp = Blerp::new(vec!["blerp", "--version"]);
    assert_eq!(blerp.unwrap_err().fatal(), false);
}

#[test]
fn test_invalid_option() {
    let blerp = Blerp::new(vec!["blerp", "-k"]);
    assert_eq!(blerp.unwrap_err().fatal(), true);
}
