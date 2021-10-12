extern crate aws_sdk_dynamodb as ext_aws_sdk_dynamodb;

use ext_aws_sdk_dynamodb::{model::AttributeValue, Blob};
use maplit::hashmap;
use serde_json::{json, Value};

use super::{from_attribute_value, from_item, to_attribute_value, to_item, Item};

#[test]
fn value() {
    fn check(av: AttributeValue, json: Value) {
        let js_value: Value = from_attribute_value(av.clone())
            .expect(&format!("Should be able to serialize to JSON: {:?}", av));
        assert_eq!(
            json, js_value,
            "Did not get expected JSON value for {:?}",
            av
        );

        let js = serde_json::to_string_pretty(&js_value)
            .unwrap_or_else(|_| panic!("Should be able to serialize to string: {:?}", av));

        println!("{}", js);

        let de: AttributeValue =
            to_attribute_value(js).expect("Should be able to convert back to Value");
        assert_eq!(av, de, "Should get back what we started with");
    }

    check(
        AttributeValue::B(Blob::new("this text is base64-encoded")),
        json!({"B": "dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk"}),
    );
    check(AttributeValue::Bool(true), json!({"BOOL": true}));
    check(
        AttributeValue::Bs(vec![
            Blob::new("Sunny"),
            Blob::new("Rainy"),
            Blob::new("Snowy"),
        ]),
        json!({"BS": ["U3Vubnk=", "UmFpbnk=", "U25vd3k="]}),
    );
    check(
        AttributeValue::L(vec![
            AttributeValue::S(String::from("A")),
            AttributeValue::N(String::from("3.14")),
        ]),
        json!({"L": [{"S": "A"}, {"N": "3.14"}]}),
    );
    check(
        AttributeValue::M(hashmap! {
            String::from("A") => AttributeValue::N(String::from("1")),
            String::from("B") => AttributeValue::N(String::from("2")),
        }),
        json!({"M": {"A": {"N": "1"}, "B": {"N": "2"}}}),
    );
    check(AttributeValue::N(String::from("1")), json!({"N": "1"}));
    check(
        AttributeValue::Ns(vec![String::from("1"), String::from("2")]),
        json!({"NS": ["1", "2"]}),
    );
    check(AttributeValue::Null(true), json!({"NULL": true}));
    check(AttributeValue::S(String::from("A")), json!({"S": "A"}));
    check(
        AttributeValue::Ss(vec![String::from("A"), String::from("B")]),
        json!({"SS": ["A", "B"]}),
    );
}

#[test]
fn item() {
    let expected: Item = hashmap! {
        "a" => AttributeValue::B(Blob::new("this text is base64-encoded")),
        "b" => AttributeValue::Bool(true),
        "c" => AttributeValue::Bs(vec![
            Blob::new("Sunny"),
            Blob::new("Rainy"),
            Blob::new("Snowy"),
        ]),
        "d" => AttributeValue::L(vec![
            AttributeValue::S(String::from("A")),
            AttributeValue::N(String::from("3.14")),
        ]),
        "e" => AttributeValue::M(hashmap!{
            String::from("A") => AttributeValue::N(String::from("1")),
            String::from("B") => AttributeValue::N(String::from("2")),
        }),
        "f" => AttributeValue::N(String::from("1")),
        "g" => AttributeValue::Ns(vec![String::from("1"), String::from("2")]),
        "h" => AttributeValue::Null(true),
        "i" => AttributeValue::S(String::from("A")),
        "j" => AttributeValue::Ss(vec![String::from("A"), String::from("B")]),
    }
    .into_iter()
    .map(|(k, v)| (String::from(k), v))
    .collect();

    let js_value: Value = from_item(expected.clone()).expect("Should be able to serialize to JSON");
    let js =
        serde_json::to_string_pretty(&js_value).expect("Should be able to serialize to string");

    println!("{}", js);

    let de: Item = to_item(js_value).expect("Should be able to deserialize back to Item");
    assert_eq!(expected, de.into(), "Should get back what we started with");
}
