use std::fs::File;
use std::io::{Read, Write, stdin};
use aes::{Aes256};
use cipher::{
    KeyInit,
    generic_array::GenericArray,
};

use rand::{RngCore, rngs::OsRng};


fn main(){
    let mut input = String::new();
    println!("Please enter a path to a file:");
    stdin().read_line(&mut input).expect("Failed to read input");
    println!("Encrypting file {}!", input.trim());
    encrypt_file(input.trim());
}


fn encrypt_file(path: &str) {
    let mut file = File::open(path).unwrap();
    let mut plaintext = Vec::new();
    file.read_to_end(&mut plaintext).unwrap();

    // Generate a random 256-bit key
    let mut key = [0u8; 32];
    let mut rng = OsRng::default();
    rng.fill_bytes(&mut key);

    // Generate a random 128-bit IV
    let mut iv = [0u8; 16];
    rng.fill_bytes(&mut iv);

    // Create cipher instance
    let cipher = cipher::StreamCipher::new(&key);

    // Encrypt the plaintext
    let mut ciphertext = plaintext.clone();
    cipher.apply_keystream(&mut ciphertext);

    // Write the encrypted data to file
    let mut file = File::create("output.txt").unwrap();
    file.write_all(&iv).unwrap();
    file.write_all(&ciphertext).unwrap();

    // Decrypt the data and print it
    let mut file = File::open("output.txt").unwrap();
    let mut iv = [0u8; 16];
    file.read_exact(&mut iv).unwrap();

    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext).unwrap();

    let cipher = Ctr::<Aes256>::new(&GenericArray::from_slice(&key), &GenericArray::from_slice(&iv));
    cipher.apply_keystream(&mut ciphertext);

    println!("{}", String::from_utf8(ciphertext).unwrap());
}