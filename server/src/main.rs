use std::sync::Arc;
use std::sync::Mutex;
use std::vec;

use raft::{
    raft_server::{Raft, RaftServer},
    AppendEntriesRequest, AppendEntriesResponse, VoteRequest, VoteResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod raft {
    tonic::include_proto!("raft");
}

// #[derive(Debug, Default)]
// enum Role {
//     #[default]
//     Follower,
//     Candidate,
//     Leader,
// }

#[derive(Debug, Default)]
pub struct RaftNode {
    // role: Role,
    // persistent state
    current_term: u64,
    voted_for: Option<String>,
    log: Log,
    // volatile state
    // commit_index: u64,
    // last_applied: u64,
    // volatile leader state?
}

type Log = Arc<Mutex<Vec<LogEntry>>>;

#[derive(Debug)]
struct LogEntry {
    command: Command,
    term: u64,
}

#[derive(Debug)]
enum Command {
    Get(String),
    Put(String, i64),
}

#[tonic::async_trait]
impl Raft for RaftNode {
    async fn request_vote(
        &self,
        request: Request<VoteRequest>,
    ) -> Result<Response<VoteResponse>, Status> {
        let r = request.into_inner();
        let can_vote = match &self.voted_for {
            None => true,
            Some(id) => *id == r.candidate_id,
        };
        Ok(Response::new(raft::VoteResponse {
            term: self.current_term,
            vote_granted: self.current_term <= r.term && can_vote,
        }))
    }

    async fn append_entries(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesResponse>, Status> {
        let r = request.into_inner();
        let is_current = r.term >= self.current_term;
        let i = r.prev_log_index as usize;
        let mut log = self.log.lock().unwrap();
        let has_entry = i < log.len() && log[i].term == r.prev_log_term;
        if is_current {
            if has_entry {
              // let a = r.entries[0];
              // log.extend(r.entries);
            } else {
                log.truncate(i);
            }
        }
        Ok(Response::new(raft::AppendEntriesResponse {
            term: self.current_term,
            success: is_current && has_entry,
        }))
    }
}

// fn unmarshal_entries(entries: Vec<raft::LogEntry>) -> Vec<LogEntry> {
//   let v = Vec::new();
//   for entry in entries.iter() {
//     // entry.
//   }
//   return v
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let node = RaftNode::default();

    Server::builder()
        .add_service(RaftServer::new(node))
        .serve(address)
        .await?;
    Ok(())
}
