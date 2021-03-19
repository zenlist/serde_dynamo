[DynamoDB] is an AWS database that stores key/value and document data.

The most common way to access DynamoDB data from Rust is to use
[rusoto_dynamodb]'s [get_item], [put_item], and related methods.

**serde_dynamo** provides a way to serialize and deserialize between data
stored in these items and strongly-typed Rust data structures.


## You may be looking for

* [serde_dynamo on crates.io](https://crates.io/crates/serde_dynamo)
* [serde_dynamo on docs.rs](https://docs.rs/serde_dynamo)
* [serde_dynamo on GitHub](https://github.com/zenlist/serde_dynamo)


## Examples

See [the docs](https://docs.rs/serde_dynamo) for more examples.

### Parsing items as strongly-typed data structures.


Items received from a [rusoto_dynamodb] call can be run through `from_items`.

```rust
#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
    age: u8,
};

// Get documents from DynamoDB
let input = ScanInput {
    table_name: "users".to_string(),
    ..ScanInput::default()
};
let result = client.scan(input).await?;

// And deserialize them as strongly-typed data structures
if let Some(items) = result.items {
    let users: Vec<User> = from_items(items)?;
    println!("Got {} users", users.len());
}
```

Alternatively, to deserialize one item at a time, `from_item` can be used.

```rust
for item in result.items.unwrap() {
    let user: User = from_item(item)?;
    println!("{} is {}", user.name, user.age);
}
```


## Creating items by serializing data structures

Writing an entire data structure to DynamoDB typically involves using `to_item` to serialize
it.

```rust
#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
    age: u8,
};

// Create a user
let user = User {
    id: "fSsgVtal8TpP".to_string(),
    name: "Arthur Dent".to_string(),
    age: 42,
};

// Turn it into an item that rusoto understands
let item = to_item(user)?;

// And write it!
let input = PutItemInput {
    table_name: "users".to_string(),
    item: item,
    ..PutItemInput::default()
};
client.put_item(input).await?;
```


## How serde_dynamo compares to serde_dynamodb

[serde_dynamodb] is an effective library for serializing and deserializing data
from [rusoto_dynamodb].

However, serde_dynamodb is unable to handle some of the more advanced features
of [serde] – especially features like [flattening], [adjacently tagged enums],
and [untagged enums] – that we would like to use.

We opted to create a new library instead of making changes to serde_dynamodb
because making the changes to support these features would cause serde_dynamodb
to be backward-incompatible. Specifically, for certain cases, serde_dynamo and
serde_dynamodb make different choices on how to serialize the exact same
serializable structure.


[DynamoDB]: https://aws.amazon.com/dynamodb/
[serde]: https://serde.rs
[serde_dynamodb]: https://docs.rs/serde_dynamodb
[rusoto_dynamodb]: https://docs.rs/rusoto_dynamodb
[get_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.get_item
[write_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.write_item
[put_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.put_item
[flattening]: https://serde.rs/attr-flatten.html
[adjacently tagged enums]: https://serde.rs/enum-representations.html#adjacently-tagged
[untagged enums]: https://serde.rs/enum-representations.html#untagged
