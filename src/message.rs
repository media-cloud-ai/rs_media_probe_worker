use crate::MediaProbeParameters;
use log::LevelFilter;
use mcai_worker_sdk::job::{JobResult, JobStatus};
use mcai_worker_sdk::{McaiChannel, MessageError};
use stainless_ffmpeg::probe::Probe;

pub fn process(
  _channel: Option<McaiChannel>,
  parameters: MediaProbeParameters,
  job_result: JobResult,
) -> Result<JobResult, MessageError> {
  let result = probe(&parameters.source_path).map_err(|error| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&error),
    )
  })?;

  Ok(
    job_result
      .with_status(JobStatus::Completed)
      .with_message(&result),
  )
}

fn probe(source_path: &str) -> Result<String, String> {
  let mut probe = Probe::new(&source_path);
  probe
    .process(LevelFilter::Off)
    .map_err(|error| format!("Unable to process probe: {}", error))?;

  match probe.format {
    Some(_) => serde_json::to_string(&probe)
      .map_err(|error| format!("Unable to serialize probe result: {:?}", error)),
    None => Err(format!("No such file: '{}'", source_path)),
  }
}

#[test]
pub fn test_probe_empty_path() {
  let result = probe("");
  assert!(result.is_err());
  assert_eq!("No such file: ''", &result.unwrap_err());
}

#[test]
pub fn test_probe_remote_file() {
  use serde_json::Value;

  let result = probe("https://github.com/avTranscoder/avTranscoder-data/raw/master/video/BigBuckBunny/BigBuckBunny_480p_stereo.avi");
  assert!(result.is_ok());

  let result: Value = serde_json::from_str(&result.unwrap()).unwrap();

  let expected = std::fs::read_to_string("./tests/result.json").unwrap();
  let expected: Value = serde_json::from_str(&expected).unwrap();
  assert_eq!(expected, result);
}
