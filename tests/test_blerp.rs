extern crate blerp;
use blerp::Blerp;

#[test]
fn test_blerp() {
    let blerp = Blerp::new(vec!["blerp"]);
    assert!(blerp.unwrap().run().is_ok());
}
