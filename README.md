# Viewstamped Replication in Rust

This project implements Viewstamped Replication (VR) based on Liskov and Cowling's paper. It provides a reliable, crash-fault tolerant replication system that maintains consistency when nodes fail, without requiring disk writes during normal operation.

VSR enables state machine replication across a cluster, ensuring operations execute in the same order at all replicas despite concurrent requests and failures. The implementation includes:

- Normal case processing for client requests
- View changes when a primary fails
- Node recovery for rejoining the cluster
- Reconfiguration to change cluster membership

## Project Status

Currently implementing the core replica logic and testing framework. More components will be added as development progresses.

## References

- Liskov, B., & Cowling, J. (2012). [Viewstamped Replication Revisited](http://pmg.csail.mit.edu/papers/vr-revisited.pdf). MIT CSAIL.
- Castro, M., & Liskov, B. (1999). [Practical Byzantine Fault Tolerance](http://pmg.csail.mit.edu/papers/osdi99.pdf). OSDI.
- Oki, B. M., & Liskov, B. (1988). Viewstamped Replication: A New Primary Copy Method to Support Highly-Available Distributed Systems. PODC.

## License

MIT License - see LICENSE for details.
