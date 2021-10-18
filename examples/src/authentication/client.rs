mod echo;

use echo::{echo_client::EchoClient, EchoRequest};
use tonic::{metadata::MetadataValue, transport::Channel, Request};

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let channel = Channel::from_static("http://[::1]:50051")
            .connect()
            .await
            .unwrap();

        let token = MetadataValue::from_str("Bearer some-secret-token").unwrap();

        let mut client = EchoClient::with_interceptor(channel, move |mut req: Request<()>| {
            println!(">>> {:?}", req.metadata());
            req.metadata_mut().insert("authorization", token.clone());
            println!(">>> {:?}", req.metadata());
            Ok(req)
        });

        let request = tonic::Request::new(EchoRequest {
            message: "hello".into(),
        });

        let response = client.unary_echo(request).await.unwrap();

        println!("RESPONSE={:?}", response);
    })
}
