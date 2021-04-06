use super::*;
use serde_derive::{Deserialize, Serialize};

fn round_trip<T>(value: T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + Eq + Clone + std::fmt::Debug,
{
    let serialized = to_item(value.clone()).unwrap();
    let deserialized: T = from_item(serialized).unwrap();
    assert_eq!(deserialized, value);
}

#[test]
fn internally_tagged_enum() {
    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    #[serde(tag = "type", rename_all = "snake_case")]
    enum Subject {
        One { one: u8 },
        Two { two: u8 },
    }

    round_trip(Subject::One { one: 1 });
    round_trip(Subject::Two { two: 2 });
}

#[test]
fn adjacently_tagged_enum() {
    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    #[serde(tag = "type", content = "value", rename_all = "snake_case")]
    enum Subject {
        One { one: u8 },
        Two { two: u8 },
    }

    round_trip(Subject::One { one: 1 });
    round_trip(Subject::Two { two: 2 });
}

#[test]
fn untagged_enum() {
    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    #[serde(untagged)]
    enum Simple {
        One { one: u8 },
        Two { two: u8 },
    }

    round_trip(Simple::One { one: 1 });
    round_trip(Simple::Two { two: 2 });

    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    #[serde(untagged)]
    enum Overlapping {
        Two { one: u8, two: u8 },
        Three { one: u8, three: u8 },
    }

    round_trip(Overlapping::Two { one: 1, two: 2 });
    round_trip(Overlapping::Three { one: 1, three: 3 });
}

#[test]
fn subsequent_flattened() {
    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    struct Subject {
        id: u64,
        #[serde(flatten)]
        email: Option<Email>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    struct Email {
        to: String,
        subject: String,
        body: String,
    }

    round_trip(Subject { id: 1, email: None });
    round_trip(Subject {
        id: 1,
        email: Some(Email {
            to: String::from("example@example.com"),
            subject: String::from("INFO!"),
            body: String::from("Some informational text"),
        }),
    });
}

#[test]
fn error_eq() {
    use super::{Error, ErrorImpl};

    assert_eq!(
        Into::<Error>::into(ErrorImpl::Message(String::from("one"))),
        Into::<Error>::into(ErrorImpl::Message(String::from("one"))),
    );

    assert_ne!(
        Into::<Error>::into(ErrorImpl::Message(String::from("one"))),
        Into::<Error>::into(ErrorImpl::Message(String::from("two"))),
    );
}

#[cfg(test)]
mod from_items {
    use maplit::hashmap;
    use serde_derive::{Deserialize, Serialize};

    use crate::*;

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct User {
        id: String,
        name: String,
        age: u8,
    }

    #[test]
    fn same_types() {
        let items = vec![
            hashmap! {
                String::from("id") => to_attribute_value("one").unwrap(),
                String::from("name") => to_attribute_value("Jane").unwrap(),
                String::from("age") => to_attribute_value(20).unwrap(),
            },
            hashmap! {
                String::from("id") => to_attribute_value("two").unwrap(),
                String::from("name") => to_attribute_value("John").unwrap(),
                String::from("age") => to_attribute_value(7).unwrap(),
            },
        ];

        let users = from_items(items).unwrap();

        assert_eq!(
            vec![
                User {
                    id: String::from("one"),
                    name: String::from("Jane"),
                    age: 20,
                },
                User {
                    id: String::from("two"),
                    name: String::from("John"),
                    age: 7,
                },
            ],
            users
        );
    }

    #[test]
    fn wrong_types() {
        let items = vec![
            hashmap! {
                String::from("id") => to_attribute_value("one").unwrap(),
                String::from("name") => to_attribute_value("Jane").unwrap(),
                String::from("age") => to_attribute_value(20).unwrap(),
            },
            hashmap! {
                String::from("id") => to_attribute_value(42).unwrap(),
                String::from("name") => to_attribute_value("John").unwrap(),
                String::from("age") => to_attribute_value("not a number").unwrap(),
            },
        ];

        let err = from_items::<Vec<User>>(items).unwrap_err();
        assert_eq!(Into::<Error>::into(ErrorImpl::ExpectedSeq), err);
    }
}

// Tests for various types being used as map keys
#[cfg(test)]
mod map_key {
    use std::{fmt::Debug, hash::Hash};

    use maplit::hashmap;
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};

    use crate::{error::ErrorImpl, from_attribute_value, to_attribute_value, Result};

    /// The provided `key` value is used as a map key that gets serialized, then deserialized.
    /// The provided `Result` indicates whether serializing should be able to be successful or not.
    /// If serializing is expected to be successful, then deserialization is also expected to be.
    ///
    /// Success/failure of serialization is compared with serialization of the map key with
    /// `serde_json`, as well. If `serde_json` is able to serialize the map key, then it's expected
    /// that this crate should, as well.
    ///
    /// An `Ok(&str)` value is compared against a successful serialization, and the result of
    /// deserialization is compared against the original value provided in `key`.
    ///
    /// An `Err(E)` value is compared against the error returned a serialization failure.
    ///
    /// If `Ok(&str)` is provided and there is an error, this panics.
    ///
    /// If `Err(E)` is provided and there is no error, this panics.
    fn map_key_round_trip<K>(key: K, expect_serialized_key: Result<&str>)
    where
        K: Debug + Clone + Eq + Hash + Serialize + DeserializeOwned,
    {
        let original = hashmap! { key => String::from("value") };

        let original_as_json = match serde_json::to_string(&original) {
            Ok(original_as_json) => {
                if expect_serialized_key.is_err() {
                    panic!(
                        "Expecting to get an error serializing {:?} to AttributeValue, \
                        but it was able to be serialized to JSON: {}",
                        original, original_as_json
                    );
                }

                original_as_json
            }
            Err(err) => {
                if expect_serialized_key.is_ok() {
                    panic!(
                        "Expecting to be able to serialize {:?} to AttributeValue, \
                        but it could not be serialized to JSON: {}",
                        original, err,
                    );
                }

                format!("unsupported by serde_json: {}", err)
            }
        };

        println!("{:?} as JSON: {}", original, original_as_json);

        let actual_serialized = to_attribute_value(original.clone());

        let (expected_serialized_key, actual_serialized) = match expect_serialized_key {
            Ok(expected_serialized) => (
                expected_serialized,
                actual_serialized.unwrap_or_else(|err| {
                    panic!(
                        "Failed to serialize to AttributeValue: {}\n\n\
                    The JSON representation would be:\n{}\n",
                        err, original_as_json,
                    )
                }),
            ),
            Err(expected_err) => {
                assert_eq!(
                    expected_err,
                    actual_serialized.expect_err("Expected an error when serializing"),
                    "Did not get the expected error when serializing"
                );
                return;
            }
        };

        let expected_serialized = to_attribute_value(hashmap! {
            expected_serialized_key => "value",
        })
        .expect("Failed to serialize expected value");
        assert_eq!(
            expected_serialized, actual_serialized,
            "Serialized value is not what was expected"
        );

        println!(
            "{} as dynamo item: {:?}",
            original_as_json, expected_serialized
        );

        let deserialized = from_attribute_value(actual_serialized.clone()).unwrap_or_else(|err| {
            panic!(
                "Failed to deserialize: {}\nThe serialized value was:\n{:#?}\n",
                err, actual_serialized
            )
        });
        assert_eq!(
            original, deserialized,
            "Deserialized value is not what was expected"
        );
    }

    fn key_must_be_a_string<T>() -> Result<T> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }

    // Tests using different types of enum variants as map keys.
    // See: https://serde.rs/enum-representations.html
    mod enum_variant {
        use super::*;

        // https://serde.rs/enum-representations.html#externally-tagged
        mod externally_tagged {
            use super::*;

            #[derive(
                Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
            )]
            enum VariantType {
                Unit,
                Newtype(String),
                Struct { value: String },
                Tuple(String, String),
            }

            #[test]
            fn unit_variant() {
                map_key_round_trip(VariantType::Unit, Ok("Unit"));
            }

            #[test]
            fn newtype_variant() {
                map_key_round_trip(
                    VariantType::Newtype(String::from("newtype")),
                    key_must_be_a_string(),
                );
            }

            #[test]
            fn struct_variant() {
                map_key_round_trip(
                    VariantType::Struct {
                        value: String::from("STRUCT VALUE"),
                    },
                    key_must_be_a_string(),
                );
            }

            #[test]
            fn tuple_struct() {
                map_key_round_trip(
                    VariantType::Tuple(String::from("TUPLE.0"), String::from("TUPLE.1")),
                    key_must_be_a_string(),
                );
            }
        }

        // https://serde.rs/enum-representations.html#internally-tagged
        mod internally_tagged {
            use super::*;

            #[derive(
                Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
            )]
            #[serde(tag = "type")]
            enum VariantType {
                Unit,
                Newtype(String),
                Struct { value: String },
                // Tuple(String, String), // serde causes a compile error for this one.
            }

            #[test]
            fn unit_variant() {
                map_key_round_trip(VariantType::Unit, key_must_be_a_string());
            }

            #[test]
            fn newtype_variant() {
                map_key_round_trip(
                    VariantType::Newtype(String::from("newtype")),
                    Err(<ErrorImpl as serde::ser::Error>::custom(
                        "cannot serialize tagged newtype variant VariantType::Newtype containing a string"
                    ).into()),
                );
            }

            #[test]
            fn struct_variant() {
                map_key_round_trip(
                    VariantType::Struct {
                        value: String::from("STRUCT VALUE"),
                    },
                    key_must_be_a_string(),
                );
            }
        }

        // https://serde.rs/enum-representations.html#adjacently-tagged
        mod adjacently_tagged {
            use super::*;

            #[derive(
                Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
            )]
            #[serde(tag = "type", content = "content")]
            enum VariantType {
                Unit,
                Newtype(String),
                Struct { value: String },
                Tuple(String, String),
            }

            #[test]
            fn unit_variant() {
                map_key_round_trip(VariantType::Unit, key_must_be_a_string());
            }

            #[test]
            fn newtype_variant() {
                map_key_round_trip(
                    VariantType::Newtype(String::from("newtype")),
                    key_must_be_a_string(),
                );
            }

            #[test]
            fn struct_variant() {
                map_key_round_trip(
                    VariantType::Struct {
                        value: String::from("STRUCT VALUE"),
                    },
                    key_must_be_a_string(),
                );
            }

            #[test]
            fn tuple_struct() {
                map_key_round_trip(
                    VariantType::Tuple(String::from("TUPLE.0"), String::from("TUPLE.1")),
                    key_must_be_a_string(),
                );
            }
        }

        // https://serde.rs/enum-representations.html#untagged
        mod untagged {
            use super::*;

            #[derive(
                Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
            )]
            #[serde(untagged)]
            enum VariantType {
                Unit,
                Newtype(String),
                Struct { value: String },
                Tuple(String, String),
            }

            #[test]
            fn unit_variant() {
                map_key_round_trip(VariantType::Unit, key_must_be_a_string());
            }

            #[test]
            fn newtype_variant() {
                map_key_round_trip(VariantType::Newtype(String::from("newtype")), Ok("newtype"));
            }

            #[test]
            fn struct_variant() {
                map_key_round_trip(
                    VariantType::Struct {
                        value: String::from("STRUCT VALUE"),
                    },
                    key_must_be_a_string(),
                );
            }

            #[test]
            fn tuple_struct() {
                map_key_round_trip(
                    VariantType::Tuple(String::from("TUPLE.0"), String::from("TUPLE.1")),
                    key_must_be_a_string(),
                );
            }
        }
    }

    #[test]
    fn i8() {
        map_key_round_trip(5_i8, Ok("5"));
    }

    #[test]
    fn u8() {
        map_key_round_trip(5_u8, Ok("5"));
    }

    #[test]
    fn i16() {
        map_key_round_trip(5_i16, Ok("5"));
    }

    #[test]
    fn u16() {
        map_key_round_trip(5_u16, Ok("5"));
    }

    #[test]
    fn i32() {
        map_key_round_trip(5_i32, Ok("5"));
    }

    #[test]
    fn u32() {
        map_key_round_trip(5_u32, Ok("5"));
    }

    #[test]
    fn i64() {
        map_key_round_trip(5_i64, Ok("5"));
    }

    #[test]
    fn u64() {
        map_key_round_trip(5_u64, Ok("5"));
    }

    #[test]
    fn i128() {
        map_key_round_trip(5_i128, Ok("5"));
    }

    #[test]
    fn u128() {
        map_key_round_trip(5_u128, Ok("5"));
    }

    #[test]
    fn bool() {
        map_key_round_trip(true, key_must_be_a_string());
    }

    #[test]
    fn char() {
        map_key_round_trip('a', Ok("a"));
    }

    #[test]
    fn none() {
        map_key_round_trip(Option::<()>::None, key_must_be_a_string());
    }

    #[test]
    fn some() {
        map_key_round_trip(Some(String::from("a")), key_must_be_a_string());
    }

    #[test]
    fn tuple() {
        map_key_round_trip((), key_must_be_a_string());
    }

    #[test]
    fn struct_() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        struct Struct {}

        map_key_round_trip(Struct {}, key_must_be_a_string());
    }

    #[test]
    fn unit_struct() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        struct Struct;

        map_key_round_trip(Struct {}, key_must_be_a_string());
    }

    #[test]
    fn tuple_struct() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        struct Struct(String, String);

        map_key_round_trip(
            Struct(String::from("a"), String::from("b")),
            key_must_be_a_string(),
        );
    }

    // TODO: Make this not panic (`unreachable!()`)
    // #[test]
    // fn newtype_struct() {
    //     #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    //     struct Struct(i64);

    //     map_key_round_trip(Struct(5), Ok("5"));
    // }
}
