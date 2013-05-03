use pixel::{YIQ, RGB, FromRGB, ToRGB};

#[test]
fn test_rgb_to_yiq() {
	let rgb = RGB(255, 255, 255);
	let yiq = YIQ(255,   0,   0);
	let rgb_2_yiq: YIQ = FromRGB::from_rgb(rgb);
	assert_eq!(rgb_2_yiq, yiq);
}

#[test]
fn test_yiq_to_rgb() {
	let rgb = RGB(255, 255, 255);
	let yiq = YIQ(255,   0,   0);
	let yiq_2_rgb = yiq.to_rgb();
	assert_eq!(yiq_2_rgb, rgb);
}

#[test]
fn test_rgb_to_rgb() {
	let rgb = RGB(255, 255, 255);
	assert_eq!(rgb, rgb.to_rgb());
}
