use aws_sdk_s3::{Client, primitives::ByteStream};
use std::path::Path;
use tokio::fs;

use crate::config::DynError;

pub async fn uploadFile(client: &Client, bucket: &str, file: String, path: &Path) -> Result<(), DynError> {

  let body = ByteStream::from_path(path.to_path_buf()).await?;
  client
    .put_object()
    .bucket(bucket)
    .key(&file)
    .body(body)
    .send()
    .await?;

  Ok(())
}