use openssl::rsa::Rsa;
use openssl::pkey::PKey;

use std::str;

fn main() {
    // Generate a new RSA key pair
    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa.clone()).unwrap();
    
    // Export the public key in PEM format
    let pub_key: Vec<u8> = pkey.public_key_to_pem().unwrap();
    println!("Public Key:\n{}", str::from_utf8(&pub_key).unwrap());


    // Plaintext to encrypt
    let plaintext = b"Hello, world!";

     // Encrypt the plaintext using the public key
     let mut buffer: Vec<u8> = vec![0; rsa.size() as usize];
     let encrypted_length = rsa.public_encrypt(plaintext, &mut buffer, openssl::rsa::Padding::PKCS1).unwrap();
     
     // Truncate the buffer to the actual length of the encrypted data
     buffer.truncate(encrypted_length);
 
    // Print the encrypted data in hexadecimal format for readability
    println!("Encrypted: {:?}", hex::encode(&buffer)); // Clone the buffer for printing if needed

    // Decrypt the message using the private key
    let mut decrypted_buffer: Vec<u8> = vec![0; rsa.size() as usize];
    let decrypted_length = rsa.private_decrypt(&buffer, &mut decrypted_buffer, openssl::rsa::Padding::PKCS1).unwrap();
    
    // Truncate the buffer to the actual length of the decrypted data
    decrypted_buffer.truncate(decrypted_length);
    let decrypted_message = str::from_utf8(&decrypted_buffer).unwrap();
    
    // Print the decrypted message
    println!("Decrypted: {}", decrypted_message);
}
