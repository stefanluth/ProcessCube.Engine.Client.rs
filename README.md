# ProcessCube.Engine.Client.rs

A 5Minds ProcessCube® Engine client written in Rust.

This client was designed to work with the 5Minds ProcessCube® Engine API used in the Docker Image
`5minds/processcube_engine:16.0.2` and API changes after this version are not supported.

## Current coverage

### Clients

| Endpoint            | Support |
| ------------------- | :-----: |
| AnonymousSessions   |   ❌    |
| ApplicationInfo     |   ✅    |
| Correlations        |   ❌    |
| Cronjobs            |   ❌    |
| DataObjectInstances |   ❌    |
| EmptyActivities     |   ❌    |
| Events              |   ✅    |
| ExternalTasks       |   ❌    |
| FlowNodeInstances   |   ✅    |
| ManualTasks         |   ❌    |
| Notifications       |   ❌    |
| ProcessDefinitions  |   ✅    |
| ProcessInstances    |   ❌    |
| ProcessModels       |   ✅    |
| UserMetadata        |   ❌    |
| UserTasks           |   ❌    |

✅ Full Support -
❌ No Support -
GET/POST/... Partial Support

### CLI

| Endpoint            | Support |
| ------------------- | :-----: |
| AnonymousSessions   |   ❌    |
| ApplicationInfo     |   ✅    |
| Correlations        |   ❌    |
| Cronjobs            |   ❌    |
| DataObjectInstances |   ❌    |
| EmptyActivities     |   ❌    |
| Events              |   ❌    |
| ExternalTasks       |   ❌    |
| FlowNodeInstances   |   ❌    |
| ManualTasks         |   ❌    |
| Notifications       |   ❌    |
| ProcessDefinitions  |   ✅    |
| ProcessInstances    |   ❌    |
| ProcessModels       |   ❌    |
| UserMetadata        |   ❌    |
| UserTasks           |   ❌    |

✅ Full Support -
❌ No Support -
GET/POST/... Partial Support

## How to Use

### Executable

If you just want to use the `processcube_engine_client` CLI, you can use

```shell
cargo install processcube_engine_client
```

to install it. You should then be able to use it, e.g. `processcube_engine_client application-info authority`

### Library

When developing an application, you can use the library by adding this package to your project.

```shell
cargo add processcube_engine_client
```

A short usage example:

```rust
use processcube_engine_client::clients::{client_factory::ClientFactory, error::EngineError};

// Be sure to have a running 5Minds Engine at the given URL
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
