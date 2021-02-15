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
    };

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
    };

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
    };

    round_trip(Simple::One { one: 1 });
    round_trip(Simple::Two { two: 2 });

    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    #[serde(untagged)]
    enum Overlapping {
        Two { one: u8, two: u8 },
        Three { one: u8, three: u8 },
    };

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
