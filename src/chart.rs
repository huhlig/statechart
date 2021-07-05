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

use super::{Action, Event, StateChartBuilder, StateMachine};
use crate::traits::EventId;
use crate::{StateMachineError, StateMachineResult};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::time::Duration;

/// Graph representation of State Chart
/// A Stateless representation of an Entities possible states.
///
#[derive(Serialize, Deserialize)]
pub struct StateChart {
    name: String,
    initial: StateId,
    states: Vec<State>,
}

impl StateChart {
    pub fn build<'a>() -> StateChartBuilder<'a> {
        StateChartBuilder::default()
    }
    pub fn update<E: Event>(
        &self,
        machine: StateMachine,
        event: E,
    ) -> StateMachineResult<StateMachine> {
        if let Some(state) = self.states.get(machine.current_state.0) {
            let hash = hash_event(&event);
            match state {
                State::Atomic {
                    id,
                    label,
                    on_entry,
                    on_exit,
                    transitions,
                } => {
                    for transition in transitions {
                        if transition.event.0 == hash {}
                    }
                }
                State::Compound { .. } => {}
                State::Final { .. } => {}
                State::Parallel { .. } => {}
            }
            Ok(StateMachine {
                current_state: StateId(0),
                duration: Default::default(),
            })
        } else {
            Err(StateMachineError::StateNotFound(Box::new(event)))
        }
    }
}

/// StateId
pub struct StateId(usize);

pub struct StateLabel(String);

/// State Graph Node
pub trait State {
    /// Get ID of State Node
    fn id(&self) -> &StateId;
    /// Get Label of State Node
    fn label(&self) -> &str;
    /// Get Substate by Label
    fn substates(&self) -> &Vec<StateId>;
    /// Get Parent of Node
    fn parent(&self) -> Option<&StateId>;
    /// Get Starting Node
    fn initial(&self) -> Option<&StateLabel>;
    /// Get OnEntry Action
    fn on_entry(&self) -> &Vec<dyn Action>;
    /// Get OnExit Action
    fn on_exit(&self) -> &Vec<dyn Action>;
}

/// Extension to Node for the Current Node
pub trait ActiveState: State {
    /// Active Transitions
    fn transitions(&self) -> &Vec<Transition>;
}

pub enum StateNode {
    Parallel {
        id: StateId,
        label: String,
        parent: Option<StateId>,
        on_entry: Vec<dyn Action>,
        on_exit: Vec<dyn Action>,
        initial: Option<StateId>,
        transitions: Vec<Transition>,
        states: Vec<StateId>,
    },
    Compound {
        id: StateId,
        label: String,
        parent: Option<StateId>,
        on_entry: Vec<dyn Action>,
        on_exit: Vec<dyn Action>,
        initial: Option<StateId>,
        transitions: Vec<Transition>,
        states: Vec<StateId>,
    },
    Atomic {
        id: StateId,
        label: String,
        parent: Option<StateId>,
        on_entry: Vec<dyn Action>,
        on_exit: Vec<dyn Action>,
        transitions: Vec<Transition>,
    },
    Final {
        id: StateId,
        label: String,
        parent: Option<StateId>,
        on_entry: Vec<dyn Action>,
    },
}

impl State for StateNode {
    /// ID of State
    fn id(&self) -> &StateId {
        match self {
            StateNode::Parallel { id, .. } => id,
            StateNode::Compound { id, .. } => id,
            StateNode::Atomic { id, .. } => id,
            StateNode::Final { id, .. } => id,
        }
    }

    /// Label of State
    fn label(&self) -> &str {
        match self {
            StateNode::Parallel { label, .. } => label,
            StateNode::Compound { label, .. } => label,
            StateNode::Atomic { label, .. } => label,
            StateNode::Final { label, .. } => label,
        }
    }

    fn substates(&self) -> &[StateId] {
        match self {
            StateNode::Parallel { states, .. } => states,
            StateNode::Compound { states, .. } => states,
            StateNode::Atomic { .. } => None,
            StateNode::Final { .. } => None,
        }
    }

    fn parent(&self) -> Option<&StateId> {
        match self {
            StateNode::Parallel { parent, .. } => Some(parent?),
            StateNode::Compound { parent, .. } => Some(parent?),
            StateNode::Atomic { parent, .. } => Some(parent?),
            StateNode::Final { parent, .. } => Some(parent?),
        }
    }

    fn initial(&self) -> Option<&StateId> {
        match self {
            StateNode::Parallel { initial, .. } => Some(initial?),
            StateNode::Compound { initial, .. } => Some(initial?),
            StateNode::Atomic { .. } => None,
            StateNode::Final { .. } => None,
        }
    }

    fn on_entry(&self) -> &[dyn Action] {
        match self {
            StateNode::Parallel { on_entry, .. } => on_entry,
            StateNode::Compound { on_entry, .. } => on_entry,
            StateNode::Atomic { on_entry, .. } => on_entry,
            StateNode::Final { on_entry, .. } => on_entry,
        }
    }

    fn on_exit(&self) -> &[dyn Action] {
        match self {
            StateNode::Parallel { on_exit, .. } => on_exit,
            StateNode::Compound { on_exit, .. } => on_exit,
            StateNode::Atomic { on_exit, .. } => on_exit,
            StateNode::Final { .. } => &[],
        }
    }
}

pub struct Transition {
    event: Vec<EventId>,
    condition: Condition,
    target: Option<StateId>,
    on_transition: Vec<dyn Action>,
}

pub struct Condition {}

fn hash_event<E: Event>(event: &E) -> u64 {
    let mut hasher = DefaultHasher::new();
    event.hash(&mut hasher);
    hasher.finish()
}
