use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Normal,
    ViewChange,
    Recovering,
    Transitioning,
}

#[derive(Debug, Error)]
pub enum ReplicaError {
    #[error("Invalid view number: expected {expected}, got {actual}")]
    InvalidViewNumber { expected: u64, actual: u64 },

    #[error("Not primary for view {0}")]
    NotPrimary(u64),

    #[error("Operation number mismatch: expected {expected}, got {actual}")]
    OpNumberMismatch { expected: u64, actual: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub client_id: String,
    pub request_number: u64,
    pub operation: Vec<u8>,
    pub result: Option<Vec<u8>>,
}

pub struct Replica {
    // Configuration is a sorted array of IP addresses
    pub configuration: Vec<String>,
    // Index of this replica in the configuration
    pub replica_number: usize,
    // Current view number
    pub view_number: u64,
    // Current status
    pub status: Status,
    // Op-number assigned to most recently received request
    pub op_number: u64,
    // Log containing operations in their assigned order
    pub log: Vec<LogEntry>,
    // Op-number of most recently committed operation
    pub commit_number: u64,
    // Records client request info and results
    pub client_table: HashMap<String, (u64, Option<Vec<u8>>)>,
}

impl Replica {
    pub fn new(configuration: Vec<String>, replica_number: usize) -> Self {
        Replica {
            configuration,
            replica_number,
            view_number: 0,
            status: Status::Normal,
            op_number: 0,
            log: Vec::new(),
            commit_number: 0,
            client_table: HashMap::new(),
        }
    }

    pub fn is_primary(&self) -> bool {
        let primary_idx = (self.view_number as usize) % self.configuration.len();
        primary_idx == self.replica_number
    }

    pub fn get_primary_address(&self) -> &str {
        let primary_idx = (self.view_number as usize) % self.configuration.len();
        &self.configuration[primary_idx]
    }

    pub fn process_request(&mut self, client_id: String, request_number: u64, operation: Vec<u8>)
        -> Result<Vec<u8>, ReplicaError>
    {
        if !self.is_primary() {
            return Err(ReplicaError::NotPrimary(self.view_number));
        }

        // check if we've already processed this request
        if let Some((last_req, result)) = self.client_table.get(&client_id) {
            if *last_req == request_number && result.is_some() {
                // we've already executed this request, return cached result
                return Ok(result.as_ref().unwrap().clone());
            }

            if *last_req >= request_number {
                // stale request, ignore
                return Err(ReplicaError::OpNumberMismatch {
                    expected: *last_req + 1,
                    actual: request_number
                });
            }
        }

        // process new request (simplified for now)
        // in a real implementation, this would:
        // 1. advance op_number
        // 2. add request to log
        // 3. send PREPARE to backups
        // 4. wait for f PREPAREOK responses
        // 5. execute operation and get result
        // 6. update commit_number

        self.op_number += 1;

        // simplified: just store in log and "execute" immediately
        let result = b"EXECUTED".to_vec();
        self.log.push(LogEntry {
            client_id: client_id.clone(),
            request_number,
            operation,
            result: Some(result.clone()),
        });

        self.client_table.insert(client_id, (request_number, Some(result.clone())));

        Ok(result)
    }
}
