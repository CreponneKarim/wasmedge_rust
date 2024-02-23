use std::{fs::OpenOptions, future, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use gag::Redirect;
use tokio::join;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions}, params, wasi::r#async::{AsyncState,  WasiContext}, VmBuilder
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    const LISTENER_PATH: &str = "./wasm_app1/target/wasm32-wasi/release/wasm_app1.wasm";
    const WRITER_PATH: &str = "./wasm_app2/target/wasm32-wasi/release/wasm_app2.wasm";

    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build().unwrap();
    let wasi_ctx = WasiContext::new(None, None, Some(vec![(".", ".")]));
    let mut vm = VmBuilder::new()
        .with_config(config.clone())
        .with_wasi_context(wasi_ctx)
        .build().unwrap();

    vm = vm.register_module_from_file("listener", LISTENER_PATH).unwrap();
    vm = vm.register_module_from_file("writer", LISTENER_PATH).unwrap();

    let vm = Arc::new(Mutex::new(vm));
    let vm_cloned = Arc::clone(&vm);

    let listener_thread = thread::spawn(  || async move  {
        let vm_child_thread = vm_cloned.lock().expect("fail to lock vm");
        
        let async_state = AsyncState::new();

        let log = OpenOptions::new()
            .truncate(true)
            .read(true)
            .create(true)
            .write(true)
            .open("tmp/listener.tmp")
            .unwrap();

        let print_redirect = Redirect::stdout(log).unwrap();

        let _ = vm_child_thread.run_func( Some("listener"), "_start", []);

        // let mut log = print_redirect.into_inner();
    });

    let vm_cloned2 = Arc::clone(&vm);

    let writer_thread = thread::spawn(  || async move{
        let vm_child_thread = vm_cloned2.lock().expect("fail to lock vm");
        
        let async_state = AsyncState::new();

        let log = OpenOptions::new()
            .truncate(true)
            .read(true)
            .create(true)
            .write(true)
            .open("tmp/listener.tmp")
            .unwrap();

        let print_redirect = Redirect::stdout(log).unwrap();

        let _ = vm_child_thread.run_func( Some("writer"), "_start", []);

        // let mut log = print_redirect.into_inner();
    });
    // writer_thread.join();

    // let futures = vec![listener_thread.join(), writer_thread.join()];

    // futures::future::join_all(
    //     futures
    // ).await;
    join!(listener_thread.join().unwrap(), writer_thread.join().unwrap());

    Ok(())
}