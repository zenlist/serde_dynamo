[DynamoDB] is an AWS database that stores key/value and document data.

**serde_dynamo** provides a way to serialize and deserialize between data
stored in these items and strongly-typed Rust data structures.


## You may be looking for

* [serde_dynamo on crates.io](https://crates.io/crates/serde_dynamo)
* [serde_dynamo on docs.rs](https://docs.rs/serde_dynamo)
* [serde_dynamo on GitHub](https://github.com/zenlist/serde_dynamo)


## Features

Support for [aws-sdk-dynamodb], [aws_lambda_events], and [rusoto_dynamodb] is
provided via features. See [the docs](https://docs.rs/serde_dynamo) for more
details.


## Examples

See [the docs](https://docs.rs/serde_dynamo) for more examples.


### Parsing items as strongly-typed data structures.


Items received from a [aws-sdk-dynamodb] call can be run through `from_items`.

```rust
#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
    age: u8,
};

// Get documents from DynamoDB
let result = client.scan().table_name("user").send().await?;

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


### Creating items by serializing data structures

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

// Turn it into an item that aws-sdk-dynamodb understands
let item = to_item(user)?;

// And write it!
client.put_item().table_name("users").set_item(Some(item)).send().await?;
```

[DynamoDB]: https://aws.amazon.com/dynamodb/
[serde]: https://serde.rs
[aws-sdk-dynamodb]: https://docs.rs/aws-sdk-dynamodb
[rusoto_dynamodb]: https://docs.rs/rusoto_dynamodb
[aws_lambda_events]: https://docs.rs/aws_lambda_events
