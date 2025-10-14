use std::env;
use crate::use_cases::flow_executor;
mod domain;
mod use_cases;

fn get_args(name: String) -> String {
    let args: Vec<String> = env::args().collect();
    args.iter()
        .position(|a| a == &name)
        .and_then(|p| args.get(p + 1))
        .cloned()
        .unwrap_or_else(|| {
            println!("Flow {} not found", name);
            String::new()
        })
}

fn main() {
    //let use_case = get_args("-flow".to_string());
    //let config_file = get_args("-config".to_string());
    let config_file = "/src/resource/driver_list_case.json".to_string();
    if config_file.is_empty() {
        panic!("Config file missing!");
    }
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    tokio_runtime.block_on(execute_flow(config_file));
}

async fn execute_flow(config_file_relative_path: String) {
    let result = flow_executor::FlowExecutor::new(config_file_relative_path);
    result.execute().await;
}


