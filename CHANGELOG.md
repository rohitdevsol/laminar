# Changelog

---

## 2026-05-12

### Added

- Added YAML configuration parsing support
- Added backend server health checking
- Added active backend discovery utility
- Added integration ping test for backend connectivity

### Notes

This phase established the initial networking foundation of Laminar.
The project could now:

- load backend definitions from configuration
- verify backend availability using TCP connectivity checks
- maintain a list of reachable upstream servers

The implementation at this stage was intentionally minimal and focused on validating the core async networking flow.

---

## 2026-05-14

### Added

- Introduced modular project architecture
- Added dedicated modules for:
  - `algorithms`
  - `config`
  - `proxy`
  - `state`
  - `health`
  - `common`
- Added typed configuration models using `serde`
- Added runtime state abstraction for backend tracking
- Added application-wide shared state model
- Added configuration validation layer
- Added structured logging using `tracing`
- Added dynamic config loading through CLI arguments

### Changed

- Refactored backend representation to separate:
  - static configuration
  - runtime state
- Replaced mixed server structs with dedicated runtime state models
- Moved YAML loading logic into isolated config module
- Reorganized crate structure for scalability and maintainability
- Replaced temporary logging/debug prints with structured tracing

### Notes

This refactor marked the transition from a prototype implementation to a production-oriented architecture.

The primary design goal was separation of concerns:

- configuration models now represent immutable desired state
- runtime state models track live health and connection metrics

This restructuring prepares Laminar for:

- multiple balancing algorithms
- runtime health management
- metrics collection
- connection pooling
- retry systems
- hot configuration reloads
- future admin APIs

The internal architecture now more closely resembles real-world proxy and load balancer systems.

---

## 2026-05-15

### Added

- Added basic TCP reverse proxy functionality
- Added bidirectional TCP traffic forwarding using Tokio
- Added naive round robin backend selection
- Added runtime backend health tracking using `AtomicBool`
- Added periodic background backend health monitoring
- Added automatic unhealthy backend skipping during selection
- Added configurable health check intervals through YAML config
- Added graceful handling when no healthy backends are available
- Added RAII-based active connection tracking using `ConnectionGuard`
- Added thread-safe connection metrics using `AtomicUsize`
- Added integration tests for:
  - round robin balancing
  - unhealthy backend filtering
  - backend health probing
  - dead backend handling

### Changed

- Refactored backend selection flow to reduce shared state lock contention
- Replaced mutable round robin counters with `AtomicUsize`
- Reduced runtime lock scope during backend selection
- Improved health logging to only emit meaningful state transitions
- Refactored proxy flow to separate:
  - backend selection
  - backend connection
  - traffic forwarding
- Refactored state ownership to use `Arc<BackendState>` for shared runtime access
- Moved connection cleanup logic to `Drop` implementation for guaranteed decrementing

### Notes

This phase transformed Laminar from a static proxy prototype into a dynamically adaptive load balancer runtime.

Laminar can now:

- forward real TCP traffic between clients and backend servers
- distribute requests across backend pools
- detect backend failures at runtime
- automatically avoid routing traffic to unhealthy servers
- recover backend availability without requiring restarts

The implementation intentionally prioritizes simplicity and observability over advanced optimizations.

Current health monitoring behavior remains intentionally naive:

- direct TCP connectivity probing
- immediate healthy/unhealthy transitions
- sequential backend checking

This creates a clean foundation for future improvements such as:

- retry policies
- failure thresholds
- recovery delays
- latency-aware health scoring
- parallelized health probes
- advanced balancing strategies

## 2026-05-17

### Added

- Added configurable backend connection timeouts
- Added configurable idle connection timeouts (connection reaper)
- Added timeout support to TCP proxy logic using `tokio::time::timeout`
- Added `connect_timeout_secs` and `idle_timeout_secs` to YAML configuration
- Added duration-based timeout tracking in `AppState`

### Changed

- Refactored `src/proxy/tcp.rs` to wrap async operations with timeouts
- Improved proxy error handling to distinguish between connection failures and timeouts
- Updated `AppState` to pre-calculate `Duration` objects for timeouts

### Notes

Laminar is now significantly more resilient against "cascading failures" caused by slow or unresponsive backends.
The implementation of connection and idle timeouts ensures that:

- a "black hole" backend (dropping packets) won't hang the proxy task indefinitely
- inactive or "dead" connections are automatically reaped to prevent resource exhaustion
- failed connection attempts due to timeouts trigger automatic failover to the next healthy backend

---

## 2026-05-16

### Added

- Added pluggable balancing algorithm structure
- Added least-connections load balancing
- Added configurable upstream balancing selection through YAML
- Added dedicated algorithm modules for:
  - round robin
  - least connections
- Added tests for least-connections backend selection

### Changed

- Refactored backend selection logic out of `UpstreamPool`
- Reorganized balancing logic into isolated algorithm modules

### Notes

Laminar can now route traffic using runtime-aware balancing strategies instead of fixed request rotation.

The current implementation intentionally keeps algorithm dispatch simple using enum matching and naive selection logic.
More advanced balancing abstractions and optimizations will evolve later as additional strategies are introduced.

---

## 2026-05-20

### Added

- Added retry stabilization to avoid retrying already-failed backends during a single request lifecycle
- Added runtime behavior tests for:
  - connection guard lifecycle tracking
  - timeout-based unhealthy backend handling
  - retry stabilization behavior
- Added backend recovery logging for healthy state transitions
- Added Excalidraw runtime architecture diagram to README
- Added visual documentation for:
  - runtime structs
  - backend ownership model
  - shared state relationships
  - YAML configuration flow

### Changed

- Refactored proxy connection handling into isolated helper flow
- Improved timeout and proxy error logging with clearer runtime context
- Reduced health checker lock scope to avoid holding shared state locks across async network operations
- Improved retry logging with backend-aware runtime details
- Cleaned up retry flow readability and connection orchestration structure

### Notes

This phase focused heavily on runtime stability, observability, and internal architecture clarity.

Laminar now has stronger runtime behavior guarantees around:

- retry isolation
- backend failover handling
- timeout-aware connection management
- async-safe shared state access
- connection lifecycle tracking

The runtime architecture documentation was also expanded to better explain how:

- `AppState`
- `UpstreamPool`
- `BackendState`
- `ConnectionGuard`

interact during live traffic routing and health monitoring.

---

## 2026-06-05

### Added

- Added Prometheus metrics exporter
- Added `/prometheus` runtime metrics endpoint
- Added backend-specific Prometheus metrics
- Added structured JSON runtime logging
- Added request correlation IDs using UUIDs
- Added graceful TCP proxy shutdown handling
- Added backend draining support
- Added draining-aware backend selection
- Added runtime request and failure metrics
- Added Prometheus active connection gauges
- Added runtime config reload support
- Added graceful backend removal semantics
- Added weighted round robin balancing
- Added backend weight configuration support
- Added runtime reload integration tests
- Added draining lifecycle integration tests

### Changed

- Refactored weighted round robin scheduling using precomputed weighted backend pools
- Refactored proxy connection lifecycle logging
- Improved structured tracing across retries and request flow
- Improved backend runtime observability
- Improved connection accounting using Prometheus gauges
- Improved backend lifecycle management semantics
- Improved runtime reload reconciliation behavior
- Improved weighted scheduling runtime efficiency
- Cleaned up proxy retry orchestration flow

### Notes

This phase focused heavily on operational observability, runtime lifecycle safety, and runtime traffic management behavior.

Laminar now supports:

- Prometheus-compatible metrics
- structured request tracing
- graceful shutdown handling
- graceful backend draining
- runtime config reloads
- weighted traffic distribution
- runtime-safe backend lifecycle transitions
- backend-aware operational telemetry

The runtime now behaves much more like a production-oriented traffic management system with live observability, graceful lifecycle handling, and runtime traffic control semantics.

---

## 2026-06-06

### Added

- Added Prometheus request duration histograms
- Added backend connection latency histograms
- Added throughput metrics for inbound and outbound traffic
- Added runtime status API endpoint
- Added backend enable/disable admin APIs
- Added backend health API endpoint
- Added dynamic backend weight update API
- Added automatic config watcher reload support

### Changed

- Standardized Prometheus metric labels using backend IDs
- Improved runtime observability with latency-aware metrics
- Improved weighted round robin runtime efficiency
- Improved operational runtime control APIs
- Improved backend runtime management semantics

### Notes

This phase focused on completing Laminar’s runtime control and observability layer.

Laminar now supports:

- runtime status introspection APIs
- backend operational control APIs
- dynamic backend weight mutation
- automatic config reload watching
- request latency observability
- backend connection latency tracking
- throughput visibility

Phase 3 now provides a much more complete operational runtime environment with live observability, runtime mutation support, and production-style backend lifecycle controls.
