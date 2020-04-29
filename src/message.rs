use mcai_worker_sdk::job::{Job, JobResult, JobStatus};
use mcai_worker_sdk::{Channel, MessageError};

pub const SOURCE_PATH_PARAMETER: &'static str = "source_path";

pub fn process(
  _channel: Option<&Channel>,
  _job: &Job,
  job_result: JobResult,
) -> Result<JobResult, MessageError> {
  Ok(job_result.with_status(JobStatus::Completed))
}
