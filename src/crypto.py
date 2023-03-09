from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad
import getpass
import os


# Function to encrypt a file
def encrypt_file(file_path, key):
    # Generate a 16-byte initialization vector
    iv = os.urandom(16)

    # Create a new AES cipher with the provided key and initialization vector
    cipher = AES.new(key, AES.MODE_CBC, iv)

    # Read in the contents of the input file
    with open(file_path, "rb") as f:
        file_contents = f.read()

    # Pad the file contents to be a multiple of 16 bytes
    padded_contents = pad(file_contents, 16)

    # Encrypt the padded file contents using AES-CBC mode
    encrypted_contents = cipher.encrypt(padded_contents)

    # Write the encrypted contents and the initialization vector to a new file
    with open(file_path + ".enc", "wb") as f:
        f.write(iv + encrypted_contents)


# Function to decrypt a file
def decrypt_file(file_path, key):
    # Read in the contents of the input file
    with open(file_path, "rb") as f:
        file_contents = f.read()

    # Extract the initialization vector and encrypted contents from the file
    iv = file_contents[:16]
    encrypted_contents = file_contents[16:]

    # Create a new AES cipher with the provided key and initialization vector
    cipher = AES.new(key, AES.MODE_CBC, iv)

    # Decrypt the encrypted file contents using AES-CBC mode
    decrypted_contents = cipher.decrypt(encrypted_contents)

    # Unpad the decrypted contents
    unpadded_contents = unpad(decrypted_contents, 16)

    # Write the decrypted contents to a new file
    with open(file_path[:-4], "wb") as f:
        f.write(unpadded_contents)


# Prompt the user for a password
password = getpass.getpass("Enter your password: ")

# Convert the password to bytes and pad it with zeros if necessary
key = password.encode("utf-8")
key = key.ljust(32, b'\0')[:32]

# Loop until the user chooses to quit
while True:
    # Prompt the user for a choice (encrypt, decrypt, or quit)
    choice = input("Enter 'e' to encrypt, 'd' to decrypt, or 'q' to quit: ")

    # Handle the user's choice
    if choice == "e":
        # Prompt the user for a file to encrypt
        file_path = input("Enter the path of the file to encrypt: ")

        # Encrypt the file using the user's password as a key
        encrypt_file(file_path, key)

        print(f"File encrypted and saved as {file_path}.enc")
    elif choice == "d":
        # Prompt the user for a file to decrypt
        file_path = input("Enter the path of the file to decrypt: ")

        # Decrypt the file using the user's password as a key
        decrypt_file(file_path, key)

        print(f"File decrypted and saved as {file_path[:-4]}")
    elif choice == "q":
        # Exit the program
        break
    else:
        # Invalid choice
        print("Invalid choice. Please try again.") 
