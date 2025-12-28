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
use crossbeam_channel::Receiver;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct MessageReceiver<M, F: sealed::FromDirection> {
    inner: Receiver<M>,
    _direction: PhantomData<F>,
}

impl<M, F: sealed::FromDirection> MessageReceiver<M, F> {
    #[inline]
    pub(super) fn new(sender: Receiver<M>) -> Self {
        Self {
            inner: sender,
            _direction: PhantomData,
        }
    }

    #[inline]
    pub fn drain(&self) -> Vec<M> {
        self.inner.try_iter().collect()
    }

    #[inline]
    pub fn drain_limit(&self, limit: usize) -> Vec<M> {
        self.inner.try_iter().take(limit).collect()
    }

    #[inline]
    pub fn try_recv(&self) -> Option<M> {
        self.inner.try_recv().ok()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = M> + '_ {
        self.inner.try_iter()
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
