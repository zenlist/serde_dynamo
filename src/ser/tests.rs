#![allow(clippy::float_cmp, clippy::redundant_clone)]

use super::*;
use maplit::hashmap;
use serde_derive::{Deserialize, Serialize};

macro_rules! assert_identical_json {
    ($expr:expr) => {
        assert_identical_json($expr, $expr);
    };
}

/// Assert that the expression is the same whether it is serialized directly, or serialized
/// first to json and then to an attribute value
fn assert_identical_json<T>(t1: T, t2: T)
where
    T: Serialize,
{
    let direct_result = to_attribute_value(t1).unwrap();
    let indirect_result = to_attribute_value(serde_json::to_value(t2).unwrap()).unwrap();
    assert_eq!(direct_result, indirect_result);
}

macro_rules! assert_identical_json_with_error {
    ($expr:expr) => {
        assert_identical_json_with_error($expr, $expr);
    };
}

fn assert_identical_json_with_error<T>(t1: T, t2: T)
where
    T: Serialize,
{
    let direct_result = to_attribute_value(t1);
    let json_result = serde_json::to_value(t2);

    match (direct_result, json_result) {
        (Ok(direct_result), Ok(json_result)) => match to_attribute_value(json_result) {
            Ok(indirect_result) => assert_eq!(direct_result, indirect_result),
            Err(_) => panic!("dynamo, json succeeded, indirect failed"),
        },
        (Ok(_), Err(_)) => panic!("dynamo succeeded, json failed"),
        (Err(_), Ok(_)) => panic!("dynamo failed, json succeeded"),
        (Err(_), Err(_)) => { /* Both failing is OK. */ }
    }
}

#[test]
fn serialize_string() {
    let result = to_attribute_value(String::from("Value")).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            s: Some(String::from("Value")),
            ..AttributeValue::default()
        }
    );
    assert_identical_json!(String::from("Value"));
}

#[test]
fn serialize_num() {
    macro_rules! serialize_num {
        ($ty:ty, $n:expr) => {{
            let v: $ty = $n;
            let result = to_attribute_value(v).unwrap();
            assert_eq!(
                result,
                AttributeValue {
                    n: Some(String::from(stringify!($n))),
                    ..AttributeValue::default()
                }
            );
        }};
    }

    serialize_num!(i8, -1);
    serialize_num!(u8, 1);
    serialize_num!(i16, -1);
    serialize_num!(u16, 1);
    serialize_num!(i32, -1);
    serialize_num!(u32, 1);
    serialize_num!(i64, -1);
    serialize_num!(u64, 1);
    serialize_num!(f32, 1.1);
    serialize_num!(f64, 1.1);
}

#[test]
fn serialize_bool() {
    let result = to_attribute_value(true).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            bool: Some(true),
            ..AttributeValue::default()
        }
    );
    assert_identical_json!(true);
}

#[test]
fn serialize_char() {
    let result = to_attribute_value('🥳').unwrap();
    assert_eq!(
        result,
        AttributeValue {
            s: Some(String::from("🥳")),
            ..AttributeValue::default()
        }
    );
    assert_identical_json!('🥳');
}

#[test]
fn serialize_unit() {
    let result = to_attribute_value(()).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            null: Some(true),
            ..AttributeValue::default()
        }
    );
    assert_identical_json!(());
}

#[test]
fn serialize_option() {
    let result = to_attribute_value(Some(1_u8)).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            n: Some(String::from("1")),
            ..AttributeValue::default()
        }
    );
    assert_identical_json!(Some(1_u8));

    let result = to_attribute_value(Option::<u8>::None).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            null: Some(true),
            ..AttributeValue::default()
        }
    );
    assert_identical_json!(Option::<u8>::None);
}

#[test]
fn serialize_struct() {
    #[derive(Clone, Serialize, Deserialize)]
    struct Subject {
        value: String,
    }

    let source = Subject {
        value: String::from("Value"),
    };

    let result = to_item(source.clone()).unwrap();
    assert_eq!(
        result,
        hashmap! {
            String::from("value") => AttributeValue { s: Some(String::from("Value")), ..AttributeValue::default() },
        }
    );
    assert_identical_json!(source.clone());
}

#[test]
fn serialize_bytes() {
    #[derive(Clone, Serialize, Deserialize)]
    struct Subject {
        #[serde(with = "serde_bytes")]
        value: Vec<u8>,
    }

    let source = Subject {
        value: vec![116, 101, 115, 116, 0, 0, 0, 0],
    };

    let result = to_item(source.clone()).unwrap();
    assert_eq!(
        result,
        hashmap! {
            String::from("value") => AttributeValue { b: Some(vec![116, 101, 115, 116, 0, 0, 0, 0].into()), ..AttributeValue::default() },
        }
    );
}

#[test]
fn serialize_array_of_structs() {
    #[derive(Clone, Serialize, Deserialize)]
    struct Subject {
        value: String,
    }

    let mut source = Vec::new();
    for i in 1..=3 {
        let s = Subject {
            value: i.to_string(),
        };
        source.push(s);
    }

    let result = to_attribute_value(source.clone()).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            l: Some(vec![
                AttributeValue {
                    m: Some(hashmap! {
                        String::from("value") => AttributeValue { s: Some(String::from("1")), ..AttributeValue::default() },
                    }),
                    ..AttributeValue::default()
                },
                AttributeValue {
                    m: Some(hashmap! {
                        String::from("value") => AttributeValue { s: Some(String::from("2")), ..AttributeValue::default() },
                    }),
                    ..AttributeValue::default()
                },
                AttributeValue {
                    m: Some(hashmap! {
                        String::from("value") => AttributeValue { s: Some(String::from("3")), ..AttributeValue::default() },
                    }),
                    ..AttributeValue::default()
                },
            ]),
            ..AttributeValue::default()
        },
    );
    assert_identical_json!(source.clone());
}

#[test]
fn serialize_unit_struct() {
    #[derive(Serialize, Deserialize)]
    struct Subject;

    let result = to_attribute_value(Subject).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            null: Some(true),
            ..AttributeValue::default()
        }
    );

    assert_identical_json!(Subject);
}

#[test]
fn serialize_newtype_struct() {
    #[derive(Serialize, Deserialize)]
    struct Subject(String);

    let result = to_attribute_value(Subject(String::from("one"))).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            s: Some(String::from("one")),
            ..AttributeValue::default()
        }
    );

    assert_identical_json!(Subject(String::from("one")));
}

#[test]
fn serialize_tuple_struct() {
    #[derive(Serialize, Deserialize)]
    struct Subject(String, String);

    let result = to_attribute_value(Subject(String::from("one"), String::from("two"))).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            l: Some(vec![
                AttributeValue {
                    s: Some(String::from("one")),
                    ..AttributeValue::default()
                },
                AttributeValue {
                    s: Some(String::from("two")),
                    ..AttributeValue::default()
                },
            ]),
            ..AttributeValue::default()
        }
    );

    assert_identical_json!(Subject(String::from("one"), String::from("two")));
}

#[test]
fn serialize_tuple() {
    let result = to_attribute_value((String::from("one"), String::from("two"))).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            l: Some(vec![
                AttributeValue {
                    s: Some(String::from("one")),
                    ..AttributeValue::default()
                },
                AttributeValue {
                    s: Some(String::from("two")),
                    ..AttributeValue::default()
                },
            ]),
            ..AttributeValue::default()
        }
    );

    assert_identical_json!((String::from("one"), String::from("two")));
}

#[test]
fn serialize_map_with_strings() {
    let result =
        to_attribute_value(hashmap! { String::from("one") => 1, String::from("two") => 2 })
            .unwrap();

    assert_eq!(
        result,
        AttributeValue {
            m: Some(hashmap! {
                String::from("one") => AttributeValue {
                    n: Some(String::from("1")),
                    ..AttributeValue::default()
                },
                String::from("two") => AttributeValue {
                    n: Some(String::from("2")),
                    ..AttributeValue::default()
                },
            }),
            ..AttributeValue::default()
        },
    );

    assert_identical_json!(hashmap! { String::from("one") => 1, String::from("two") => 2 });
}

#[test]
fn serialize_maps_with_various_types() {
    let result =
        to_attribute_value(hashmap! { 1 => String::from("1"), 2 => String::from("2") }).unwrap();

    assert_eq!(
        result,
        AttributeValue {
            m: Some(hashmap! {
                String::from("1") => AttributeValue {
                    s: Some(String::from("1")),
                    ..AttributeValue::default()
                },
                String::from("2") => AttributeValue {
                    s: Some(String::from("2")),
                    ..AttributeValue::default()
                },
            }),
            ..AttributeValue::default()
        },
    );

    assert_identical_json!(hashmap! { 1 => String::from("1"), 2 => String::from("2") });

    macro_rules! test_map {
        ($($expr:expr),*) => {
            assert_identical_json_with_error!(hashmap! {
                $(
                    $expr => String::from(stringify!($expr)),
                )*
            })
        }
    }

    test_map!(1_u8, 2_u8);
    test_map!(-1_i8, -2_i8);
    test_map!('a', 'b');

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    struct Struct(i64);
    test_map!(Struct(1), Struct(2));

    {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        /// Externally tagged
        enum VariantType {
            Unit,
            Newtype(String),
            Struct { value: String },
            Tuple(String, String),
        }

        test_map!(VariantType::Unit);
        test_map!(VariantType::Newtype(String::from("one")));
        test_map!(VariantType::Struct {
            value: String::from("one")
        });
        test_map!(VariantType::Tuple(String::from("one"), String::from("two")));
    }

    {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(tag = "type")]
        /// Internally tagged
        enum VariantType {
            Unit,
            Newtype(String),
            Struct { value: String },
        }

        test_map!(VariantType::Unit);
        test_map!(VariantType::Newtype(String::from("one")));
        test_map!(VariantType::Struct {
            value: String::from("one")
        });
    }

    {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(tag = "type", content = "content")]
        /// Adjacently tagged
        enum VariantType {
            Unit,
            Newtype(String),
            Struct { value: String },
            Tuple(String, String),
        }

        test_map!(VariantType::Unit);
        test_map!(VariantType::Newtype(String::from("one")));
        test_map!(VariantType::Struct {
            value: String::from("one")
        });
        test_map!(VariantType::Tuple(String::from("one"), String::from("two")));
    }

    {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(untagged)]
        /// Untagged
        enum VariantType {
            Unit,
            Newtype(String),
            Struct { value: String },
            Tuple(String, String),
        }

        test_map!(VariantType::Unit);
        test_map!(VariantType::Newtype(String::from("one")));
        test_map!(VariantType::Struct {
            value: String::from("one")
        });
        test_map!(VariantType::Tuple(String::from("one"), String::from("two")));
    }
}

#[test]
fn serialize_enum_unit() {
    #[derive(Serialize, Deserialize)]
    enum Subject {
        Unit,
    }

    let result = to_attribute_value(Subject::Unit).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            s: Some(String::from("Unit")),
            ..AttributeValue::default()
        }
    );

    assert_identical_json!(Subject::Unit);
}

#[test]
fn serialize_enum_newtype() {
    #[derive(Serialize, Deserialize)]
    enum Subject {
        Newtype(u8),
    }

    let result = to_attribute_value(Subject::Newtype(1)).unwrap();
    assert_eq!(
        result,
        AttributeValue {
            m: Some(hashmap! {
                String::from("Newtype") => AttributeValue {
                    n: Some(String::from("1")),
                    ..AttributeValue::default()
                },
            }),
            ..AttributeValue::default()
        }
    );

    assert_identical_json!(Subject::Newtype(1));
}

#[test]
fn serialize_enum_tuple() {
    #[derive(Serialize, Deserialize)]
    enum Subject {
        Tuple(u8, u8),
    }

    let result = to_attribute_value(Subject::Tuple(1, 2)).unwrap();

    assert_eq!(
        result,
        AttributeValue {
            m: Some(hashmap! {
                String::from("Tuple") => AttributeValue {
                    l: Some(vec![
                        AttributeValue {
                            n: Some(String::from("1")),
                            ..AttributeValue::default()
                        },
                        AttributeValue {
                            n: Some(String::from("2")),
                            ..AttributeValue::default()
                        },
                    ]),
                    ..AttributeValue::default()
                },
            }),
            ..AttributeValue::default()
        }
    );

    assert_identical_json!(Subject::Tuple(1, 2));
}

#[test]
fn serialize_enum_struct_variant() {
    #[derive(Serialize, Deserialize)]
    enum Subject {
        Structy { one: u8, two: u8 },
    }

    let result = to_attribute_value(Subject::Structy { one: 1, two: 2 }).unwrap();

    assert_eq!(
        result,
        AttributeValue {
            m: Some(hashmap! {
                String::from("Structy") => AttributeValue {
                    m: Some(hashmap! {
                        String::from("one") => AttributeValue {
                            n: Some(String::from("1")),
                            ..AttributeValue::default()
                        },
                        String::from("two") => AttributeValue {
                            n: Some(String::from("2")),
                            ..AttributeValue::default()
                        },
                    }),
                    ..AttributeValue::default()
                },
            }),
            ..AttributeValue::default()
        }
    );

    assert_identical_json!(Subject::Structy { one: 1, two: 2 });
}

#[test]
fn internally_tagged_enum() {
    #[derive(Serialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    enum Enum {
        One { one: u8 },
        Two { one: u8, two: u8 },
    }

    let result = to_attribute_value(Enum::Two { one: 1, two: 2 }).unwrap();

    assert_eq!(
        result,
        AttributeValue {
            m: Some(hashmap! {
                String::from("type") => AttributeValue {
                    s: Some(String::from("two")),
                    ..AttributeValue::default()
                },
                String::from("one") => AttributeValue {
                    n: Some(String::from("1")),
                    ..AttributeValue::default()
                },
                String::from("two") => AttributeValue {
                    n: Some(String::from("2")),
                    ..AttributeValue::default()
                },
            }),
            ..AttributeValue::default()
        }
    );

    assert_identical_json!(Enum::One { one: 1 });
    assert_identical_json!(Enum::Two { one: 1, two: 2 });
}
