//
// Copyright 2021 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use super::{Event, StateId};
use std::fmt::Formatter;

/// Result for StateMachine Methods
pub type StateMachineResult<T> = std::result::Result<T, StateMachineError>;

/// Error Type for StateChart methods
pub enum StateMachineError {
    StateNotFound(Box<StateId>, Box<dyn Event>),
}

impl std::fmt::Display for StateMachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StateNotFound(id, event) => write!(f, "StateNotFound({} -> {})", id, event),
        }
    }
}

impl std::fmt::Debug for StateMachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StateNotFound(id, event) => {
                write!(f, "StateMachine::StateNotFound({} -> {})", id, event)
            }
        }
    }
}

impl std::error::Error for StateMachineError {}
