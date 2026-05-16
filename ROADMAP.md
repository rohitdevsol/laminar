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
- [ ] Setup graceful shutdown handling

---

## TCP Listener

- [x] Create listening socket
- [x] Bind address/port
- [ ] Configure socket options
  - [ ] SO_REUSEADDR
  - [ ] TCP_NODELAY
  - [ ] Keepalive
- [x] Implement accept loop
- [x] Handle concurrent client connections
- [x] Handle client disconnects
- [ ] Implement connection cleanup

---

## TCP Proxying

- [x] Connect to backend server
- [x] Forward client → backend traffic
- [x] Forward backend → client traffic
- [x] Support bidirectional forwarding
- [x] Handle disconnect propagation
- [ ] Add connection timeout handling
- [ ] Add idle timeout handling

---

## Backend Pool

- [x] Implement backend registry
- [x] Track backend runtime state
- [ ] Add backend availability tracking
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
- [ ] Add backend recovery detection

---

## Connection Tracking

- [x] Track active connections per backend
- [x] Increment on connect
- [x] Decrement on disconnect
- [ ] Track total requests

---

## Least Connections Balancing

- [x] Implement Least Connections algorithm
- [x] Ignore unhealthy backends
- [x] Add algorithm abstraction layer

---

## Retry Logic

- [ ] Retry failed backend connections
- [ ] Retry next available backend
- [ ] Add retry limits
- [ ] Add retry logging

---

## Timeout Management

- [ ] Backend connect timeout
- [ ] Idle connection timeout
- [ ] Read timeout
- [ ] Write timeout

---

## Runtime State Refactor

- [ ] Reduce lock scope sizes
- [ ] Refactor duplicated runtime logic
- [ ] Separate balancing module
- [ ] Separate health module
- [*] Improve state ownership model

---

## Phase 2 Deliverable

- [ ] Health-aware balancing
- [ ] Retry support
- [ ] Connection metrics
- [ ] Runtime stability improvements

---

# Phase 3 — Observability & Runtime Control

## Weighted Balancing

- [ ] Weighted Round Robin
- [ ] Backend weights in config
- [ ] Dynamic weight updates

---

## Metrics

- [ ] Active connection metrics
- [ ] Request counters
- [ ] Failure counters
- [ ] Backend health metrics
- [ ] Throughput metrics

---

## Prometheus Integration

- [ ] Prometheus metrics exporter
- [ ] Metrics endpoint
- [ ] Backend-specific metrics

---

## Logging Improvements

- [ ] Structured JSON logs
- [ ] Request correlation IDs
- [ ] Retry logging
- [ ] Timeout logging
- [ ] Backend transition logging

---

## Graceful Backend Draining

- [ ] Add draining backend state
- [ ] Stop routing new connections
- [ ] Wait for active connections
- [ ] Graceful backend removal

---

## Dynamic Config Reloading

- [ ] Watch configuration file
- [ ] Reload backend configuration
- [ ] Preserve active connections
- [ ] Runtime backend updates

---

## Admin API

- [ ] Add runtime status endpoint
- [ ] Add backend health endpoint
- [ ] Add backend enable/disable API
- [ ] Add metrics endpoint

---

## Phase 3 Deliverable

- [ ] Runtime configurability
- [ ] Operational observability
- [ ] Graceful backend management

---

# Phase 4 — High Performance Runtime

## Event-Driven Runtime

- [ ] Integrate epoll
- [ ] Add edge-triggered events
- [ ] Implement event batching
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
