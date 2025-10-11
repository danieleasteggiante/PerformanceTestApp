use std::env;
use crate::use_cases::driver_list_case::DriverListCase;
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
    let use_case = "driver_list".to_string();
    let config_file = "/src/resource/driver_list_case.json".to_string();
    if use_case.is_empty() || config_file.is_empty() {
        panic!("use case or config file missing!");
    }
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    tokio_runtime.block_on(execute_flow(use_case, config_file));
}

async fn execute_flow(use_case: String, config_file_relative_path: String) {
    let result = match use_case.as_str() {
        "driver_list" => DriverListCase::new(config_file_relative_path),
        _ => panic!("Use case {} not found", use_case),
    };
    result.execute().await;
}


