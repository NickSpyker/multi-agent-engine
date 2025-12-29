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
// TODO: Write here this section description

#[derive(Default, Clone)]
struct State {
    /* Some data here */
    elapsed: Duration,
    other_data: HashMap<usize, String>,
}

#[derive(Default, Clone)]
struct Config {/* Some config or user inputs here */}

// ----------------------------------------------------------------------------------------------------
// TODO: Write here this section description

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
// TODO: Write here this section description

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
// TODO: Write here this section description

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
// TODO: Write here this section description

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
