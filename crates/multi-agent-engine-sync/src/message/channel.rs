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

use super::{
    direction::{sealed, FromController, FromSystem, ToController, ToSystem}, MessageReceiver,
    MessageSender,
};
use crossbeam_channel as channel;

pub type FromControllerToSystem<M> = MessageChannel<M, FromController, ToSystem>;

pub type FromSystemToController<M> = MessageChannel<M, FromSystem, ToController>;

pub type FromControllerToController<M> = MessageChannel<M, FromController, ToController>;

pub struct MessageChannel<M, F: sealed::FromDirection, T: sealed::ToDirection> {
    pub sender: MessageSender<M, T>,
    pub receiver: MessageReceiver<M, F>,
}

impl<M, F: sealed::FromDirection, T: sealed::ToDirection> MessageChannel<M, F, T> {
    fn new(sender: channel::Sender<M>, receiver: channel::Receiver<M>) -> Self {
        Self {
            sender: MessageSender::new(sender),
            receiver: MessageReceiver::new(receiver),
        }
    }

    pub fn bounded(capacity: usize) -> Self {
        let (sender, receiver) = channel::bounded(capacity);

        Self::new(sender, receiver)
    }

    pub fn unbounded() -> Self {
        let (sender, receiver) = channel::unbounded();

        Self::new(sender, receiver)
    }

    pub fn split(self) -> (MessageSender<M, T>, MessageReceiver<M, F>) {
        (self.sender, self.receiver)
    }
}
