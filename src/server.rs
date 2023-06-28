use tonic::{transport::Server, Request, Response, Status};

use payments::cad_server::{Cad, CadServer};
use payments::{PaymentRequest, PaymentResponse};
pub mod payments {
    tonic::include_proto!("payments");
}
#[derive(Debug, Default)]
pub struct PaymentService {}

#[tonic::async_trait]
impl Cad for PaymentService {
    async fn send_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Request Received: {:?}", request);

        let req = request.into_inner();

        let reply = PaymentResponse {
            success: true,
            message: format!("Send {} CAD to {}.", req.amount, req.to_addr).into(),
        };
        Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let cad_service = PaymentService::default();
    Server::builder()
        .add_service(CadServer::new(cad_service))
        .serve(addr)
        .await?;
    Ok(())
}
