use tonic::{Request, Response, Status};

use crate::greeter::{HelloReply, HelloRequest, greeter_server::Greeter};

pub mod hello_world {
    include!("./greeter.rs");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn hello_rpc(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}
