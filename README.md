<h1 align="center">Multi-Agent Engine</h1>

<p align="center">A concurrent Rust library for building multi-agent simulators with lock-free CPU/GPU hybrid execution.</p>

## Overview

`multi-agent-engine` is a library designed for building simulators and applications based on multi-agent systems (MAS), agent-based models (ABM), and agent-oriented programming (AOP) paradigms.
It provides a ready-to-use pipeline and tooling for generic use cases, with native support for multithreaded CPU execution and GPU acceleration.

## Core Architecture

The engine follows a dual-thread architecture that separates simulation logic from user interaction, enabling independent operation at different frequencies while maintaining efficient data synchronization.

```txt
┌──────────────────────────────────────────────────────────────┐
│                      Multi-Agent Engine                      │
├──────────────────────────────────────────────────────────────┤
│ ┌───────────────────┐                   ┌──────────────────┐ │
│ │ Controller Thread │                   │ Simulator Thread │ │
│ │  (e.g., 60 Hz)    │                   │  (e.g., 30 Hz)   │ │
│ │                   │                   │                  │ │
│ │  - User Input     │                   │  - Agent Logic   │ │
│ │  - Rendering      │                   │  - Physics       │ │
│ │  - UI Logic       │                   │  - Simulation    │ │
│ └─────────┬─────────┘                   └────────┬─────────┘ │
│           │      ┌────────────────────────┐      │           │
│           ├─────→│     Config (ArcSwap)   ├→─────┤           │
│           │      │ Controller → Simulator │      │           │
│           │      └────────────────────────┘      │           │
│           │      ┌────────────────────────┐      │           │
│           └─────←┤     State (ArcSwap)    │←─────┘           │
│                  │ Controller ← Simulator │                  │
│                  └────────────────────────┘                  │
│                ┌────────────────────────────┐                │
│                │ Message Queue (Controller) │                │
│                │  Controller → Simulator    │                │
│                └────────────────────────────┘                │
│                ┌────────────────────────────┐                │
│                │ Message Queue (Simulator)  │                │
│                │  Controller ← Simulator    │                │
│                └────────────────────────────┘                │
└──────────────────────────────────────────────────────────────┘
```

## User Interface

### Required Trait Implementations

You must implement two core traits to connect with the engine:

#### 1. Controller Trait

Handles user interaction, input processing, and visualization.

```rust
trait Controller {
    fn initialize(&mut self, initial_state: &State);
    fn update(&mut self, state: &State) -> Option<Config>;
    fn handle_messages(&mut self, messages: Vec<SimulatorMessage>);
}
```

#### 2. Simulator Trait

Contains the core simulation logic, agent behaviors, and physics computation.

```rust
trait Simulator {
    fn initialize(&mut self, config: &Config);
    fn tick(&mut self, config: &Config) -> State;
    fn handle_messages(&mut self, messages: Vec<ControllerMessage>);
}
```

### Required Data Structures

#### Config

Configuration and input parameters (Controller → Simulator).

```rust
struct Config {
    // User-defined fields
    // e.g., simulation_speed, agent_count, environment_params
}
```

#### State

Current simulation state (Simulator → Controller).

```rust
struct State {
    // User-defined fields
    // e.g., agent_positions, environment_state, statistics
}
```

## Concurrency Model

### Thread Independence

The Controller and Simulator operate in **separate threads** that **never block each other**.
This design enables:

- **Asynchronous execution**: Each thread runs at its own frequency
- **Responsive UI**: The controller remains responsive even during heavy simulation
- **Scalable performance**: Simulation can utilize multiple cores independently

```txt
Timeline Example:
─────────────────────────────────────────────────────────────────>

Controller (60 Hz): |─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─
                        |       |       |       |       |       |
Simulator  (30 Hz): |───●───────●───────●───────●───────●───────●─
                        |       |       |       |       |       |
                       tick    tick    tick    tick    tick    tick

● = frame/tick execution
```

### Data Sharing with ArcSwap

Both threads share `Config` and `State` through **lock-free atomic swap** (ArcSwap):

```txt
┌────────────────────────────────────────────────┐
│                Data Flow Diagram               │
└────────────────────────────────────────────────┘

 Controller Thread               Simulator Thread
 ─────────────────               ────────────────
       WRITE                          WRITE
         ↓                              ↓
    ┌────────┐                     ┌────────┐
    │ Config │ ←──── ArcSwap ────→ │ State  │
    └────────┘                     └────────┘
       READ                           READ
         ↓                              ↓
    ┌────────┐                     ┌────────┐
    │ State  │ ←──── ArcSwap ────→ │ Config │
    └────────┘                     └────────┘
```

**Access Patterns:**

| Thread     | Config     | State      |
|:-----------|:-----------|:-----------|
| Simulator  | Read-only  | Write-only |
| Controller | Write-only | Read-only  |

This design ensures:

- ✅ **No locks or mutexes** needed for state sharing
- ✅ **Zero blocking** between threads
- ✅ **Data race freedom** through clear ownership

## Message Passing

In addition to shared data, threads communicate through **typed message queues** using distinct enum types:

```txt
┌─────────────────────────────────────────────────────────────┐
│                    Message Communication                    │
└─────────────────────────────────────────────────────────────┘

 Controller                                  Simulator
 ──────────                                  ─────────
     ├→ [Controller Message Queue] ────────→  handle_messages()
     │   - Pause                                 ↓
     │   - Resume                             Process msg
     │   - Reset                                 ↓
     │   - SpawnAgent                         Execute
     │   - CustomCmd                             │
     │                                           │
  handle_messages() ← [Simulator Message Queue] ←┤
     ↓                 - SimComplete             │
  Process msg          - AgentDied               │
     ↓                 - Statistics              │
  Execute              - CustomEvent             │
```

### Example Message Definitions:

```rust
// Controller → Simulator
enum ControllerMessage {
    Pause,
    Resume,
    Reset,
    SpawnAgent { position: Vec3, agent_type: AgentType },
    CustomCommand(String),
}

// Simulator → Controller
enum SimulatorMessage {
    SimulationComplete,
    AgentDied { id: AgentId, reason: String },
    Statistics { tick: u64, data: SimStats },
    CustomEvent(String),
}
```

## GPU Acceleration & Multi-Layer Pipeline

### Hybrid CPU/GPU Execution

The simulator supports **flexible compute allocation** between CPU and GPU, enabling efficient execution of agent behaviors and physics on appropriate hardware.

```txt
┌──────────────────────────────────────────────┐
│       Multi-Layer Processing Pipeline        │
└──────────────────────────────────────────────┘

 Input State
      │
      ↓
 ┌────┴────┐
 │ Layer 1 │ ← CPU: Agent Decision Making
 │  (CPU)  │    - Perception processing
 └────┬────┘    - Behavior selection
      ↓ output1
 ┌────┴────┐
 │ Layer 2 │ ← GPU: Spatial Queries
 │  (GPU)  │    - Neighbor detection
 └────┬────┘    - Collision detection
      ↓ output2
 ┌────┴────┐
 │ Layer 3 │ ← GPU: Physics Simulation
 │  (GPU)  │    - Force calculations
 └────┬────┘    - Position updates
      ↓ output3
 ┌────┴────┐
 │ Layer 4 │ ← CPU: Environment Update
 │  (CPU)  │    - Resource distribution
 └────┬────┘    - Event handling
      ↓ output4
      │
 Final State
```

### Agent-Level GPU Execution

Each agent's behavior can be **individually executed on GPU**, enabling:

- Massive parallelization for homogeneous agent populations
- Efficient SIMD operations for agent computations
- Scalability to thousands or millions of agents

```
Agents: A1, A2, A3, A4, ..., An

 CPU Execution          GPU Execution
 ─────────────          ─────────────

 A1 → compute           ┌──────────────────┐
    ↓                   │ A1 A2 A3 A4 ...  │
 A2 → compute     VS    │ ║  ║  ║  ║       │
    ↓                   │ Parallel Compute │
 A3 → compute           │ ↓  ↓  ↓  ↓       │
    ↓                   └──────────────────┘
   ...
 (Sequential)              (Parallel)
```

## Getting Started

```rust
// 1. Define your data structures
struct MyConfig {
    /* ... */
}
struct MyState {
    /* ... */
}

// 2. Define your message enums
enum ControllerMessage {
    /* ... */
}
enum SimulatorMessage {
    /* ... */
}

// 3. Implement the required traits
struct MyController {
    /* ... */
}
impl Controller for MyController { /* ... */ }

struct MySimulator {
    /* ... */
}
impl Simulator for MySimulator { /* ... */ }

fn main() {
    // 4. Run the engine
    let engine = MultiAgentEngine::new(
        MyController::new(),
        MySimulator::new(),
        config,
    );

    engine.run();
}
```

## License

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
