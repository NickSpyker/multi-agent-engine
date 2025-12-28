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

use super::direction::sealed;
use crossbeam_channel::{Sender, TrySendError};
use multi_agent_engine_core::{Error, Result};
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct MessageSender<M, T: sealed::ToDirection> {
    inner: Sender<M>,
    _direction: PhantomData<T>,
}

impl<M, T: sealed::ToDirection> MessageSender<M, T> {
    #[inline]
    pub(super) fn new(sender: Sender<M>) -> Self {
        Self {
            inner: sender,
            _direction: PhantomData,
        }
    }

    #[inline]
    pub fn send(&self, message: M) -> Result<()> {
        self.inner.try_send(message).map_err(|err| match err {
            TrySendError::Full(_) => Error::MessageSenderFull,
            TrySendError::Disconnected(_) => Error::MessageSenderDisconnected,
        })
    }

    #[inline]
    pub fn send_lossy(&self, message: M) {
        let _ = self.send(message);
    }

    #[inline]
    pub fn pending(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn capacity(&self) -> Option<usize> {
        self.inner.capacity()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.inner.is_full()
    }
}
