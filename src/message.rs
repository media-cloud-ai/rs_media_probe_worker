use log::LevelFilter;
use mcai_worker_sdk::job::{Job, JobResult, JobStatus};
use mcai_worker_sdk::{Channel, MessageError, ParametersContainer};
use stainless_ffmpeg::probe::Probe;

pub const SOURCE_PATH_PARAMETER: &'static str = "source_path";

pub fn process(
  _channel: Option<&Channel>,
  job: &Job,
  job_result: JobResult,
) -> Result<JobResult, MessageError> {
  let source_path = job
    .get_string_parameter(SOURCE_PATH_PARAMETER)
    .ok_or_else(|| {
      MessageError::ProcessingError(
        job_result
          .clone()
          .with_status(JobStatus::Error)
          .with_message(&format!(
            "Invalid job message: missing expected '{}' parameter.",
            SOURCE_PATH_PARAMETER
          )),
      )
    })?;

  let mut probe = Probe::new(&source_path);
  probe.process(LevelFilter::Off).map_err(|error| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&format!("Unable to process probe: {}", error)),
    )
  })?;

  let result = serde_json::to_string(&probe).map_err(|error| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&format!("Unable to serialize probe result: {:?}", error)),
    )
  })?;

  Ok(
    job_result
      .with_status(JobStatus::Completed)
      .with_message(&result),
  )
}
