pub fn get_file_bytes(path: &str) -> Result<~[u8], ~str> {
	use core::path::GenericPath::from_str;
	use core::io::read_whole_file;

	let path: Path = from_str(path);
	let result = read_whole_file(&path);
	return result;
}
