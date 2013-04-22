pub fn get_file_bytes(path: &str) -> ~[u8] {
	use core::path::GenericPath::{from_str};
	use core::result::{is_err, unwrap};
	use core::io::read_whole_file;

	let basedir: Path = from_str(file!());
	let basedir: ~str = basedir.dirname();
	let path: Path = from_str(basedir + ~"/" + path);
	let result = read_whole_file(&path);
	if is_err::<~[u8], ~str>(&result) {
		fail!(fmt!("Couldn't open file %s", path.to_str()));
	}

	let result = unwrap(result);
	result
}
