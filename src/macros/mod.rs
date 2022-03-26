mod aws_sdk;
mod aws_sdk_streams;
mod rusoto;
mod rusoto_streams;

pub(crate) use aws_sdk::aws_sdk_macro;
pub(crate) use aws_sdk_streams::aws_sdk_streams_macro;
pub(crate) use rusoto::rusoto_macro;
pub(crate) use rusoto_streams::rusoto_streams_macro;
