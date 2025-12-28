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
use multi_agent_engine::{Controller, MultiAgentEngine, Result, System};

enum GuiMessage {}

enum SimMessage {}

struct Gui {
    sender: MessageSender<GuiMessage, direction::ToSystem>,
    receiver: MessageReceiver<SimMessage, direction::FromSystem>,
}

impl Controller for Gui {
    type Config = ();
    type State = ();
    type OutgoingMessage = ();
    type IncomingMessage = ();

    fn run(self) -> Result<()> {
        Ok(())
    }
}

struct Simulator {
    sender: MessageSender<SimMessage, direction::ToController>,
    receiver: MessageReceiver<GuiMessage, direction::FromController>,
}

impl System for Simulator {
    type Config = ();
    type State = ();
    type OutgoingMessage = ();
    type IncomingMessage = ();

    fn run(self) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    let gui_channel = channel::FromControllerToSystem::<GuiMessage>::bounded(10);
    let (gui_sender, gui_receiver) = gui_channel.split();

    let sys_channel = channel::FromSystemToController::<SimMessage>::bounded(10);
    let (sys_sender, sys_receiver) = sys_channel.split();

    let gui = Gui {
        sender: gui_sender,
        receiver: sys_receiver,
    };

    let sim = Simulator {
        sender: sys_sender,
        receiver: gui_receiver,
    };

    let engine = MultiAgentEngine::new_with_system(sim).with_controller(gui);

    engine.run()
}
