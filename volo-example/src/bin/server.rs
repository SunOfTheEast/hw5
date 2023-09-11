#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;

use volo_grpc::server::{Server, ServiceBuilder};

use volo_example::LogLayer;
//use volo_example::S;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    Server::new()
        .add_service(ServiceBuilder::new(volo_gen::volo::example::ItemServiceServer::new(S)).build())
        .run(addr)
        .await
        .unwrap();
}

pub struct S;

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
    // 这部分是我们需要增加的代码
    async fn get_item(
        &self,
        _req: volo_grpc::Request<volo_gen::volo::example::GetItemRequest>,
    ) -> core::result::Result<volo_grpc::Response<volo_gen::volo::example::GetItemResponse>, volo_grpc::Status>
    {
        Ok(volo_grpc::Response::new(Default::default()))
    }
}
