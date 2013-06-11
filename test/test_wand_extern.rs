use wand_extern;
use super::helper;

#[test]
fn test_wand_genesis() {
	unsafe {
		wand_extern::MagickWandGenesis();
	}
}

//Can't test this due to tests running in parallel
// #[test]
// fn test_wand_terminus() {
// 	unsafe {
// 		wand_extern::MagickWandGenesis();
// 		wand_extern::MagickWandTerminus();
// 	}
// }

#[test]
fn test_magick_read_image() {
	unsafe {
		let wand = wand_extern::NewMagickWand();
		let path_bytes = str::to_bytes("test/read/small_bmp.bmp");
		let raw_path_bytes = vec::raw::to_ptr(path_bytes);
		assert!(wand_extern::MagickReadImage(wand, raw_path_bytes as *libc::c_char));
		assert!(wand_extern::MagickIdentifyImage(wand) != ptr::null());
	}
}

#[test]
fn test_adaptive_resize_image() {
	unsafe {
		let wand = wand_extern::NewMagickWand();
		assert!(
		wand_extern::MagickReadImage(
			wand,
			vec::raw::to_ptr(str::to_bytes("test/read/small_bmp.bmp")) as *i8
		));
		assert!(wand_extern::MagickAdaptiveResizeImage(wand, 10, 10));
	}
}

#[test]
fn test_clear_wand() {
	unsafe {
		let wand = wand_extern::NewMagickWand();
		wand_extern::ClearMagickWand(wand);
	}
}

#[test]
fn test_reset_itterator() {
	unsafe {
		let wand = wand_extern::NewMagickWand();
		wand_extern::MagickResetIterator(wand);
	}
}

#[test]
fn test_identify_image() {
	unsafe {
		let wand = wand_extern::NewMagickWand();
		let attrs = wand_extern::MagickIdentifyImage(wand);
		assert_eq!(attrs, ptr::null());
	}

	unsafe {
		let wand = wand_extern::NewMagickWand();
		let image_bytes = match helper::get_file_bytes("test/read/small_bmp.bmp") {
			Ok(bytes) => bytes,
			Err(msg) => fail!(fmt!("Error reading file bytes: %s", msg))
		};
		wand_extern::MagickReadImageBlob(wand, vec::raw::to_ptr(image_bytes), image_bytes.len() as u32);
		let attrs = wand_extern::MagickIdentifyImage(wand);
		assert!(attrs != ptr::null());
		// let attrs_safe = core::str::raw::from_c_str(attrs);
		// io::println(attrs_safe);
	}
}

#[test]
fn test_relinquish_memory() {
	unsafe {
		let wand = wand_extern::NewMagickWand();
		let image_bytes = match helper::get_file_bytes("test/read/small_bmp.bmp") {
			Ok(bytes) => bytes,
			Err(msg) => fail!(fmt!("Error reading file bytes: %s", msg))
		};
		wand_extern::MagickReadImageBlob(wand, vec::raw::to_ptr(image_bytes), image_bytes.len() as u32);
		let attrs = wand_extern::MagickIdentifyImage(wand);
		wand_extern::MagickRelinquishMemory(attrs as *libc::c_void);
	}
}

#[test]
fn test_magick_get_image_blob() {
	let image_bytes = match helper::get_file_bytes("test/read/small_bmp.bmp") {
		Ok(bytes) => bytes,
		Err(msg) => fail!(fmt!("Error reading file bytes: %s", msg))
	};
	unsafe {
		//Test without a blob in the wand
		let wand = wand_extern::NewMagickWand();
		let mut len: libc::size_t = 0;
		let blob = wand_extern::MagickGetImageBlob(wand, &mut len);
		assert_eq!(len, 0);
		assert_eq!(blob, ptr::null());
		wand_extern::DestroyMagickWand(wand);
	}

	unsafe {
		//Test with some image bytes
		let wand = wand_extern::NewMagickWand();
		wand_extern::MagickReadImageBlob(wand, vec::raw::to_ptr(image_bytes), image_bytes.len() as u32);
		let mut len: libc::size_t = 0;
		let blob = wand_extern::MagickGetImageBlob(wand, &mut len);
		assert!(len != 0);
		assert!(blob != ptr::null());
		wand_extern::DestroyMagickWand(wand);
		wand_extern::MagickRelinquishMemory(blob);
	}
}

#[test]
fn test_magick_read_image_blob() {

	let image_bytes = match helper::get_file_bytes("test/read/small_bmp.bmp") {
		Ok(bytes) => bytes,
		Err(msg) => fail!(fmt!("Error reading file bytes: %s", msg))
	};
	unsafe {
		let wand = wand_extern::NewMagickWand();
		let res = wand_extern::MagickReadImageBlob(wand, vec::raw::to_ptr(image_bytes), image_bytes.len() as u32);
		assert!(res == true);

		let wand = wand_extern::NewMagickWand();
		let image_bytes: ~[u8] = ~[];
		let res = wand_extern::MagickReadImageBlob(wand, vec::raw::to_ptr(image_bytes), image_bytes.len() as u32);
		assert!(res == false);
	}
}

#[test]
fn test_is_magick_wand() {
	unsafe {
		let mw = wand_extern::NewMagickWand();
		assert!(wand_extern::IsMagickWand(mw) == true);
		assert!(wand_extern::IsMagickWand(ptr::null()) == false);
	}
}

#[test]
fn test_new_magick_wand() {
	unsafe {
		let mw = wand_extern::NewMagickWand();
		assert!(mw != ptr::null());
	}
}

#[test]
fn test_destroy_magick_wand() {
	unsafe {
		let mw = wand_extern::NewMagickWand();
		let mw = wand_extern::DestroyMagickWand(mw);
		assert!(mw == ptr::null());
		assert!(wand_extern::IsMagickWand(mw) == false);
	}
}
