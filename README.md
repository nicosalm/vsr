This project is based on "Viewstamped Replication Revisited" by Liskov and Cowling and provides a
reliable, crash-fault tolerant replication system that maintains consistency in the presence of
node failures without requiring disk writes during normal operation.

VSR enables state machine replication across a configurable cluster of nodes, ensuring that
operations execute in the same order at all replicas despite concurrent client requests and node
failures. The implementation includes the core protocol components: normal case processing, view
changes for leader election, node recovery, and reconfiguration.
