use std::collections::HashMap;

/// The value for an attribute that comes from DynamoDb.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttributeValue {
    /// Number
    N(String),
    /// String
    S(String),
    /// Boolean
    Bool(bool),
    /// Bytes
    B(Vec<u8>),
    /// Null
    Null(bool),
    /// Map
    M(HashMap<String, AttributeValue>),
    /// List
    L(Vec<AttributeValue>),
    /// String list
    Ss(Vec<String>),
    /// Number list
    Ns(Vec<String>),
    /// Byte string list
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
