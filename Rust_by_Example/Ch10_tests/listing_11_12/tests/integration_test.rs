use listing_11_12;

mod common;

#[test]
fn it_adds_two() {
	common::setup();
	assert_eq!(4, listing_11_12::add_two(2));
}