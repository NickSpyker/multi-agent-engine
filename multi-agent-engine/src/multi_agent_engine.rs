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

use crate::{Controller, Simulator};
use multi_agent_engine_core::{Error, Result};
use std::thread;

pub struct MultiAgentEngine<C, S>
where
    C: Controller,
    S: Simulator,
{
    controller: C,
    simulator: S,
}

impl<C, S> MultiAgentEngine<C, S>
where
    C: Controller + Send + 'static,
    S: Simulator + Send + 'static,
{
    pub fn new(controller: C, simulator: S) -> Self {
        Self {
            controller,
            simulator,
        }
    }

    pub fn run(self) -> Result<()> {
        let Self {
            controller,
            simulator,
        } = self;

        let controller_handle = thread::spawn(move || controller.run());
        let simulator_handle = thread::spawn(move || simulator.run());

        controller_handle.join().map_err(Error::Thread)??;
        simulator_handle.join().map_err(Error::Thread)??;

        Ok(())
    }
}
