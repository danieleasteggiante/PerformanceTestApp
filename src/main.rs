use std::env;
use std::sync::Arc;
use tokio::runtime::Builder;
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
    let n_threads = std::thread::available_parallelism().unwrap().get();
    let runtime_tokio = Builder::new_multi_thread()
        .thread_name("flow-executor")
        .worker_threads(n_threads)
        .enable_all()
        .build()
        .unwrap();
    runtime_tokio.block_on(execute_flow(config_file));
}

async fn execute_flow(config_file_relative_path: String) {
    let executor = Arc::new(flow_executor::FlowExecutor::new(config_file_relative_path.clone()));
    executor.execute().await;
}


