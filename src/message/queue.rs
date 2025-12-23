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

use super::{Receiver, Sender};
use crossbeam_channel::unbounded;
use std::fmt::Debug;

pub struct Queue<T>
where
    T: Debug + Clone,
{
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Default for Queue<T>
where
    T: Debug + Clone,
{
    #[inline]
    fn default() -> Self {
        let (sender, receiver) = unbounded();

        Self {
            sender: Sender::<T>::new(sender),
            receiver: Receiver::<T>::new(receiver),
        }
    }
}

impl<T> Queue<T>
where
    T: Debug + Clone,
{
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn sender(&self) -> Sender<T> {
        self.sender.clone()
    }

    #[inline]
    pub fn receiver(&self) -> Receiver<T> {
        self.receiver.clone()
    }
}
