use pixel_wand;

#[test]
fn test_new_pixel_wand() {
	let pw = pixel_wand::PixelWand::new();
	assert!(pw.is_pixel_wand());
}
