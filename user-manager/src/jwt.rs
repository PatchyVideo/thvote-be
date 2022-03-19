
use std::io::Read;

use jwt_simple::prelude::*;

fn read_a_file(filename: &str) -> std::io::Result<Vec<u8>> {
	let mut file = std::fs::File::open(filename)?;

	let mut data = Vec::new();
	file.read_to_end(&mut data)?;

	return Ok(data);
}

pub async fn load_keys() -> Result<ES256kKeyPair, Box<dyn std::error::Error>> {
	Ok(ES256kKeyPair::from_pem(std::str::from_utf8(&read_a_file("../keys/key-priv.pem")?)?)?)
}

