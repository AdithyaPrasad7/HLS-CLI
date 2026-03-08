use aws_credential_types::Credentials;
use aws_sdk_s3::{Client};
use aws_sdk_s3::config::Region;
use aws_config::BehaviorVersion;
use aws_credential_types::provider::SharedCredentialsProvider;

use crate::model::awsSessionResponse::TemporaryCredentials;

pub async fn s3Client(creds: TemporaryCredentials) -> Client{

// println!("Access key: {}", creds.accessKey);
// println!("Secret key: {}", creds.secretKey);
// println!("Session token: {}", creds.sessionToken);
  let credentials = Credentials::new(
        creds.accessKey.clone(),
        creds.secretKey.clone(),
        Some(creds.sessionToken.clone()),
        None,
        "upload-session",
    );

    let config = aws_config::defaults(BehaviorVersion::latest())
    .region(Region::new("us-east-1"))
        .credentials_provider(SharedCredentialsProvider::new(credentials))
        .load()
        .await;

    Client::new(&config)
}