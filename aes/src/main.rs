//! In Module 1, we discussed Block ciphers like AES. Block ciphers have a fixed length input.
//! Real wold data that we wish to encrypt _may_ be exactly the right length, but is probably not.
//! When your data is too short, you can simply pad it up to the correct length.
//! When your data is too long, you have some options.
//!
//! In this exercise, we will explore a few of the common ways that large pieces of data can be broken
//! up and combined in order to encrypt it with a fixed-length block cipher.
//!
//! WARNING: ECB MODE IS NOT SECURE.
//! Seriously, ECB is NOT secure. Don't use it irl. We are implementing it here to understand _why_ it
//! is not secure and make the point that the most straight-forward approach isn't always the best, and
//! can sometimes be trivially broken.

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

///We're using AES 128 which has 16-byte (128 bit) blocks.
const BLOCK_SIZE: usize = 16;
const INIT: [u8; BLOCK_SIZE] = [12, 5, 2, 11, 4, 5, 12, 5, 2, 11, 4, 5, 12, 4, 6, 12];

fn main() {
	let data: Vec<u8> = vec![12, 5, 2, 11, 4, 5];
	let data16: Vec<u8> = vec![12, 5, 2, 11, 4, 5, 12, 5, 2, 11, 4, 5, 12, 4, 6, 12];
	let key: [u8; 16] = [12, 5, 2, 11, 4, 5, 0, 5, 2, 11, 4, 5, 12, 4, 6, 12];
	let encr = cbc_encrypt(data.clone(), key);
	let encr16 = cbc_encrypt(data16.clone(), key);
	println!("CBC encr data {:?}", encr);
	println!("CBC encr data 16 {:?}", encr16);
	println!("------");
	println!("CBC unencr data {:?}", cbc_decrypt(encr, key));
	println!("CBC unencr data 16 {:?}", cbc_decrypt(encr16, key));

	println!("******************");

	let encr = ecb_encrypt(data, key);
	let encr16 = ecb_encrypt(data16, key);
	println!("ECB encr data {:?}", encr);
	println!("ECB encr data 16 {:?}", encr16);
	println!("------");
	println!("ECB unencr data {:?}", ecb_decrypt(encr, key));
	println!("ECB unencr data 16 {:?}", ecb_decrypt(encr16, key));
}

/// Simple AES encryption
/// Helper function to make the core AES block cipher easier to understand.
fn aes_encrypt(data: [u8; BLOCK_SIZE], key: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
	// Convert the inputs to the necessary data type
	let mut block = GenericArray::from(data);
	let key = GenericArray::from(*key);

	let cipher = Aes128::new(&key);

	cipher.encrypt_block(&mut block);

	block.into()
}

/// Simple AES encryption
/// Helper function to make the core AES block cipher easier to understand.
fn aes_decrypt(data: [u8; BLOCK_SIZE], key: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
	// Convert the inputs to the necessary data type
	let mut block = GenericArray::from(data);
	let key = GenericArray::from(*key);

	let cipher = Aes128::new(&key);

	cipher.decrypt_block(&mut block);

	block.into()
}

/// Before we can begin encrypting our raw data, we need it to be a multiple of the
/// block length which is 16 bytes (128 bits) in AES128.
///
/// The padding algorithm here is actually not trivial. The trouble is that if we just
/// naively throw a bunch of zeros on the end, there is no way to know, later, whether
/// those zeros are padding, or part of the message, or some of each.
///
/// The scheme works like this. If the data is not a multiple of the block length,  we
/// compute how many pad bytes we need, and then write that number into the last several bytes.
/// Later we look at the last byte, and remove that number of bytes.
///
/// But if the data _is_ a multiple of the block length, then we have a problem. We don't want
/// to later look at the last byte and remove part of the data. Instead, in this case, we add
/// another entire block containing the block length in each byte. In our case,
/// [16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16]
fn pad(mut data: Vec<u8>) -> Vec<u8> {
	// When twe have a multiple the second term is 0
	let number_pad_bytes = BLOCK_SIZE - data.len() % BLOCK_SIZE;

	for _ in 0..number_pad_bytes {
		data.push(number_pad_bytes as u8);
	}

	data
}

/// Groups the data into BLOCK_SIZE blocks. Assumes the data is already
/// a multiple of the block size. If this is not the case, call `pad` first.
fn group(data: Vec<u8>) -> Vec<[u8; BLOCK_SIZE]> {
	let mut blocks = Vec::new();
	let mut i = 0;
	while i < data.len() {
		let mut block: [u8; BLOCK_SIZE] = Default::default();
		block.copy_from_slice(&data[i..i + BLOCK_SIZE]);
		blocks.push(block);

		i += BLOCK_SIZE;
	}

	blocks
}

/// Does the opposite of the group function
fn un_group(blocks: Vec<[u8; BLOCK_SIZE]>) -> Vec<u8> {
	blocks.iter().fold(Vec::new(), |mut vec, chunk| {
		vec.extend_from_slice(&chunk[..]);
		vec
	})
}

/// Does the opposite of the pad function.
fn un_pad(data: Vec<[u8; BLOCK_SIZE]>) -> Vec<u8> {
	let mut arr = un_group(data);
	let bytes_to_remove = arr[arr.len() - 1] as usize;
	arr.truncate(arr.len() - bytes_to_remove);
	arr
}

/// The first mode we will implement is the Electronic Code Book, or ECB mode.
/// Warning: THIS MODE IS NOT SECURE!!!!
///
/// This is probably the first thing you think of when considering how to encrypt
/// large data. In this mode we simply encrypt each block of data under the same key.
/// One good thing about this mode is that it is parallelizable. But to see why it is
/// insecure look at: https://www.ubiqsecurity.com/wp-content/uploads/2022/02/ECB2.png
fn ecb_encrypt(plain_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
	let blocks = group(pad(plain_text));
	un_group(blocks.iter().map(|bl| aes_encrypt(*bl, &key)).collect())
}

/// Opposite of ecb_encrypt.
fn ecb_decrypt(cipher_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
	un_pad(group(cipher_text).iter().map(|block| aes_decrypt(*block, &key)).collect())
}

/// The next mode, which you can implement on your own is cipherblock chaining.
/// This mode actually is secure, and it often used in real world applications.
///
/// In this mode, the ciphertext from the first block is XORed with the
/// plaintext of the next block before it is encrypted.
fn cbc_encrypt(plain_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
	// Remember to generate a random initialization vector for the first block.
	let mut data: Vec<u8> = vec![];
	let chunks = group(pad(plain_text));
	let mut to_xor: [u8; BLOCK_SIZE] = INIT;
	chunks.iter().for_each(|ch| {
		let xored = xor(*ch, to_xor);
		let encrypted = aes_encrypt(xored, &key);
		to_xor = encrypted;
		data.extend_from_slice(&encrypted);
	});
	data
}

fn cbc_decrypt(cipher_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
	let chunks: Vec<[u8; BLOCK_SIZE]> = group(cipher_text);
	let mut unencrypted_chunks: Vec<[u8; BLOCK_SIZE]> = vec![];
	let mut to_xor: [u8; BLOCK_SIZE] = INIT;
	chunks.iter().for_each(|ch| {
		let data = aes_decrypt(*ch, &key);
		let xored = xor(data, to_xor);
		unencrypted_chunks.push(xored);
		to_xor = *ch;
	});
	un_pad(unencrypted_chunks)
}

fn xor(first: [u8; BLOCK_SIZE], second: [u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
	first
		.iter()
		.zip(second.iter())
		.map(|(f, s)| f ^ s)
		.collect::<Vec<u8>>()
		.try_into()
		.unwrap()
}
