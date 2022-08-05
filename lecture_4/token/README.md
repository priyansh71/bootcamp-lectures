## Token Program

- Token Account : The amount of that token a particular user possesses.
  - Amount : **Integer** = Amount of these tokens held.
  - Owner : **Public Key** = The wallet who needs to sign the transaction for amount changes.
  - Mint : **Mint Object**
- Mint : Token Type.
  - Mint Authority : **Public Key** = Public Key of System Account who can mint more tokens.
  - Supply : **Integer** = Total circulation of that token type.

Operations supported for Tokens :
- Transfer : change ownership of tokens from A to B
- Burn : remove some tokens from circulation
- Mint : change ownership of tokens from Token Program to User

Borsh:
- Deserialize
    ```rust
      fn try_from_slice(v: &[u8]) -> Result<Self>
      // Deserialize this instance from a slice of bytes.
    ```

- Serialize
    ```rust
      fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error>
    ```