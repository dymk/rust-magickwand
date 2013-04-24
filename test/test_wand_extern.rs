// use wand_extern::wand;

#[path = "../lib/types.rs"]
mod types;
#[path = "../lib/wand_extern.rs"]
mod wand_extern;
mod helper;
#[test]
fn test_wand_genesis() {
	unsafe {
		wand_extern::wand::MagickWandGenesis();
	}
}

//Can't test this due to tests running in parallel
// #[test]
// fn test_wand_terminus() {
// 	unsafe {
// 		wand_extern::wand::MagickWandGenesis();
// 		wand_extern::wand::MagickWandTerminus();
// 	}
// }

#[test]
fn test_magick_read_image() {
	unsafe {
		let wand = wand_extern::wand::NewMagickWand();
		let path_bytes = str::to_bytes("../test/read/small_bmp.bmp");
		let raw_path_bytes = vec::raw::to_ptr(path_bytes);
		assert!(wand_extern::wand::MagickReadImage(wand, raw_path_bytes as *libc::c_char) == true);
		assert!(wand_extern::wand::MagickIdentifyImage(wand) != ptr::null());
	}
}

#[test]
fn test_adaptive_resize_image() {
	unsafe {
		let wand = wand_extern::wand::NewMagickWand();
		assert!(
		wand_extern::wand::MagickReadImage(
			wand,
			vec::raw::to_ptr(str::to_bytes("../test/read/small_bmp.bmp")) as *i8
		) == true);
		assert!(wand_extern::wand::MagickAdaptiveResizeImage(wand, 10, 10) == true);
	}
}

#[test]
fn test_clear_wand() {
	unsafe {
		let wand = wand_extern::wand::NewMagickWand();
		wand_extern::wand::ClearMagickWand(wand);
	}
}

#[test]
fn test_reset_itterator() {
	unsafe {
		let wand = wand_extern::wand::NewMagickWand();
		wand_extern::wand::MagickResetIterator(wand);
	}
}

#[test]
fn test_identify_image() {
	unsafe {
		let wand = wand_extern::wand::NewMagickWand();
		let attrs = wand_extern::wand::MagickIdentifyImage(wand);
		assert!(attrs == ptr::null());
	}

	unsafe {
		let wand = wand_extern::wand::NewMagickWand();
		let image_bytes = helper::get_file_bytes("../test/read/small_bmp.bmp");
		wand_extern::wand::MagickReadImageBlob(wand, vec::raw::to_ptr(image_bytes), image_bytes.len() as u32);
		let attrs = wand_extern::wand::MagickIdentifyImage(wand);
		assert!(attrs != ptr::null());
		// let attrs_safe = core::str::raw::from_c_str(attrs);
		// io::println(attrs_safe);
	}
}

#[test]
fn test_relinquish_memory() {
	unsafe {
		let wand = wand_extern::wand::NewMagickWand();
		let image_bytes = helper::get_file_bytes("../test/read/small_bmp.bmp");
		wand_extern::wand::MagickReadImageBlob(wand, vec::raw::to_ptr(image_bytes), image_bytes.len() as u32);
		let attrs = wand_extern::wand::MagickIdentifyImage(wand);
		wand_extern::wand::MagickRelinquishMemory(attrs as *libc::c_void);
	}
}

#[test]
fn test_magick_get_image_blob() {
	let image_bytes = helper::get_file_bytes("../test/read/small_bmp.bmp");
	unsafe {
		//Test without a blob in the wand
		let wand = wand_extern::wand::NewMagickWand();
		let mut len: libc::size_t = 0;
		let blob = wand_extern::wand::MagickGetImageBlob(wand, &len);
		assert!(len == 0);
		assert!(blob == ptr::null());
		wand_extern::wand::DestroyMagickWand(wand);
	}

	unsafe {
		//Test with some image bytes
		let wand = wand_extern::wand::NewMagickWand();
		wand_extern::wand::MagickReadImageBlob(wand, vec::raw::to_ptr(image_bytes), image_bytes.len() as u32);
		let mut len: libc::size_t = 0;
		let blob = wand_extern::wand::MagickGetImageBlob(wand, &len);
		assert!(len != 0);
		assert!(blob != ptr::null());
		wand_extern::wand::DestroyMagickWand(wand);
		wand_extern::wand::MagickRelinquishMemory(blob);
	}
}

#[test]
fn test_magick_read_image_blob() {

	let image_bytes = helper::get_file_bytes("../test/read/small_bmp.bmp");
	unsafe {
		let wand = wand_extern::wand::NewMagickWand();
		let res = wand_extern::wand::MagickReadImageBlob(wand, vec::raw::to_ptr(image_bytes), image_bytes.len() as u32);
		assert!(res == true);

		let wand = wand_extern::wand::NewMagickWand();
		let image_bytes: ~[u8] = ~[];
		let res = wand_extern::wand::MagickReadImageBlob(wand, vec::raw::to_ptr(image_bytes), image_bytes.len() as u32);
		assert!(res == false);
	}
}

#[test]
fn test_is_magick_wand() {
	unsafe {
		let mw = wand_extern::wand::NewMagickWand();
		assert!(wand_extern::wand::IsMagickWand(mw) == true);
		assert!(wand_extern::wand::IsMagickWand(ptr::null()) == false);
	}
}

#[test]
fn test_new_magick_wand() {
	unsafe {
		let mw = wand_extern::wand::NewMagickWand();
		assert!(mw != ptr::null());
	}
}

#[test]
fn test_destroy_magick_wand() {
	unsafe {
		let mw = wand_extern::wand::NewMagickWand();
		let mw = wand_extern::wand::DestroyMagickWand(mw);
		assert!(mw == ptr::null());
		assert!(wand_extern::wand::IsMagickWand(mw) == false);
	}
}
