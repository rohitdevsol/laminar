# Layer 4 Load Balancer Roadmap

# Phase 1 — Core TCP Load Balancer

## Project Foundation

- [x] Initialize repository structure
- [x] Setup formatting/linting
- [x] Setup CI pipeline
- [x] Setup logging framework
- [x] Define configuration format
- [x] Create runtime AppState
- [x] Define backend configuration model
- [x] Setup graceful shutdown handling

---

## TCP Listener

- [x] Create listening socket
- [x] Bind address/port
- [x] Implement accept loop
- [x] Handle concurrent client connections
- [x] Handle client disconnects
- [x] Implement connection cleanup

---

## TCP Proxying

- [x] Connect to backend server
- [x] Forward client → backend traffic
- [x] Forward backend → client traffic
- [x] Support bidirectional forwarding
- [x] Handle disconnect propagation
- [x] Add connection timeout handling
- [x] Add idle timeout handling

---

## Backend Pool

- [x] Implement backend registry
- [x] Track backend runtime state
- [x] Add backend availability tracking
- [x] Implement backend selection interface

---

## Round Robin Balancing

- [x] Implement Round Robin
- [x] Skip unavailable backends
- [x] Handle empty backend pool safely

---

## Phase 1 Deliverable

- [x] Functional TCP load balancer
- [x] Concurrent connection support
- [x] Multiple backend support
- [x] Basic balancing
- [x] Structured logs

---

# Phase 2 — Reliability & Runtime State

## Backend Health Checks

- [x] Implement active TCP health probes
- [x] Add healthy/unhealthy backend state
- [x] Skip unhealthy backends
- [x] Add health transition logging

---

## Periodic Health Monitoring

- [x] Create background health task
- [x] Add configurable health intervals
- [x] Add backend recovery detection

---

## Connection Tracking

- [x] Track active connections per backend
- [x] Increment on connect
- [x] Decrement on disconnect

---

## Least Connections Balancing

- [x] Implement Least Connections algorithm
- [x] Ignore unhealthy backends
- [x] Add algorithm abstraction layer

---

## Retry Logic

- [x] Retry failed backend connections
- [x] Retry next available backend
- [x] Add retry limits
- [x] Add retry logging

---

## Timeout Management

- [x] Backend connect timeout
- [x] Idle connection timeout
- [ ] Read timeout
- [ ] Write timeout

---

## Runtime State Refactor

- [x] Reduce lock scope sizes
- [x] Separate balancing module
- [x] Separate health module
- [x] Improve state ownership model

---

## Phase 2 Deliverable

- [x] Health-aware balancing
- [x] Retry support
- [x] Connection metrics
- [x] Runtime stability improvements

---

# Phase 3 — Observability & Runtime Control

## Weighted Balancing

- [x] Weighted Round Robin
- [x] Backend weights in config
- [ ] Dynamic weight updates

---

## Metrics

- [x] Active connection metrics
- [x] Request counters
- [x] Failure counters
- [x] Backend health metrics
- [x] Throughput metrics
- [x] Track total requests

---

## Prometheus Integration

- [x] Prometheus metrics exporter
- [x] Metrics endpoint
- [x] Backend-specific metrics

---

## Logging Improvements

- [x] Structured JSON logs
- [x] Request correlation IDs
- [x] Retry logging
- [x] Timeout logging
- [x] Backend transition logging

---

## Graceful Backend Draining

- [x] Add draining backend state
- [x] Stop routing new connections
- [x] Wait for active connections
- [x] Graceful backend removal

---

## Dynamic Config Reloading

- [x] Runtime config reload API
- [x] Automatic file watcher reload
- [x] Reload backend configuration
- [x] Preserve active connections
- [x] Runtime backend updates

---

## Admin API

- [x] Add runtime status endpoint
- [ ] Add backend health endpoint
- [x] Add backend enable/disable API
- [x] Add metrics endpoint

---

## Phase 3 Deliverable

- [x] Runtime configurability
- [x] Operational observability
- [x] Graceful backend management

---

# Phase 4 — High Performance Runtime

## Event-Driven Runtime

- [ ] Add worker thread model

---

## Memory & Buffer Optimization

- [ ] Buffer pooling
- [ ] Reduce allocations
- [ ] Optimize stream forwarding
- [ ] Reduce lock contention

---

## Advanced Algorithms

- [ ] IP Hash
- [ ] Consistent Hashing
- [ ] Power of Two Choices
- [ ] Least Response Time

---

## TLS Features

- [ ] TLS passthrough
- [ ] SNI inspection
- [ ] Certificate reload support

---

## UDP Support

- [ ] UDP listener
- [ ] Datagram forwarding
- [ ] Stateless balancing
- [ ] UDP health checks

---

## Advanced Observability

- [ ] Grafana dashboards
- [ ] OpenTelemetry support
- [ ] Distributed tracing

---

## Benchmarking

- [ ] wrk benchmarks
- [ ] iperf benchmarks
- [ ] Concurrent connection stress testing
- [ ] Latency measurements
- [ ] Throughput measurements

---

## Chaos Testing

- [ ] Backend crash simulation
- [ ] Packet loss simulation
- [ ] High latency simulation
- [ ] High churn testing

---

## Deployment

- [ ] Docker image
- [ ] Static binary builds
- [ ] systemd service
- [ ] Kubernetes manifests
- [ ] CI/CD pipeline

---

## Phase 4 Deliverable

- [ ] Production-grade runtime
- [ ] High concurrency support
- [ ] Operational tooling
- [ ] Performance optimization

---

# Future Exploration

- [ ] io_uring
- [ ] eBPF observability
- [ ] XDP packet filtering
- [ ] DPDK experimentation
- [ ] QUIC exploration
- [ ] Plugin system
- [ ] WASM/Lua extensions
- [ ] Distributed load balancing
