use engine_client::clients::engine_client::EngineClient;
use engine_client::clients::process_model_client::ProcessModelClient;
mod structs;

const ENGINE_URL: &str = "http://localhost:10560";
const AUTH_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let engine_client = EngineClient::new(ENGINE_URL, AUTH_TOKEN);
    let process_model_client = ProcessModelClient::new(engine_client);
    let response = process_model_client.get_process_models().await?;
    println!("{:#?}", response);
    Ok(())
}
