use std::fs;

use chrono::Local;
use tracing_appender::rolling;
use uuid::Uuid;
use once_cell::sync::OnceCell;
use tracing_appender::non_blocking::WorkerGuard;

static LOG_GUARD: OnceCell<WorkerGuard> = OnceCell::new();

pub fn init_logging() {
  fs::create_dir_all("logs").expect("Failed to create logs directory");

  let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
  let uuid = Uuid::new_v4();
  let log_file_name = format!("log_{}_{}.log", timestamp, uuid);

  let file_appender = rolling::never("logs", log_file_name);
  let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

  LOG_GUARD.set(guard).ok();

  tracing_subscriber::fmt()
      .with_writer(non_blocking)
      .with_ansi(false)
      .init();
}