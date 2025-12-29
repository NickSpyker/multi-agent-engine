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

use multi_agent_engine_core::{Controller, Error, Result, System};
use multi_agent_engine_runtime::BoxedController;
use std::thread::{self, JoinHandle};

pub struct MultiAgentEngine<S: System> {
    system: S,
    controllers: Vec<BoxedController>,
}

impl<S: System + Send + 'static> MultiAgentEngine<S> {
    pub fn new_with_system(system: S) -> Self {
        Self {
            system,
            controllers: Vec::new(),
        }
    }

    pub fn with_controller<C: Controller + Send + 'static>(mut self, controller: C) -> Self {
        self.controllers.push(Box::new(controller));
        self
    }

    pub fn run(self) -> Result<()> {
        let Self {
            system,
            controllers,
        } = self;

        let controller_handles: Vec<JoinHandle<Result<()>>> = controllers
            .into_iter()
            .map(|controller| thread::spawn(move || controller.run()))
            .collect();

        let system_handle = thread::spawn(move || system.run());

        for handle in controller_handles {
            handle.join().map_err(Error::Thread)??;
        }

        system_handle.join().map_err(Error::Thread)?
    }
}
