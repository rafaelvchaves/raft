use tonic::{transport::Server, Request, Response, Status};
use raft::{};

pub mod raft {
  tonic::include_proto!("raft");
}

#[derive(Debug, Default)]
pub struct RaftService {}

#[tonic::async_trait]
impl Raft for RaftService {
  // async fn vote(&self, request: Request<VotingRequest>) -> Result<Response<VotingResponse>, Status> {
  //   let r = request.into_inner();
  //   match r.vote {
  //     0 => Ok(Response::new(voting::VotingResponse { confirmation: { 
  //       format!("Happy to confirm that you upvoted for {}", r.url)
  //     }})),
  //     1 => Ok(Response::new(voting::VotingResponse { confirmation: { 
  //       format!("Confirmation that you downvoted for {}", r.url)
  //     }})), 
  //     _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid vote provided"))
  //   }
  // }
}

fn main() {
    println!("Hello, world!");
}
