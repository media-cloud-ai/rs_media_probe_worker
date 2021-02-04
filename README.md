# rs_media_probe_worker

Perform a ffprobe on local or remote videos. It returns the result to RabbitMQ or in a destination_path file.

## Examples

```bash
# Result to a destination_path file
export SOURCE_PATH=<video path>
export DESTINATION_PATH=<result path>
RUST_LOG=debug SOURCE_ORDERS=./examples/with_destinaton_path.json cargo run


# Result to a RabbitMQ
export SOURCE_PATH=<video path>
RUST_LOG=debug SOURCE_ORDERS=./examples/without_destinaton_path.json cargo run
```
