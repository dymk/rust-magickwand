#[path = "../lib/wand.rs"]
mod wand;
mod helper;

#[test]
fn test_new_wand() {
	let _wand = wand::MagickWand::new();
}

#[test]
fn test_isMagickWand() {
	let wand = wand::MagickWand::new();
	assert!(wand.isMagickWand());
}

#[test]
fn test_clear() {
	let wand = wand::MagickWand::new();
	wand.clear();
}

#[test]
fn test_read_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("../test/small_bmp.bmp") == true);
	assert!(wand.readImage("../test/non_existant_image.bmp") == false);
}

#[test]
fn test_adaptive_resize_image() {
	let wand = wand::MagickWand::new();
	wand.readImage("../test/small_bmp.bmp");
	assert!(wand.adaptiveResizeImage(10, 10));
}

#[test]
fn test_write_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("../test/small_bmp.bmp"));
	assert!(wand.writeImage("../test/written/small_bmp_write.bmp"));
}

#[test]
fn test_adaptive_resize_write_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("../test/small_bmp.bmp"));
	assert!(wand.adaptiveResizeImage(100, 100));
	assert!(wand.writeImage("../test/written/small_bmp_adaptive_resize_write.bmp"));
}
