use anyhow::Result;
use serde_json;
use spin_sdk::{
    http::{Request,Response},
    http_component,
};

/// Send an HTTP request and return the response.
#[http_component]
fn send_outbound(_req: Request) -> Result<Response> {

    let rpc_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": "sjriddle",
        "method": "query",
        "params": {
            "request_type": "view_account",
            "finality": "final",
            "account_id": "peechz.near"
        }
    }).to_string();

    let mut res = spin_sdk::outbound_http::send_request(
        http::Request::builder()
            .method("POST")
            .uri("https://rpc.mainnet.near.org")
            .header("Content-Type", "application/json")
            .header("Accept", "*/*")
            .body(Some(rpc_body.into()))?
    )?;

    res.headers_mut()
        .insert("spin-component", "cadre-near-account".try_into()?);
    println!("{:?}", res);
    Ok(res)
}
