use std::collections::HashMap;

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

/// An item that comes from DynamoDb.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Item(HashMap<String, AttributeValue>);

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

impl<T> From<&HashMap<String, T>> for Item
where
    AttributeValue: From<T>,
    T: Clone,
{
    fn from(m: &HashMap<String, T>) -> Self {
        Item(
            m.into_iter()
                .map(|(key, value)| (key.clone(), AttributeValue::from(value.clone())))
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
    use crate::from_item;
    use serde_json::Value;

    #[test]
    fn from_item_aws_sdk_works() {
        let mut item = HashMap::new();
        item.insert("key".to_string(), AttributeValue::S("val".to_string()));

        let _deserialized: Value = from_item(&item).unwrap();
    }
}
