mod utils;  // exports log and set_panic macros
mod fetch;

use wasmer::{Store, Module, Instance, Function, WasmerEnv, Value, ImportObject, PartiallyTypedImport, imports};

use wasm_bindgen::prelude::*;

use wasm_bindgen_futures::spawn_local;

use bytes::Bytes;

use js_sys::Promise;

use std::sync::{Arc, Mutex, MutexGuard};


#[wasm_bindgen]
pub fn main()
{
    set_panic!();

    spawn_local(async move { app().await });
}

async fn app()
{
    // get bytes
    
    let url: &str = "http://127.0.0.1:7878/resources/alpha.wasm";

    let store: Store = Store::new();

    // let module: Module = module(url, &store).await;

    let (instance, env) = instance(url).await;

    log!("{:?}", instance);

    let app: _ = instance
        .exports
        .get_function("app")
        .expect("failed to get function 'app' from exports");

    let result: Box<[Value]> = app
        .call(&[ ])
        .expect("failed to call function 'app'.");

    log!("{:?}", result);

    let counter: i32 = get_counter_i32(&env);

    log!("{}", counter);
}

async fn module(url: &str, store: &Store) -> Module
{
    let bytes: Bytes = fetch::fetch_wasm(url).await;

    Module::from_binary(store, &bytes).expect("failed to make a module.")
}

async fn instance(url: &str) -> (Instance, Env<i32>)
{
    let response: Promise= fetch::fetch_wasm2(url).await;

    let store: Store = Store::new();

    let env: Env<i32> = Env::new(0);

    let import_object: ImportObject = imports!
    {
        "wbg" => {
            "__wbg_getcounter_2d994047dff704ba" => Function::new_native_with_env(
                &store, 
                env.clone(),
                get_counter_i32
            ),
            "__wbg_addtocounter_d7ed1b01de5cf7c7" => Function::new_native_with_env(
                &store,
                env.clone(),
                add_to_counter_i32
            )
        }
    };

    let import_types: Vec<PartiallyTypedImport> = vec![
        PartiallyTypedImport { module: "wbg".into(), name: "__wbg_getcounter_2d994047dff704ba".into() },
        PartiallyTypedImport { module: "wbg".into(), name: "__wbg_addtocounter_d7ed1b01de5cf7c7".into() }
    ];

    let (instance, module): (Instance, Module) = Instance::from_promise(
        store,
        &response, 
        &import_object,
        import_types
    )
    .await
    .expect("failed to instantiate wasm module from promise");

    (instance, env)
}

// Wasmer Env. for Import Example

#[derive(WasmerEnv, Clone)]
struct Env<T> 
{
    counter: Arc<Mutex<T>>
}

impl<T> Env<T>
where
    T: Copy
{
    fn new(init: T) -> Self 
    {
        let counter: Arc<Mutex<T>> = 
        {
            let mtx: Mutex<T> = Mutex::new(init);
            
            Arc::new(mtx)
        };

        Self {
            counter
        }
    }

    fn get_counter(&self) -> MutexGuard<T>
    { 
        self
            .counter
            .lock()
            .expect("failed to get a lock on the counter.")
    }
    
    fn add_to_counter(&self, value: T) -> MutexGuard<T>
    where
        T: std::ops::AddAssign<T>
    {
        let mut counter: MutexGuard<T> = self.get_counter();

        *counter += value;

        counter
    }
}

fn get_counter_i32(env: &Env<i32>) -> i32 { *env.get_counter() }

fn add_to_counter_i32(env: &Env<i32>, value: i32) -> i32 { *env.add_to_counter(value) }
