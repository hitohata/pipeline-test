use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = resp(&message);
    Ok(resp)
}

fn resp(message: &String) -> Response<Body> {
    Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::Text(message.to_owned()))
        .map_err(Box::new)
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resp_test() {
        let some_string = "some_string".to_string();
        let result = resp(&some_string);
        assert_eq!(result.body(), &Body::Text(some_string));
    }
}