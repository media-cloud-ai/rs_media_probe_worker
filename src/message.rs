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

  let result = probe(&source_path).map_err(|error| {
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
  let result = probe("https://github.com/avTranscoder/avTranscoder-data/raw/master/video/BigBuckBunny/BigBuckBunny_480p_stereo.avi");
  assert!(result.is_ok());

  let expected = "{\
  \"format\":{\
    \"format_name\":\"avi\",\
    \"format_long_name\":\"AVI (Audio Video Interleaved)\",\
    \"program_count\":0,\
    \"start_time\":0.0,\
    \"duration\":30.0,\
    \"bit_rate\":2325208,\
    \"packet_size\":0,\
    \"nb_streams\":2,\
    \"metadata\":{\
      \"encoder\":\"Lavf54.29.104\"\
    },\
    \"streams\":[\
      {\
        \"index\":0,\
        \"stream_type\":\"video\",\
        \"codec_name\":\"msmpeg4v2\",\
        \"codec_long_name\":\"MPEG-4 part 2 Microsoft variant version 2\",\
        \"codec_tag\":null,\
        \"start_time\":0.0,\
        \"duration\":30.0,\
        \"bit_rate\":2064556,\
        \"stream_metadata\":{},\
        \"width\":854,\
        \"height\":480,\
        \"display_aspect_ratio\":{\
          \"num\":0,\
          \"den\":1\
        },\
        \"frame_rate\":{\
          \"num\":24,\
          \"den\":1\
        },\
        \"level\":null,\
        \"profile\":null,\
        \"scanning_type\":null,\
        \"chroma_subsampling\":\"4:2:0\",\
        \"timecode\":null,\
        \"pix_fmt\":\"yuv420p\"\
      },\
      {\
        \"index\":1,\
        \"stream_type\":\"audio\",\
        \"codec_name\":\"mp3\",\
        \"codec_long_name\":\"MP3 (MPEG audio layer 3)\",\
        \"codec_tag\":null,\
        \"start_time\":0.0,\
        \"duration\":30.0,\
        \"bit_rate\":256000,\
        \"stream_metadata\":{},\
        \"channels\":2,\
        \"sample_rate\":48000,\
        \"sample_fmt\":\"fltp\",\
        \"bits_per_sample\":0\
      }\
    ]\
  }\
}";
  assert_eq!(expected, result.unwrap());
}
