/*
 * Copyright 2025 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use multi_agent_engine::message::{channel, direction, MessageReceiver, MessageSender};
use multi_agent_engine::{Controller, MultiAgentEngine, Result, Shared, System};
use std::{
    cmp::PartialEq,
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

// ----------------------------------------------------------------------------------------------------
// SHARED DATA STRUCTURES
//
// State: Read by Controllers, written by System
//   - Represents the current state of the simulation/system
//   - Updated each system tick and shared with all controllers via ArcSwap
//   - Controllers read this to render/display/log the current state
//
// Config: Written by Controllers, read by System
//   - Contains user inputs, settings, or configuration
//   - Controllers update this based on user interaction
//   - System reads this to adjust behavior based on user input

#[derive(Default, Clone)]
struct State {
    /* Some data here */
    elapsed: Duration,
    other_data: HashMap<usize, String>,
}

#[derive(Default, Clone)]
struct Config {/* Some config or user inputs here */}

// ----------------------------------------------------------------------------------------------------
// MESSAGE TYPES
//
// Messages provide bidirectional communication between System and Controllers
// Each controller has its own typed message queues (independent from other controllers)
//
// SimMessage: Sent from System → Controller
//   - System can send events, responses, or notifications to controllers
//
// GuiMessage: Sent from Controller → System
//   - Controllers can send commands, requests, or events to the system

#[derive(Debug, Eq, PartialEq)]
enum SimMessage {
    HelloController,
}

#[derive(Debug, Eq, PartialEq)]
enum GuiMessage {
    HelloSystem,
    Middle,
    ByeSystem,
}

// ----------------------------------------------------------------------------------------------------
// SYSTEM IMPLEMENTATION
//
// The System runs the core simulation/processing logic on its own thread
// It has:
//   - state: Shared state it writes to (read by controllers)
//   - config: Shared config it reads from (written by controllers)
//   - sender: Sends messages to controller(s)
//   - receiver: Receives messages from controller(s)
//
// The System::run() method contains the main loop that:
//   1. Reads the latest config from controllers
//   2. Processes incoming messages from controllers
//   3. Performs the core system tick (simulation step, physics, agent behaviors, etc.)
//   4. Updates the shared state for controllers to read
//   5. Optionally sends messages back to controllers

struct Simulator {
    state: Shared<State>,
    config: Shared<Config>,
    sender: MessageSender<SimMessage, direction::ToController>,
    receiver: MessageReceiver<GuiMessage, direction::FromController>,
}

impl System for Simulator {
    fn run(self) -> Result<()> {
        const FREQUENCY_HZ: u64 = 30;

        self.sender.send_lossy(SimMessage::HelloController);

        let frequency = Duration::from_millis(1000 / FREQUENCY_HZ);
        let mut is_running = true;
        while is_running {
            let now = Instant::now();

            let _read_config = self.config.load();

            for msg in self.receiver.iter() {
                println!("⇋ Controller → System    : {msg:?}");
                if msg == GuiMessage::ByeSystem {
                    is_running = false;
                }
            }

            println!("□ tick");

            let elapsed = now.elapsed();

            let current_state = self.state.load();
            self.state.store(State {
                elapsed,
                other_data: current_state.other_data.clone(),
            });

            if elapsed < frequency {
                thread::sleep(frequency - elapsed);
            }
        }

        Ok(())
    }
}

// ----------------------------------------------------------------------------------------------------
// CONTROLLER IMPLEMENTATION
//
// Controllers handle user interaction, rendering, logging, or any I/O on independent threads
// Each controller has:
//   - state: Shared state it reads from (written by system)
//   - config: Shared config it writes to (read by system)
//   - sender: Sends messages to the system
//   - receiver: Receives messages from the system
//
// The Controller::run() method contains the controller loop that:
//   1. Reads the latest state from the system
//   2. Processes incoming messages from the system
//   3. Handles user input, rendering, logging, or other I/O
//   4. Updates the shared config if user changed settings
//   5. Optionally sends messages to the system (commands, events, etc.)
//
// Controllers run at their own frequency independent of the system and other controllers

struct Gui {
    state: Shared<State>,
    config: Shared<Config>,
    sender: MessageSender<GuiMessage, direction::ToSystem>,
    receiver: MessageReceiver<SimMessage, direction::FromSystem>,
}

impl Controller for Gui {
    fn run(self) -> Result<()> {
        const FREQUENCY_HZ: u64 = 60;
        const MAX_FRAMES: usize = 20;

        self.sender.send_lossy(GuiMessage::HelloSystem);

        let frequency = Duration::from_millis(1000 / FREQUENCY_HZ);
        for i in 0..=MAX_FRAMES {
            let now = Instant::now();

            let read_state = self.state.load();

            for msg in self.receiver.iter() {
                println!("⇌ System ────→ Controller: {msg:?}");
            }

            println!(
                "● frame {i:02}/{MAX_FRAMES} - system {:?}",
                read_state.elapsed
            );

            if i == MAX_FRAMES / 2 {
                self.sender.send_lossy(GuiMessage::Middle);
            }

            let elapsed = now.elapsed();
            if elapsed < frequency {
                thread::sleep(frequency - elapsed);
            }
        }

        self.sender.send_lossy(GuiMessage::ByeSystem);

        Ok(())
    }
}

// ----------------------------------------------------------------------------------------------------
// ENGINE SETUP AND INITIALIZATION
//
// This section shows how to wire up the multi-agent engine:
//
// 1. Create shared data structures (State and Config)
//    - These are wrapped in Shared<T> (ArcSwap) for lock-free sharing between threads
//
// 2. Create message channels for bidirectional communication
//    - gui_channel: Controller → System messages (GuiMessage)
//    - sys_channel: System → Controller messages (SimMessage)
//    - Each channel is split into a sender and receiver
//
// 3. Construct the System and Controller(s)
//    - Pass cloned references to shared state/config
//    - Wire up message senders/receivers (note the crossover: gui's receiver gets sys messages)
//
// 4. Build and run the engine
//    - Create engine with the system
//    - Add controller(s) via with_controller()
//    - Call run() to start all threads and begin execution

fn main() -> Result<()> {
    let state = Shared::new(State::default());
    let config = Shared::new(Config::default());

    let gui_channel = channel::FromControllerToSystem::<GuiMessage>::bounded(10);
    let (gui_sender, gui_receiver) = gui_channel.split();

    let sys_channel = channel::FromSystemToController::<SimMessage>::bounded(10);
    let (sys_sender, sys_receiver) = sys_channel.split();

    let gui = Gui {
        state: state.clone(),
        config: config.clone(),
        sender: gui_sender,
        receiver: sys_receiver,
    };

    let sim = Simulator {
        state,
        config,
        sender: sys_sender,
        receiver: gui_receiver,
    };

    let engine = MultiAgentEngine::new_with_system(sim).with_controller(gui);

    engine.run()
}
