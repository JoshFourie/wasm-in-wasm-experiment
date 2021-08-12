use reqwest::Response;

use bytes::Bytes;

use js_sys::{Object, Promise, WebAssembly};

use web_sys::{Request, RequestInit, RequestMode};

use wasm_bindgen_futures::JsFuture;


// pub async fn fetch_wasm(url: &str) -> (Bytes, WebAssembly::Module )
pub async fn fetch_wasm(url: &str) -> Bytes
{

    let bytes: Bytes = 
    {
        let response: Response = reqwest::get(url).await.expect("failed to await GET request for url.");
        
        response.bytes().await.expect("failed to get bytes from response.")

    };

    bytes
}

pub async fn fetch_wasm2(url: &str) -> Promise
{
    let response: Promise =
    {
        let request: Request = 
        {
            let init: RequestInit = 
            {
                let mut init: RequestInit = RequestInit::new();  // temporary borrow will be dropped unless persisted.
        
                init.method("GET").mode(RequestMode::Cors);
        
                init
            };
        
            Request::new_with_str_and_init(url, &init).expect("failed to build the request.")
        };

        web_sys::window()
            .map(|window| 
            {
                window.fetch_with_request(&request)
            })
            .expect("failed to get window.")    
    };

    response
}
