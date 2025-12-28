<h1 align="center">Multi-Agent Engine</h1>

<p align="center">A concurrent Rust library for building multi-agent systems with lock-free CPU/GPU hybrid execution.</p>

## Overview

`multi-agent-engine` is a library designed for building applications based on multi-agent systems (MAS), agent-based models (ABM), and agent-oriented programming (AOP) paradigms.
It provides a ready-to-use pipeline and tooling for generic use cases, with native support for multithreaded CPU execution and GPU acceleration.

While the engine excels at simulations, its architecture is general-purpose and can power any application requiring concurrent processing with responsive user interaction: games, real-time data pipelines, robotics controllers, and more.

## Core Architecture

The engine follows a multi-thread architecture that separates system logic from user interaction, enabling independent operation at different frequencies while maintaining efficient data synchronization.

```txt
┌─────────────────────────────────────────────────────────────────────────┐
│                           Multi-Agent Engine                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌──────────────────┐  ┌──────────────────┐       ┌──────────────────┐  │
│  │ Controller A     │  │ Controller B     │  ...  │ Controller N     │  │
│  │ Thread (60 Hz)   │  │ Thread (30 Hz)   │       │ Thread (10 Hz)   │  │
│  │                  │  │                  │       │                  │  │
│  │ - User Input     │  │ - Logging        │       │ - Network I/O    │  │
│  │ - Rendering      │  │ - Metrics        │       │ - Remote Control │  │
│  │ - UI Logic       │  │ - Recording      │       │ - Sync           │  │
│  └────────┬─────────┘  └────────┬─────────┘       └────────┬─────────┘  │
│           │                     │                          │            │
│           ▼                     ▼                          ▼            │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                        Config Channels (ArcSwap)                  │  │
│  │  ┌───────────┐       ┌───────────┐            ┌───────────┐       │  │
│  │  │ Config A  │       │ Config B  │    ...     │ Config N  │       │  │
│  │  └───────────┘       └───────────┘            └───────────┘       │  │
│  │            Controllers → System (independent configs)             │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                    │                                    │
│                                    ▼                                    │
│                         ┌──────────────────┐                            │
│                         │  System Thread   │                            │
│                         │   (e.g., 30 Hz)  │                            │
│                         │                  │                            │
│                         │  - Agent Logic   │                            │
│                         │  - Physics       │                            │
│                         │  - Processing    │                            │
│                         └────────┬─────────┘                            │
│                                  │                                      │
│                                  ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                         State (ArcSwap)                           │  │
│  │                     System → Controllers                          │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                    Message Queues (per Controller)                │  │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐    │  │
│  │  │ Controller A ↔  │  │ Controller B ↔  │  │ Controller N ↔  │    │  │
│  │  │     System      │  │     System      │  │     System      │    │  │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────┘    │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## Thread Hosting Options

By default, the engine spawns dedicated threads for the System and each Controller.
However, you can configure the engine to run either the System or one Controller on the engine's main thread.
This is useful for:

- **GUI frameworks** that require the main thread for rendering.
- **Reducing thread count** in resource-constrained environments.
- **Integration** with existing event loops.

```txt
┌─────────────────────────────────────────────────────────────────────────┐
│                        Thread Hosting Modes                             │
└─────────────────────────────────────────────────────────────────────────┘

 Mode 1: Default (all separate threads)
 ───────────────────────────────────────
 Engine Thread: idle/management only
 Controller A:  dedicated thread
 Controller B:  dedicated thread
 System:        dedicated thread

 Mode 2: System on engine thread
 ───────────────────────────────────────
 Engine Thread: runs System
 Controller A:  dedicated thread
 Controller B:  dedicated thread

 Mode 3: Controller on engine thread
 ───────────────────────────────────────
 Engine Thread: runs Controller A (e.g., GUI)
 Controller B:  dedicated thread
 System:        dedicated thread
```

## User Interface

### Required Trait Implementations

You must implement two core traits to connect with the engine:

#### 1. Controller Trait

Handles user interaction, input processing, and visualization. Each Controller provides its own typed Config to the System.

```rust
trait Controller {
    type Config;
    type OutgoingMessage;
    type IncomingMessage;

    fn initialize(&mut self, initial_state: &State);
    fn update(&mut self, state: &State) -> Option<Self::Config>;
    fn handle_messages(&mut self, messages: Vec<Self::IncomingMessage>);
}
```

#### 2. System Trait

Contains the core processing logic, agent behaviors, and computation. The System can read from multiple independent Configs.

```rust
trait System {
    fn initialize(&mut self);
    fn tick(&mut self, configs: &Configs) -> State;
    fn handle_messages(&mut self, messages: Vec<ControllerMessage>);
}
```

### Required Data Structures

#### Config (per Controller)

Each Controller defines and provides its own configuration type. The System reads these independently.

```rust
// Controller A's config
struct InputConfig {
    // e.g., mouse_position, keyboard_state, selected_tool
}

// Controller B's config
struct SimulationConfig {
    // e.g., time_scale, agent_count, environment_params
}

// Controller C's config
struct NetworkConfig {
    // e.g., sync_rate, peer_list, connection_state
}
```

#### State

Current system state, shared with all Controllers (System → Controllers).

```rust
struct State {
    // User-defined fields
    // e.g., agent_positions, environment_state, statistics
}
```

## Concurrency Model

### Thread Independence

The System and all Controllers operate in **separate threads** that **never block each other**.
This design enables:

- **Asynchronous execution**: Each thread runs at its own frequency
- **Responsive UI**: Controllers remain responsive even during heavy processing
- **Scalable performance**: System can utilize multiple cores independently
- **Modular design**: Controllers can be added or removed without affecting others

```txt
Timeline Example (2 Controllers + System):
────────────────────────────────────────────────────────────────────────────>

Controller A (60 Hz): |─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─
                          │       │       │       │       │       │
Controller B (20 Hz): |───●───────────────●───────────────●───────────────●─
                          │       │       │       │       │       │
System       (30 Hz): |───●───────●───────●───────●───────●───────●───────●─
                          │       │       │       │       │       │
                         tick    tick    tick    tick    tick    tick

● = frame/tick execution
```

### Data Sharing with ArcSwap

Threads share data through **lock-free atomic swap** (ArcSwap). Each Controller maintains its own Config channel, while all Controllers read from a shared State.

```txt
┌──────────────────────────────────────────────────────────────────────────┐
│                           Data Flow Diagram                              │
└──────────────────────────────────────────────────────────────────────────┘

 Controller A          Controller B          Controller N
 ────────────          ────────────          ────────────
    WRITE                 WRITE                 WRITE
      │                     │                     │
      ▼                     ▼                     ▼
 ┌──────────┐          ┌──────────┐          ┌──────────┐
 │ Config A │          │ Config B │          │ Config N │
 └──────────┘          └──────────┘          └──────────┘
      │                     │                     │
      └──────────────┬──────┴─────────────────────┘
                     │
              ArcSwap (per config)
                     │
                     ▼
              ┌─────────────┐
              │   System    │  ← Reads configs selectively
              └─────────────┘
                     │
                   WRITE
                     │
                     ▼
              ┌─────────────┐
              │    State    │
              └─────────────┘
                     │
              ArcSwap (shared)
                     │
      ┌──────────────┼──────────────────────────┐
      │              │                          │
      ▼              ▼                          ▼
    READ           READ                       READ
 ┌──────────┐  ┌──────────┐              ┌──────────┐
 │ Ctrl A   │  │ Ctrl B   │     ...      │ Ctrl N   │
 └──────────┘  └──────────┘              └──────────┘
```

**Access Patterns:**

| Thread       | Own Config | Other Configs | State      |
|:-------------|:-----------|:--------------|:-----------|
| System       | N/A        | Read-only     | Write-only |
| Controller A | Write-only | None          | Read-only  |
| Controller B | Write-only | None          | Read-only  |
| Controller N | Write-only | None          | Read-only  |

This design ensures:

- ✅ **No locks or mutexes** needed for state sharing
- ✅ **Zero blocking** between threads
- ✅ **Data race freedom** through clear ownership
- ✅ **Independent configs** per Controller with typed safety

## Message Passing

In addition to shared data, threads communicate through **typed message queues**. Each Controller has its own bidirectional message channel with the System.

```txt
┌─────────────────────────────────────────────────────────────────────────┐
│                         Message Communication                           │
└─────────────────────────────────────────────────────────────────────────┘

 Controller A                              System
 ────────────                              ──────
     ├──→ [Outgoing Queue A] ────────────→  handle_messages()
     │     - UserAction                          │
     │     - ToolSelected                        ▼
     │     - SpawnRequest                   Process & Execute
     │                                           │
     │                                           │
     ←── [Incoming Queue A] ←────────────────────┤
           - SelectionResult                     │
           - ActionFeedback                      │
                                                 │
 Controller B                                    │
 ────────────                                    │
     ├──→ [Outgoing Queue B] ────────────────────┤
     │     - StartRecording                      │
     │     - SetMarker                           │
     │                                           │
     ←── [Incoming Queue B] ←────────────────────┤
           - RecordingStats                      │
           - FrameCaptured                       │
                                                 │
 Controller N                                    │
 ────────────                                    │
     ├──→ [Outgoing Queue N] ────────────────────┤
     │     - SyncRequest                         │
     │     - PeerUpdate                          │
     │                                           │
     ←── [Incoming Queue N] ←────────────────────┘
           - SyncComplete
           - PeerStatus
```

### Example Message Definitions:

```rust
// Controller A (UI) → System
enum UIControllerMessage {
    Pause,
    Resume,
    Reset,
    SpawnAgent { position: Vec3, agent_type: AgentType },
    SelectEntity(EntityId),
}

// System → Controller A (UI)
enum UISystemMessage {
    SelectionResult { entity: Option<EntityInfo> },
    AgentSpawned { id: AgentId },
    ActionFeedback { success: bool, message: String },
}

// Controller B (Metrics) → System
enum MetricsControllerMessage {
    RequestSnapshot,
    SetSamplingRate(f32),
}

// System → Controller B (Metrics)
enum MetricsSystemMessage {
    Snapshot { tick: u64, data: SystemStats },
    SamplingRateConfirmed(f32),
}
```

## GPU Acceleration & Multi-Layer Pipeline

### Hybrid CPU/GPU Execution

The System supports **flexible compute allocation** between CPU and GPU, enabling efficient execution of agent behaviors and physics on appropriate hardware.

```txt
┌──────────────────────────────────────────────┐
│       Multi-Layer Processing Pipeline        │
└──────────────────────────────────────────────┘

 Input State
      │
      ▼
 ┌────┴────┐
 │ Layer 1 │ ← CPU: Agent Decision Making
 │  (CPU)  │    - Perception processing
 └────┬────┘    - Behavior selection
      ▼ output1
 ┌────┴────┐
 │ Layer 2 │ ← GPU: Spatial Queries
 │  (GPU)  │    - Neighbor detection
 └────┬────┘    - Collision detection
      ▼ output2
 ┌────┴────┐
 │ Layer 3 │ ← GPU: Physics Simulation
 │  (GPU)  │    - Force calculations
 └────┬────┘    - Position updates
      ▼ output3
 ┌────┴────┐
 │ Layer 4 │ ← CPU: Environment Update
 │  (CPU)  │    - Resource distribution
 └────┬────┘    - Event handling
      ▼ output4
      │
 Final State
```

### Agent-Level GPU Execution

Each agent's behavior can be **individually executed on GPU**, enabling:

- Massive parallelization for homogeneous agent populations
- Efficient SIMD operations for agent computations
- Scalability to thousands or millions of agents

```txt
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

### Basic Setup (Single Controller)

```rust
// 1. Define your data structures
struct MyConfig {
    /* ... */
}
struct MyState {
    /* ... */
}

// 2. Define your message types
enum MyControllerMessage { /* ... */ }
enum MySystemMessage { /* ... */ }

// 3. Implement the required traits
struct MyController {
    /* ... */
}
impl Controller for MyController {
    type Config = MyConfig;
    type OutgoingMessage = MyControllerMessage;
    type IncomingMessage = MySystemMessage;
    // ...
}

struct MySystem {
    /* ... */
}
impl System for MySystem { /* ... */ }

fn main() {
    // 4. Build and run the engine
    let engine = MultiAgentEngine::builder()
        .with_controller(MyController::new())
        .with_system(MySystem::new())
        .build();

    engine.run();
}
```

### Multi-Controller Setup

```rust
// Define multiple controllers with different responsibilities
struct RenderController {
    /* ... */
}
struct MetricsController {
    /* ... */
}
struct NetworkController {
    /* ... */
}

// Each with its own Config type
struct RenderConfig {
    /* ... */
}
struct MetricsConfig {
    /* ... */
}
struct NetworkConfig {
    /* ... */
}

fn main() {
    let engine = MultiAgentEngine::builder()
        .with_controller(RenderController::new())
        .with_controller(MetricsController::new())
        .with_controller(NetworkController::new())
        .with_system(MySystem::new())
        .build();

    engine.run();
}
```

### Running on Engine Thread

```rust
fn main() {
    // Run the render controller on the main thread (required by some GUI frameworks)
    let engine = MultiAgentEngine::builder()
        .with_controller_on_engine_thread(RenderController::new())
        .with_controller(MetricsController::new())
        .with_system(MySystem::new())
        .build();

    engine.run(); // RenderController runs here, others in spawned threads
}
```

```rust
fn main() {
    // Or run the System on the main thread
    let engine = MultiAgentEngine::builder()
        .with_controller(RenderController::new())
        .with_controller(MetricsController::new())
        .with_system_on_engine_thread(MySystem::new())
        .build();

    engine.run(); // System runs here, Controllers in spawned threads
}
```

## License

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
