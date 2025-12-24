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

use std::{
    any::Any,
    error,
    fmt::{self, Debug, Display, Formatter},
};

#[non_exhaustive]
pub enum Error {
    Thread(Box<dyn Any + Send + 'static>),
    MessageSender,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Thread(err) => write!(f, "{err:?}"),
            Self::MessageSender => write!(f, "MessageSenderError(..)"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Thread(err) => write!(f, "{err:?}"),
            Self::MessageSender => write!(f, "sending on a disconnected channel"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Thread(_) => None,
            Self::MessageSender => None,
        }
    }
}
