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

use arc_swap::{ArcSwap, Guard};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Shared<T> {
    data: Arc<ArcSwap<T>>,
}

impl<T> Shared<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(ArcSwap::from_pointee(data)),
        }
    }

    pub fn load(&self) -> Guard<Arc<T>> {
        self.data.load()
    }

    pub fn store(&self, data: T) {
        self.data.store(Arc::new(data));
    }
}
