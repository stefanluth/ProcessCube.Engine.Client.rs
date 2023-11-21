# ProcessCube.Engine.Client.rs

A library and CLI tool written in Rust, to send requests to the 5Minds ProcessCube® Engine API.

This project was designed to work with the ProcessCube® Engine API provided in the Docker Image
[`5minds/processcube_engine:16.0.0`](https://hub.docker.com/r/5minds/processcube_engine/tags).

Older or newer versions _might_ work, but support is not guaranteed.

## Current coverage

Coverage of the available endpoints is currently rather limited.

| Endpoint            | Library | CLI |
| ------------------- | :-----: | :-: |
| AnonymousSessions   |   ❌    | ❌  |
| ApplicationInfo     |   ✅    | ✅  |
| Correlations        |   ✅    | ✅  |
| Cronjobs            |   ❌    | ❌  |
| DataObjectInstances |   ❌    | ❌  |
| EmptyActivities     |   ❌    | ❌  |
| Events              |   ✅    | ❌  |
| ExternalTasks       |   ❌    | ❌  |
| FlowNodeInstances   |   ✅    | ❌  |
| ManualTasks         |   ❌    | ❌  |
| Notifications       |   ❌    | ❌  |
| ProcessDefinitions  |   ✅    | ✅  |
| ProcessInstances    |   ❌    | ❌  |
| ProcessModels       |   ✅    | ❌  |
| UserMetadata        |   ❌    | ❌  |
| UserTasks           |   ❌    | ❌  |

✅ Full Support -
❌ No Support -
GET/POST/... Partial Support

## How to Use

### CLI

If you just want to use the `processcube_engine_client` CLI tool, you can use

```shell
cargo install processcube_engine_client
```

to install it. You should then be able to use it, e.g. `processcube_engine_client application-info authority`

### Library

When developing an application, you can use the library by adding this package to your project, i.e.

```shell
cargo add processcube_engine_client
```

A short usage example:

```rust
use processcube_engine_client::clients::{client_factory::ClientFactory, error::EngineError};

// Be sure to have a running ProcessCube® Engine at the given URL
const ENGINE_URL: &str = "http://localhost:10560";
const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";

#[tokio::main]
async fn main() -> Result<(), EngineError> {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let client = client_factory.create_application_info_client();
    let info = client.get_application_info().await?;
    println!("{:#?}", info);
    Ok(())
}
```

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

### Setup

1. Clone this repository
2. Run `docker run -p 10560:80 5minds/processcube_engine:16.0.0` to start the 5Minds ProcessCube® Engine
3. Ready to go!

You can now run `cargo test` to run the tests, or `cargo run --bin processcube_engine_client` to run the CLI tool.
Swagger documentation for the API can be found at [`http://localhost:10560/`](http://localhost:10560/).
