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

use multi_agent_engine::{Controller, MultiAgentEngine, Result, Shared, Simulator, message};
use std::cmp::PartialEq;
use std::{thread, time::Duration};

// 1. Define your data structures
#[derive(Clone)]
struct MyConfig {}

#[derive(Clone)]
struct MyState {}

// 2. Define your message enums

#[derive(Debug, Clone, PartialEq)]
enum ControllerMessage {
    Hello,
    HalfDone,
    Bye,
}

#[derive(Debug, Clone, PartialEq)]
enum SimulatorMessage {
    Hello,
    HalfDone,
    Bye,
}

// 3. Implement the required traits

struct MyController {
    _config: Shared<MyConfig>,
    _state: Shared<MyState>,
    sender: message::Sender<ControllerMessage>,
    receiver: message::Receiver<SimulatorMessage>,
}

impl Controller for MyController {
    fn run(self) -> Result<()> {
        println!("  Controller Start");

        self.sender.send(ControllerMessage::Hello);

        for i in 0..20 {
            let messages: Vec<SimulatorMessage> = self.receiver.receive();
            for msg in messages {
                println!("  Simulator → Controller: {msg:?}");
            }

            if i == 20 {
                self.sender.send(ControllerMessage::HalfDone);
            }

            println!("□ frame");
            thread::sleep(Duration::from_millis(1000 / 60));
        }

        self.sender.send(ControllerMessage::Bye);

        println!("  Controller Stop");
        Ok(())
    }
}

struct MySimulator {
    _config: Shared<MyConfig>,
    _state: Shared<MyState>,
    sender: message::Sender<SimulatorMessage>,
    receiver: message::Receiver<ControllerMessage>,
}

impl Simulator for MySimulator {
    fn run(self) -> Result<()> {
        println!("  Simulator Start");

        self.sender.send(SimulatorMessage::Hello);

        for i in 0..10 {
            let messages: Vec<ControllerMessage> = self.receiver.receive();
            for msg in messages {
                println!("  Controller → Simulator: {msg:?}");
            }

            if i == 10 {
                self.sender.send(SimulatorMessage::HalfDone);
            }

            println!("● tick");
            thread::sleep(Duration::from_millis(1000 / 30));
        }

        self.sender.send(SimulatorMessage::Bye);

        println!("  Simulator Stop");
        Ok(())
    }
}

fn main() -> Result<()> {
    let (controller_sender, controller_receiver) = message::Queue::channel();
    let (simulator_sender, simulator_receiver) = message::Queue::channel();

    let config = Shared::new(MyConfig {});
    let state = Shared::new(MyState {});

    let controller = MyController {
        _config: config.clone(),
        _state: state.clone(),
        sender: controller_sender,
        receiver: simulator_receiver,
    };

    let simulator = MySimulator {
        _config: config.clone(),
        _state: state.clone(),
        sender: simulator_sender,
        receiver: controller_receiver,
    };

    let engine = MultiAgentEngine::new(controller, simulator);

    engine.run()
}
