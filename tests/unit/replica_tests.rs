use viewstamped_replication::replica::{Replica, ReplicaError};

#[test]
fn test_replica_initialization() {
    let config = vec![
        "127.0.0.1:7000".to_string(),
        "127.0.0.1:7001".to_string(),
        "127.0.0.1:7002".to_string()
    ];

    // create replica 0 (should be primary in view 0)
    let replica = Replica::new(config.clone(), 0);
    assert_eq!(replica.view_number, 0);
    assert_eq!(replica.status, viewstamped_replication::replica::Status::Normal);
    assert_eq!(replica.is_primary(), true);
    assert_eq!(replica.get_primary_address(), "127.0.0.1:7000");

    // create replica 1 (should be backup in view 0)
    let replica = Replica::new(config, 1);
    assert_eq!(replica.is_primary(), false);
}

#[test]
fn test_process_request_as_primary() {
    let config = vec![
        "127.0.0.1:7000".to_string(),
        "127.0.0.1:7001".to_string(),
        "127.0.0.1:7002".to_string()
    ];

    // create primary replica
    let mut replica = Replica::new(config, 0);

    // process a client request
    let result = replica.process_request(
        "client-1".to_string(),
        1,
        b"SET key1 value1".to_vec()
    );

    assert!(result.is_ok());
    assert_eq!(replica.op_number, 1);
    assert_eq!(replica.log.len(), 1);

    // process same request again should return cached result
    let second_result = replica.process_request(
        "client-1".to_string(),
        1,
        b"SET key1 value1".to_vec()
    );

    assert!(second_result.is_ok());
    assert_eq!(second_result.unwrap(), result.unwrap());
    assert_eq!(replica.op_number, 1); // shouldn't have increased
}

#[test]
fn test_process_request_as_backup() {
    let config = vec![
        "127.0.0.1:7000".to_string(),
        "127.0.0.1:7001".to_string(),
        "127.0.0.1:7002".to_string()
    ];

    // create backup replica
    let mut replica = Replica::new(config, 1);

    // process a client request should fail
    let result = replica.process_request(
        "client-1".to_string(),
        1,
        b"SET key1 value1".to_vec()
    );

    assert!(result.is_err());
    match result {
        Err(ReplicaError::NotPrimary(_)) => (),
        _ => panic!("Expected NotPrimary error"),
    }
}
