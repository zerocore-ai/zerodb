## RAFT

RAFT is a consensus algorithm that is designed to be easy to understand. It's equivalent to Paxos in fault-tolerance and performance. The difference is that it's decomposed into relatively independent subproblems, and it cleanly addresses all major pieces needed for practical systems.

---

## Single Partition Transaction

- Modified RAFT with 3 types of leaders
  - Cluster Leader
  - Partition Leader
  - Delegate Leader

---

## Multi Partition Transaction

- Undetermined
