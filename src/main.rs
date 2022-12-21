///function wrapper

//github crate
use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_runtime::{Context, error::HandlerError};

fn main() { //parameter main function
    lambda!(hello)
}

fn hello(
    request: Request, ///grabbing data from the source
    _ctx: Context ///defining context type
) -> Result<impl IntoResponse, HandlerError> { ///genrate a response and check for errors from handlers
    Ok(format!( ///converting value into a string, Ok is an enmu that represents success and containing a value,
        "hello {}",
        request
            .query_string_parameters() ///	Parses a given query string back into a vector of key-value pairs. 
            .get("name")
            .unwrap_or_else(|| "stranger")
    ))
}
