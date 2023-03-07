use base64::Engine;
use std::collections::HashMap;

const BASE64_ENGINE: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

/// The value for an attribute that comes from DynamoDb.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttributeValue {
    /// An attribute of type Number. For example:
    ///
    /// ```text
    /// "N": "123.45"
    /// ```
    ///
    /// Numbers are sent across the network to DynamoDB as strings, to maximize compatibility across
    /// languages and libraries. However, DynamoDB treats them as number type attributes for
    /// mathematical operations.
    N(String),
    /// An attribute of type String. For example:
    ///
    /// ```text
    /// "S": "Hello"
    /// ```
    S(String),
    /// An attribute of type Boolean. For example:
    ///
    /// ```text
    /// "BOOL": true
    /// ```
    Bool(bool),
    /// An attribute of type Binary. For example:
    ///
    /// ```text
    /// "B": "dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk"
    /// ```
    ///
    /// Type: Base64-encoded binary data object
    B(Vec<u8>),
    /// An attribute of type Null. For example:
    ///
    /// ```text
    /// "NULL": true
    /// ```
    Null(bool),
    /// An attribute of type Map. For example:
    ///
    /// ```text
    /// "M": {"Name": {"S": "Joe"}, "Age": {"N": "35"}}
    /// ```
    ///
    /// Key Length Constraints: Maximum length of 65535.
    M(HashMap<String, AttributeValue>),
    /// An attribute of type List. For example:
    ///
    /// ```text
    /// "L": [ {"S": "Cookies"} , {"S": "Coffee"}, {"N": "3.14159"}]
    /// ```
    L(Vec<AttributeValue>),
    /// An attribute of type String Set. For example:
    ///
    /// ```text
    /// "SS": ["Giraffe", "Hippo" ,"Zebra"]
    /// ```
    Ss(Vec<String>),
    /// An attribute of type Number Set. For example:
    ///
    /// ```text
    /// "NS": ["42.2", "-19", "7.5", "3.14"]
    /// ```
    ///
    /// Numbers are sent across the network to DynamoDB as strings, to maximize compatibility across
    /// languages and libraries. However, DynamoDB treats them as number type attributes for
    /// mathematical operations.
    Ns(Vec<String>),
    /// An attribute of type Binary Set. For example:
    ///
    /// ```text
    /// "BS": ["U3Vubnk=", "UmFpbnk=", "U25vd3k="]
    /// ```
    ///
    /// Type: Array of Base64-encoded binary data objects
    Bs(Vec<Vec<u8>>),
}

impl serde::Serialize for AttributeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        match self {
            AttributeValue::N(inner) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("N", inner)?;
                map.end()
            }
            AttributeValue::S(inner) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("S", inner)?;
                map.end()
            }
            AttributeValue::Bool(inner) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("BOOL", inner)?;
                map.end()
            }
            AttributeValue::B(inner) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("B", &BASE64_ENGINE.encode(inner))?;
                map.end()
            }
            AttributeValue::Null(inner) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("NULL", inner)?;
                map.end()
            }
            AttributeValue::M(inner) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("M", inner)?;
                map.end()
            }
            AttributeValue::L(inner) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("L", inner)?;
                map.end()
            }
            AttributeValue::Ss(inner) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("SS", inner)?;
                map.end()
            }
            AttributeValue::Ns(inner) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("NS", inner)?;
                map.end()
            }
            AttributeValue::Bs(inner) => {
                let mut map = serializer.serialize_map(Some(1))?;
                let items: Vec<String> = inner
                    .iter()
                    .map(|item| BASE64_ENGINE.encode(item))
                    .collect();
                map.serialize_entry("BS", &items)?;
                map.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for AttributeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AttributeValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(r#"an object with a single key "N", "S", "BOOL", "B", "NULL", "M", "L", "SS", "NS", or "BS""#)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                use serde::de::Error;

                let first_key: String = match map.next_key()? {
                    Some(key) => key,
                    None => {
                        return Err(A::Error::custom(
                            "Expected exactly one key in the object, found none",
                        ))
                    }
                };

                let attribute_value = match first_key.as_str() {
                    "N" => AttributeValue::N(map.next_value()?),
                    "S" => AttributeValue::S(map.next_value()?),
                    "BOOL" => AttributeValue::Bool(map.next_value()?),
                    "B" => {
                        let string: String = map.next_value()?;
                        let bytes = BASE64_ENGINE.decode(string).map_err(|err| {
                            A::Error::custom(format!("Failed to decode base64: {err}"))
                        })?;
                        AttributeValue::B(bytes)
                    }
                    "NULL" => AttributeValue::Null(map.next_value()?),
                    "M" => AttributeValue::M(map.next_value()?),
                    "L" => AttributeValue::L(map.next_value()?),
                    "SS" => AttributeValue::Ss(map.next_value()?),
                    "NS" => AttributeValue::Ns(map.next_value()?),
                    "BS" => {
                        let strings: Vec<String> = map.next_value()?;
                        let mut byte_entries = Vec::with_capacity(strings.len());
                        for string in strings {
                            let bytes = base64::engine::general_purpose::STANDARD
                                .decode(string)
                                .map_err(|err| {
                                    A::Error::custom(format!("Failed to decode base64: {err}"))
                                })?;
                            byte_entries.push(bytes);
                        }
                        AttributeValue::Bs(byte_entries)
                    }
                    key => {
                        return Err(A::Error::custom(format!(
                            "The key '{key}' is not a known DynamoDB prefix"
                        )))
                    }
                };

                if map.next_key::<String>()?.is_some() {
                    return Err(A::Error::custom(
                        "Expected exactly one key in the object, found multiple keys",
                    ));
                }

                Ok(attribute_value)
            }
        }

        let visitor = Visitor;
        deserializer.deserialize_map(visitor)
    }
}

impl<'de> serde::Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        HashMap::deserialize(deserializer).map(Item)
    }
}

impl serde::Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

/// An item that comes from DynamoDb.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Item(HashMap<String, AttributeValue>);

impl Item {
    /// Get a reference to the inner HashMap
    pub fn inner(&self) -> &HashMap<String, AttributeValue> {
        &self.0
    }

    /// Get a mutable reference to the inner HashMap
    pub fn inner_mut(&mut self) -> &mut HashMap<String, AttributeValue> {
        &mut self.0
    }

    /// Take the inner HashMap
    pub fn into_inner(self) -> HashMap<String, AttributeValue> {
        self.0
    }
}

impl AsRef<HashMap<String, AttributeValue>> for Item {
    fn as_ref(&self) -> &HashMap<String, AttributeValue> {
        self.inner()
    }
}

impl AsMut<HashMap<String, AttributeValue>> for Item {
    fn as_mut(&mut self) -> &mut HashMap<String, AttributeValue> {
        self.inner_mut()
    }
}

impl std::ops::Deref for Item {
    type Target = HashMap<String, AttributeValue>;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl std::ops::DerefMut for Item {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl<T> From<Item> for HashMap<String, T>
where
    T: From<AttributeValue>,
{
    fn from(Item(m): Item) -> Self {
        m.into_iter()
            .map(|(key, value)| (key, T::from(value)))
            .collect()
    }
}

impl<T> From<HashMap<String, T>> for Item
where
    AttributeValue: From<T>,
{
    fn from(m: HashMap<String, T>) -> Self {
        Item(
            m.into_iter()
                .map(|(key, value)| (key, AttributeValue::from(value)))
                .collect(),
        )
    }
}

/// Multiple items that come from DynamoDb.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Items(Vec<Item>);

impl<T> From<Items> for Vec<HashMap<String, T>>
where
    HashMap<String, T>: From<Item>,
{
    fn from(Items(items): Items) -> Self {
        items.into_iter().map(Into::into).collect()
    }
}

impl<T> From<Vec<HashMap<String, T>>> for Items
where
    Item: From<HashMap<String, T>>,
{
    fn from(items: Vec<HashMap<String, T>>) -> Self {
        Items(items.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_from_example() {
        // Example from https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/S3DataExport.Output.html
        let subject = json!({
            "Authors":{
                "SS":[
                    "Author1",
                    "Author2"
                ]
            },
            "Dimensions":{
                "S":"8.5 x 11.0 x 1.5"
            },
            "ISBN":{
                "S":"333-3333333333"
            },
            "Id":{
                "N":"103"
            },
            "InPublication":{
                "BOOL":false
            },
            "PageCount":{
                "N":"600"
            },
            "Price":{
                "N":"2000"
            },
            "ProductCategory":{
                "S":"Book"
            },
            "Title":{
                "S":"Book 103 Title"
            }
        });

        let item: Item =
            serde_json::from_value(subject).expect("expected successful deserialization");

        assert_eq!(
            item,
            Item(HashMap::from([
                (
                    String::from("Authors"),
                    AttributeValue::Ss(vec![String::from("Author1"), String::from("Author2")])
                ),
                (
                    String::from("Dimensions"),
                    AttributeValue::S(String::from("8.5 x 11.0 x 1.5"))
                ),
                (
                    String::from("ISBN"),
                    AttributeValue::S(String::from("333-3333333333"))
                ),
                (String::from("Id"), AttributeValue::N(String::from("103"))),
                (String::from("InPublication"), AttributeValue::Bool(false)),
                (
                    String::from("PageCount"),
                    AttributeValue::N(String::from("600"))
                ),
                (
                    String::from("Price"),
                    AttributeValue::N(String::from("2000"))
                ),
                (
                    String::from("ProductCategory"),
                    AttributeValue::S(String::from("Book"))
                ),
                (
                    String::from("Title"),
                    AttributeValue::S(String::from("Book 103 Title"))
                ),
            ]))
        )
    }

    #[test]
    fn deserialize_exhaustive() {
        let subject = json!({
            "n_example": { "N": "123.45" },
            "s_example": { "S": "Hello" },
            "bool_example": { "BOOL": true },
            "b_example": { "B": "dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk" },
            "null_example": { "NULL": true },
            "m_example": { "M": {"Name": {"S": "Joe"}, "Age": {"N": "35"}} },
            "l_example": { "L": [ {"S": "Cookies"} , {"S": "Coffee"}, {"N": "3.14159"}] },
            "ss_example": { "SS": ["Giraffe", "Hippo" ,"Zebra"] },
            "ns_example": { "NS": ["42.2", "-19", "7.5", "3.14"] },
            "bs_example": { "BS": ["U3Vubnk=", "UmFpbnk=", "U25vd3k="] },
        });

        let item: Item =
            serde_json::from_value(subject).expect("expected successful deserialization");

        assert_eq!(
            item,
            Item(HashMap::from([
                (
                    String::from("n_example"),
                    AttributeValue::N(String::from("123.45"))
                ),
                (
                    String::from("s_example"),
                    AttributeValue::S(String::from("Hello"))
                ),
                (String::from("bool_example"), AttributeValue::Bool(true)),
                (
                    String::from("b_example"),
                    AttributeValue::B(Vec::from(b"this text is base64-encoded".as_slice()))
                ),
                (String::from("null_example"), AttributeValue::Null(true)),
                (
                    String::from("m_example"),
                    AttributeValue::M(HashMap::from([
                        (String::from("Name"), AttributeValue::S(String::from("Joe"))),
                        (String::from("Age"), AttributeValue::N(String::from("35"))),
                    ]))
                ),
                (
                    String::from("l_example"),
                    AttributeValue::L(vec![
                        AttributeValue::S(String::from("Cookies")),
                        AttributeValue::S(String::from("Coffee")),
                        AttributeValue::N(String::from("3.14159"))
                    ])
                ),
                (
                    String::from("ss_example"),
                    AttributeValue::Ss(vec![
                        String::from("Giraffe"),
                        String::from("Hippo"),
                        String::from("Zebra")
                    ])
                ),
                (
                    String::from("ns_example"),
                    AttributeValue::Ns(vec![
                        String::from("42.2"),
                        String::from("-19"),
                        String::from("7.5"),
                        String::from("3.14")
                    ])
                ),
                (
                    String::from("bs_example"),
                    AttributeValue::Bs(vec![
                        Vec::from(b"Sunny".as_slice()),
                        Vec::from(b"Rainy".as_slice()),
                        Vec::from(b"Snowy".as_slice())
                    ])
                ),
            ]))
        );
    }

    #[test]
    fn deserialize_error_invalid_key() {
        let err = serde_json::from_str::<AttributeValue>(r#"{ "X": "1" }"#)
            .expect_err("expected to fail");
        assert!(err.to_string().contains("'X'"))
    }

    #[test]
    fn deserialize_error_zero_keys() {
        let err = serde_json::from_str::<AttributeValue>(r#"{}"#).expect_err("expected to fail");
        assert!(err.to_string().contains("none"))
    }

    #[test]
    fn deserialize_error_multiple_keys() {
        let err = serde_json::from_str::<AttributeValue>(r#"{ "S": "1", "N": "1" }"#)
            .expect_err("expected to fail");
        assert!(err.to_string().contains("multiple keys"))
    }

    #[test]
    fn deserialize_error_base64_b() {
        let err = serde_json::from_str::<AttributeValue>(r#"{ "B": "X" }"#)
            .expect_err("expected to fail");
        assert!(err.to_string().contains("base64"))
    }

    #[test]
    fn deserialize_error_base64_bs() {
        let err = serde_json::from_str::<AttributeValue>(r#"{ "BS": ["X"] }"#)
            .expect_err("expected to fail");
        assert!(err.to_string().contains("base64"))
    }

    #[test]
    fn deserialize_expecting() {
        let err = serde_json::from_str::<AttributeValue>(r#"42"#).expect_err("expected to fail");
        assert!(err
            .to_string()
            .contains("expected an object with a single key"));
    }

    #[test]
    fn serialize_exhaustive() {
        let subject = Item(HashMap::from([
            (
                String::from("n_example"),
                AttributeValue::N(String::from("123.45")),
            ),
            (
                String::from("s_example"),
                AttributeValue::S(String::from("Hello")),
            ),
            (String::from("bool_example"), AttributeValue::Bool(true)),
            (
                String::from("b_example"),
                AttributeValue::B(Vec::from(b"this text is base64-encoded".as_slice())),
            ),
            (String::from("null_example"), AttributeValue::Null(true)),
            (
                String::from("m_example"),
                AttributeValue::M(HashMap::from([
                    (String::from("Name"), AttributeValue::S(String::from("Joe"))),
                    (String::from("Age"), AttributeValue::N(String::from("35"))),
                ])),
            ),
            (
                String::from("l_example"),
                AttributeValue::L(vec![
                    AttributeValue::S(String::from("Cookies")),
                    AttributeValue::S(String::from("Coffee")),
                    AttributeValue::N(String::from("3.14159")),
                ]),
            ),
            (
                String::from("ss_example"),
                AttributeValue::Ss(vec![
                    String::from("Giraffe"),
                    String::from("Hippo"),
                    String::from("Zebra"),
                ]),
            ),
            (
                String::from("ns_example"),
                AttributeValue::Ns(vec![
                    String::from("42.2"),
                    String::from("-19"),
                    String::from("7.5"),
                    String::from("3.14"),
                ]),
            ),
            (
                String::from("bs_example"),
                AttributeValue::Bs(vec![
                    Vec::from(b"Sunny".as_slice()),
                    Vec::from(b"Rainy".as_slice()),
                    Vec::from(b"Snowy".as_slice()),
                ]),
            ),
        ]));

        let json = serde_json::to_value(subject).expect("expected successful deserialization");
        assert_eq!(
            json,
            json!({
                "n_example": { "N": "123.45" },
                "s_example": { "S": "Hello" },
                "bool_example": { "BOOL": true },
                "b_example": { "B": "dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk" },
                "null_example": { "NULL": true },
                "m_example": { "M": {"Name": {"S": "Joe"}, "Age": {"N": "35"}} },
                "l_example": { "L": [ {"S": "Cookies"} , {"S": "Coffee"}, {"N": "3.14159"}] },
                "ss_example": { "SS": ["Giraffe", "Hippo" ,"Zebra"] },
                "ns_example": { "NS": ["42.2", "-19", "7.5", "3.14"] },
                "bs_example": { "BS": ["U3Vubnk=", "UmFpbnk=", "U25vd3k="] },
            })
        );
    }
}
