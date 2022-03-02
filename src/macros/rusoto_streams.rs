macro_rules! rusoto_streams_macro {
    (feature = $feature:literal, crate_name = $mod_name:ident, rusoto_version = $version:literal,) => {
        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        pub mod $mod_name {
            #![doc = concat!("Support for [rusoto_dynamodbstreams](https://docs.rs/rusoto_dynamodbstreams/", $version, ") version ", $version)]
            //!
            //! Because [rusoto_dynamodbstreams] has not yet reached version 1.0, a feature is
            //! required to enable support. Add the following to your dependencies.
            //!
            //! ```toml
            //! [dependencies]
            #![doc = concat!("rusoto_core = { version = ", stringify!($version), ", default-features = false, features = [\"rustls\"] }")]
            #![doc = concat!("rusoto_dynamodbstreams = { version = ", stringify!($version), ", default-features = false, features = [\"rustls\"] }")]
            #![doc = concat!("serde_dynamo = { version = \"3\", features = [", stringify!($feature), "] }")]
            //! ```
            //!
            //! [rusoto_dynamodbstreams]: https://docs.rs/rusoto_dynamodbstreams

            use crate::Result;
            use ::$mod_name::AttributeValue;

            impl crate::AttributeValue for AttributeValue {
                fn is_n(&self) -> bool {
                    self.n.is_some()
                }

                fn is_s(&self) -> bool {
                    self.s.is_some()
                }

                fn is_bool(&self) -> bool {
                    self.bool.is_some()
                }

                fn is_b(&self) -> bool {
                    self.b.is_some()
                }

                fn is_null(&self) -> bool {
                    self.null.is_some()
                }

                fn is_m(&self) -> bool {
                    self.m.is_some()
                }

                fn is_l(&self) -> bool {
                    self.l.is_some()
                }

                fn is_ss(&self) -> bool {
                    self.ss.is_some()
                }

                fn is_ns(&self) -> bool {
                    self.ns.is_some()
                }

                fn is_bs(&self) -> bool {
                    self.bs.is_some()
                }

                fn as_n(&self) -> Option<&str> {
                    self.n.as_deref()
                }

                fn as_s(&self) -> Option<&str> {
                    self.s.as_deref()
                }

                fn as_bool(&self) -> Option<bool> {
                    self.bool
                }

                fn as_b(&self) -> Option<&[u8]> {
                    self.b.as_deref()
                }

                fn as_null(&self) -> Option<bool> {
                    self.null
                }

                fn as_m(&self) -> Option<&std::collections::HashMap<String, Self>> {
                    self.m.as_ref()
                }

                fn as_l(&self) -> Option<&[Self]> {
                    self.l.as_deref()
                }

                fn as_ss(&self) -> Option<&[String]> {
                    self.ss.as_deref()
                }

                fn as_ns(&self) -> Option<&[String]> {
                    self.ns.as_deref()
                }

                fn into_n(self) -> Option<String> {
                    self.n
                }

                fn into_s(self) -> Option<String> {
                    self.s
                }

                fn into_bool(self) -> Option<bool> {
                    self.bool
                }

                fn into_b(self) -> Option<Vec<u8>> {
                    self.b.map(|bytes| bytes.into_iter().collect())
                }

                fn into_null(self) -> Option<bool> {
                    self.null
                }

                fn into_m(self) -> Option<std::collections::HashMap<String, Self>> {
                    self.m
                }

                fn into_l(self) -> Option<Vec<Self>> {
                    self.l
                }

                fn into_ss(self) -> Option<Vec<String>> {
                    self.ss
                }

                fn into_ns(self) -> Option<Vec<String>> {
                    self.ns
                }

                fn into_bs(self) -> Option<Vec<Vec<u8>>> {
                    self.bs.map(|vec| {
                        vec.into_iter()
                            .map(|bytes| bytes.into_iter().collect())
                            .collect()
                    })
                }

                fn construct_n(input: String) -> Self {
                    Self {
                        n: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_s(input: String) -> Self {
                    Self {
                        s: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_bool(input: bool) -> Self {
                    Self {
                        bool: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_b(input: &[u8]) -> Self {
                    Self {
                        b: Some(input.to_vec().into()),
                        ..Default::default()
                    }
                }

                fn construct_null(input: bool) -> Self {
                    Self {
                        null: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_m(input: std::collections::HashMap<String, Self>) -> Self {
                    Self {
                        m: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_l(input: Vec<Self>) -> Self {
                    Self {
                        l: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_ss(input: Vec<String>) -> Self {
                    Self {
                        ss: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_ns(input: Vec<String>) -> Self {
                    Self {
                        ns: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_bs(input: Vec<Vec<u8>>) -> Self {
                    let input = input.into_iter().map(|vec| vec.into()).collect();
                    Self {
                        bs: Some(input),
                        ..Default::default()
                    }
                }
            }

            /// A version of [`crate::to_attribute_value`] where the `Tout` generic is tied to
            /// [`rusoto_dynamodbstreams::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `Tout`.
            pub fn to_attribute_value<T>(value: T) -> Result<AttributeValue>
            where
                T: serde::ser::Serialize,
            {
                crate::ser::to_attribute_value(value)
            }

            /// A version of [`crate::to_item`] where the `Tout` generic is tied to
            /// [`rusoto_dynamodbstreams::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `Tout`.
            pub fn to_item<T>(value: T) -> Result<std::collections::HashMap<String, AttributeValue>>
            where
                T: serde::ser::Serialize,
            {
                crate::ser::to_item(value)
            }

            /// A version of [`crate::from_attribute_value`] where the `Tin` generic is tied to
            /// [`rusoto_dynamodbstreams::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `Tin`.
            pub fn from_attribute_value<'a, T>(attribute_value: AttributeValue) -> Result<T>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_attribute_value(attribute_value)
            }

            /// A version of [`crate::from_item`] where the `Tin` generic is tied to
            /// [`rusoto_dynamodbstreams::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `Tin`.
            pub fn from_item<'a, T>(
                item: std::collections::HashMap<String, AttributeValue>,
            ) -> Result<T>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_item(item)
            }

            /// A version of [`crate::from_items`] where the `Tin` generic is tied to
            /// [`rusoto_dynamodbstreams::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `Tin`.
            pub fn from_items<'a, T>(
                items: Vec<std::collections::HashMap<String, AttributeValue>>,
            ) -> Result<Vec<T>>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_items(items)
            }
        }
    };
}

pub(crate) use rusoto_streams_macro;
