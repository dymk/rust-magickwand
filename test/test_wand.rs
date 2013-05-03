use wand;
use pixel::{RGB,YIQ};
use types;

#[test]
fn test_new_wand() {
	let _wand = wand::MagickWand::new();
}

#[test]
fn test_clone_wand() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/small_bmp.bmp"));
	let second_wand = wand.clone();
	wand.clear();
	assert!(second_wand.num_images() == 1);
}

#[test]
fn test_get_num_images() {
	let wand = wand::MagickWand::new();
	assert!(wand.num_images() == 0);
	assert!(wand.read_image("test/read/small_bmp.bmp"));
	assert!(wand.num_images() == 1);
}

#[test]
fn test_is_magick_wand() {
	let wand = wand::MagickWand::new();
	assert!(wand.is_magick_wand());
}

#[test]
fn test_clear() {
	let wand = wand::MagickWand::new();
	wand.clear();
}

#[test]
fn test_read_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/small_bmp.bmp") == true);
	assert!(wand.read_image("test/non_existant_image.bmp") == false);
}

#[test]
fn test_adaptive_resize_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/small_bmp.bmp"));
	assert!(wand.adaptive_resize_image(10, 10));
}

#[test]
fn test_write_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/small_bmp.bmp"));
	assert!(wand.write_image("test/written/small_bmp.bmp"));
}

#[test]
fn test_adaptive_resize_write_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/small_bmp.bmp"));
	assert!(wand.adaptive_resize_image(100, 100));
	assert!(wand.write_image("test/written/small_bmp_adaptive_resize.bmp"));
}

#[test]
fn test_resize() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/small_bmp.bmp"));
	assert!(wand.resize_image(100, 50, types::TriangleFilter, 0.0));

	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/small_bmp_nonexistant.bmp") == false);
	assert!(wand.resize_image(100, 50, types::TriangleFilter, 0.0) == false);
}

#[test]
fn test_resize_write_image() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/small_bmp.bmp"));
	assert!(wand.resize_image(100, 50, types::LanczosFilter, 1.0));
	assert!(wand.write_image("test/written/small_bmp_resize.bmp"));
}

#[test]
fn test_image_width() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/white_line_10px_bmp.bmp"));
	assert!(wand.image_width() == 10);
}

#[test]
fn test_image_height() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/white_line_10px_bmp.bmp"));
	assert!(wand.image_height() == 1);
}

#[test]
fn test_export_image_pixels() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/white_line_10px_bmp.bmp"));
	let pixels = match wand.export_image_pixels::<RGB>(None, None) {
		Some(pixels) => pixels,
		None         => fail!(~"Export failed")
	};

	assert!(pixels.len() == 1);
	assert!(pixels[0].len() == 10);
	for pixels[0].each |&px| {
		assert!(px == RGB(255, 255, 255));
	}

	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/rgb_line_3px_bmp.bmp"));
	let pixels = match wand.export_image_pixels::<RGB>(None, None) {
		Some(pixels) => pixels,
		None         => fail!(~"Export failed")
	};

	let row1 = &pixels[0];
	let r = row1[0];
	let g = row1[1];
	let b = row1[2];
	assert!(r == RGB(255, 0, 0));
	assert!(g == RGB(0, 255, 0));
	assert!(b == RGB(0, 0, 255));
}

#[test]
fn test_export_image_pixels_without_image() {
	let wand = wand::MagickWand::new();
	match wand.export_image_pixels::<RGB>(None, None) {
		Some(_) => fail!(~"Pixels not expected"),
		None    => assert!(true)
	}
}

#[test]
fn test_export_image_pixels_yiq() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/white_line_10px_bmp.bmp"));
	let pixels = match wand.export_image_pixels::<YIQ>(None, None) {
		Some(pixels) => pixels,
		None         => fail!(~"Should have found pixels")
	};
	for pixels[0].each |p| {
		assert!(*p == YIQ(255, 0, 0));
	}
}

#[test]
fn test_export_image_pixels_flat() {
	let wand = wand::MagickWand::new();
	assert!(wand.read_image("test/read/white_line_10px_bmp.bmp"));
	let flat_pixels = match wand.export_image_pixels_flat::<RGB>(None, None) {
		Some(p) => p,
		None    => fail!(~"Should have found pixels")
	};
	assert!(flat_pixels.len() == 10);
	for flat_pixels.each |p| {
		assert!(*p == RGB(255, 255, 255));
	}
}

#[test]
fn test_import_pixels_flat() {
	let wand = wand::MagickWand::new();
	//White line
	let pixels = vec::from_elem(10, RGB(255, 255, 255));
	assert!(wand.new_image(10, 1, None));
	assert!(wand.import_image_pixels_flat(pixels, Some((0, 0)), Some((10, 1))));

	let ex_pixel = match wand.export_image_pixels_flat::<RGB>(None, None) {
		Some(p) => p,
		None    => fail!(~"expected pixels")
	};
	for ex_pixel.each |p| {
		assert!(*p == RGB(255, 255, 255));
	}
}

#[test]
fn test_import_pixels() {
	let wand = wand::MagickWand::new();
	//White 5x5 rectangle
	let pixels = vec::from_fn(5, |_| {
		vec::from_elem(5, RGB(255, 255, 255))
	});
	assert!(wand.new_image(5, 5, None));
	assert!(wand.import_image_pixels(pixels, None));

	let ex_pixel = match wand.export_image_pixels_flat::<RGB>(None, None) {
		Some(p) => p,
		None    => fail!(~"expected pixels")
	};
	for ex_pixel.each |p| {
		assert!(*p == RGB(255, 255, 255));
	}
}

#[test]
fn test_import_pixels_offseted() {
	let wand = wand::MagickWand::new();
	assert!(wand.new_image(2, 2, None));

	//Make the bottom half white
	let px = ~[~[RGB(255, 255, 255), RGB(255, 255, 255)]];
	assert!(wand.import_image_pixels(px, Some((0, 1))));
	let px = wand.export_image_pixels::<RGB>(None, None).unwrap();
	assert_eq!(px[0][0], RGB(0, 0, 0)); //Top row
	assert_eq!(px[0][1], RGB(0, 0, 0));
	assert_eq!(px[1][0], RGB(255, 255, 255)); //Bottom row
	assert_eq!(px[1][1], RGB(255, 255, 255));
}

#[test]
fn test_new_image_default_pixels() {
	let wand = wand::MagickWand::new();
	assert!(wand.new_image(10, 10, None));
	let px = match wand.export_image_pixels_flat::<RGB>(None, None) {
		Some(p) => p,
		None    => fail!(~"expected pixels")
	};
	assert_eq!(px.len(), 100);
	for px.each |p| {
		assert!(*p == RGB(0, 0, 0));
	}
}

#[test]
fn test_new_image_write() {
	let wand = wand::MagickWand::new();
	//Going to put a red square in the center of the image
	wand.new_image(5, 5, None);
	let px = vec::from_elem(9, RGB(255, 0, 0));
	wand.import_image_pixels_flat(px, Some((1, 1)), Some((3, 3)));
	assert!(wand.write_image("test/written/red_square_imported_px.bmp"));
}
