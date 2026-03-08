use walkdir::WalkDir;
use futures::future::join_all;
use std::path::Path;
use aws_sdk_s3::Client;

use crate::config::DynError;

use super::uploadFile::uploadFile;

pub async fn uploadFiles(client: &Client, bucket: &str, prefix: &str, outputPath: &str,) -> Result<(), DynError> {
  let mut tasks = vec![];
  for entry in WalkDir::new(outputPath) {
    let entry = entry?;
    if entry.file_type().is_file() {

      let path = entry.path().to_owned();
      let relative = path.strip_prefix(outputPath)?;
      let key = format!("{}/{}", prefix, relative.display());
      let display = relative.display().to_string();

      let clientClone = client.clone();
      let bucketClone = bucket.to_string();

      let task = tokio::spawn(async move {
        let result = uploadFile(&clientClone, &bucketClone, key, &path).await;
        if result.is_ok() {
            println!("Uploaded {}", display);
        }
        result
      });

      tasks.push(task);
    }
  }

  let results = join_all(tasks).await;

  let mut errors = vec![];

  for res in results {
    match res {
      Ok(inner) => {
        if let Err(e) = inner {
          errors.push(e);
        }
      }
      Err(e) => {
        errors.push(Box::new(e));
      }
    }
  }

  if !errors.is_empty() {
    println!("Some file uploads failed:");

    for e in errors {
      println!(" - {}", e);
    }

    return Err("Upload failed".into());
  }

  println!("All files uploaded successfully");

  Ok(())
}