extern crate blerp;
use blerp::Blerp;

macro_rules! blerp [
    ($($e:expr), *) => ({
        let mut argv = Vec::new();
        argv.push("blerp");
        $(argv.push($e);)*
        Blerp::new(argv)
    })
];

#[test]
fn blerp_no_args() {
    let blerp = blerp![];
    assert!(blerp.unwrap().run().is_ok());
}

#[test]
fn blerp_help_option() {
    let blerp = blerp!["--help"];
    assert_eq!(blerp.unwrap_err().fatal(), false);
}

#[test]
fn blerp_version_option() {
    let blerp = blerp!["--version"];
    assert_eq!(blerp.unwrap_err().fatal(), false);
}

#[test]
fn blerp_invalid_option() {
    let blerp = blerp!["-k"];
    assert_eq!(blerp.unwrap_err().fatal(), true);
}
