mod echo;

use echo::{echo_server, EchoRequest, EchoResponse};
use futures::Stream;
use std::pin::Pin;
use tonic::{metadata::MetadataValue, transport::Server, Request, Response, Status, Streaming};

type EchoResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send + Sync>>;

#[derive(Default)]
pub struct EchoServerImpl;

#[tonic::async_trait]
impl echo_server::Echo for EchoServerImpl {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let message = request.into_inner().message;
        Ok(Response::new(EchoResponse { message }))
    }
}

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let addr = "[::1]:50051".parse().unwrap();

        let server = EchoServerImpl::default();

        // let svc = echo_server::EchoServer::new(server);

        let svc = echo_server::EchoServer::with_interceptor(server, check_auth);

        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap();
    })
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token = MetadataValue::from_str("Bearer some-secret-token").unwrap();
    let meta = req.metadata();
    println!("{:?}", meta);
    match meta.get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
