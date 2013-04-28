use wand;
use pixel;
use types;

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
	assert!(wand.readImage("test/read/small_bmp.bmp") == true);
	assert!(wand.readImage("test/non_existant_image.bmp") == false);
}

#[test]
fn test_adaptive_resize_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/read/small_bmp.bmp"));
	assert!(wand.adaptiveResizeImage(10, 10));
}

#[test]
fn test_write_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/read/small_bmp.bmp"));
	assert!(wand.writeImage("test/written/small_bmp.bmp"));
}

#[test]
fn test_adaptive_resize_write_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/read/small_bmp.bmp"));
	assert!(wand.adaptiveResizeImage(100, 100));
	assert!(wand.writeImage("test/written/small_bmp_adaptive_resize.bmp"));
}

#[test]
fn test_resize() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/read/small_bmp.bmp"));
	assert!(wand.resizeImage(100, 50, types::TriangleFilter, 0.0));

	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/small_bmp_nonexistant.bmp") == false);
	assert!(wand.resizeImage(100, 50, types::TriangleFilter, 0.0) == false);
}

#[test]
fn test_resize_write_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/read/small_bmp.bmp"));
	assert!(wand.resizeImage(100, 50, types::LanczosFilter, 1.0));
	assert!(wand.writeImage("test/written/small_bmp_resize.bmp"));
}

#[test]
fn test_image_width() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/read/white_line_10px_bmp.bmp"));
	assert!(wand.imageWidth() == 10);
}

#[test]
fn test_image_height() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/read/white_line_10px_bmp.bmp"));
	assert!(wand.imageHeight() == 1);
}

#[test]
fn test_export_image_pixels() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/read/white_line_10px_bmp.bmp"));
	let pixels = match wand.exportPixels::<pixel::RGB>() {
		Some(pixels) => pixels,
		None         => fail!(~"Export failed")
	};

	assert!(pixels.len() == 10);
	for pixels.each |&px| {
		assert!(px == pixel::RGB(255, 255, 255));
	}

	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/read/rgb_line_3px_bmp.bmp"));
	let pixels = match wand.exportPixels::<pixel::RGB>() {
		Some(pixels) => pixels,
		None         => fail!(~"Export failed")
	};

	let r = pixels[0];
	let g = pixels[1];
	let b = pixels[2];
	assert!(r == pixel::RGB(255, 0, 0));
	assert!(g == pixel::RGB(0, 255, 0));
	assert!(b == pixel::RGB(0, 0, 255));
}

#[test]
fn test_export_image_pixels_without_image() {
	let wand = wand::MagickWand::new();
	match wand.exportPixels::<pixel::RGB>() {
		Some(_) => fail!(~"Pixels not expected"),
		None    => assert!(true)
	}
}

#[test]
fn test_export_image_pixels_yiq() {
	let wand = wand::MagickWand::new();
	assert!(wand.readImage("test/read/white_line_10px_bmp.bmp"));
	let pixels = match wand.exportPixels::<pixel::YIQ>() {
		Some(pixels) => pixels,
		None         => fail!(~"Should have found pixels")
	};
	for pixels.each |p| {
		assert!(*p == pixel::YIQ(255, 0, 0));
	}
}
