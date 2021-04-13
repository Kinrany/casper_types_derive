use casper_types::bytesrepr::{FromBytes, ToBytes};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use std::collections::BTreeMap;

#[derive(Clone, CLTyped, Debug, FromBytes, PartialEq, ToBytes)]
struct Dog {
  name: String,
  food_ratings: BTreeMap<String, u32>,
}

#[test]
fn dog() {
  let dog = Dog {
    name: "Fred".into(),
    food_ratings: vec![("foo".into(), 1)].into_iter().collect(),
  };

  let bytes = dog.clone().into_bytes().unwrap();
  let restored_dog = Dog::from_bytes(&bytes).unwrap().0;

  assert_eq!(dog, restored_dog);
}
