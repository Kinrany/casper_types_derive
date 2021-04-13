This crate contains three [derive macros] for [`casper_types::CLTyped`],
[`casper_types::bytesrepr::FromBytes`] and [`casper_types::bytesrepr::ToBytes`].

[derive macros]: https://doc.rust-lang.org/reference/attributes/derive.html
[`casper_types::CLTyped`]: https://docs.rs/casper-types/1.0.1/casper_types/trait.CLTyped.html
[`casper_types::bytesrepr::FromBytes`]: https://docs.rs/casper-types/1.0.1/casper_types/bytesrepr/trait.FromBytes.html
[`casper_types::bytesrepr::ToBytes`]: https://docs.rs/casper-types/1.0.1/casper_types/bytesrepr/trait.ToBytes.html

You might want to implement these three traits for a `struct` you want to store in [Casper] storage. See storage API documentation on [docs.casperlabs.io].

[Casper]: https://casperlabs.io/
[docs.casperlabs.io]: https://docs.casperlabs.io/en/latest/dapp-dev-guide/writing-contracts/writing-rust-contracts.html#storage

A macro declared on a struct like this:

```rust
use casper_types_derive::{CLTyped, ToBytes, FromBytes};

#[derive(CLTyped, ToBytes, FromBytes)]
struct Dog {
  name: String,
  likes_treat: BTreeMap<String, bool>,
}
```

Will expand into this:

```rust
impl CLTypes for Dog {
  fn cl_type(&self) -> CLType {
    CLType::Any
  }
}

impl FromBytes for Dog {
  fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
    let (name, bytes) = FromBytes::from_bytes(bytes)?;
    let (likes_treat, bytes) = FromBytes::from_bytes(bytes)?;
    let value = Dog {
      name,
      likes_treat,
    };
    Ok((value, bytes))
  }
}

impl ToBytes for Dog {
  fn serialized_length(&self) -> usize {
    let mut size = 0;
    size += name.serialized_length();
    size += likes_treat.serialized_length();
    return size;
  }

  fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
    let mut vec = Vec::with_capacity(self.serialized_length());
    vec.append(self.name.to_bytes()?);
    vec.append(self.likes_treat.to_bytes()?);
    Ok(vec)
  }
}
```
