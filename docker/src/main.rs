use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};
use std::process;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_handler(event: Value, _: Context) -> Result<Value, Error> {
    let first_name = event["firstName"].as_str().unwrap_or("world");
    let notion_key = "NOTION_API";

    let notion_api_key = match env::var(notion_key) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, notion_key);
            process::exit(1);
        }
    };
    Ok(json!({
        "message": format!("Hello, {}!, {}", first_name, notion_api_key)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    //async関数は#[test]では使用できない
    //#[test]
    #[tokio::test]
    async fn my_handler_response() -> Result<(), Error> {
        let event = json!({
            "firstName": "AWS Lambda on Rust"
        });
        let json = json!({
            "message": "Hello, AWS Lambda on Rust!",
        });
        let response = my_handler(event, Context::default()).await?;
        assert_eq!(response, json);
        Ok(())
    }
}
