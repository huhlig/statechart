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

use std::io::Read;
use crate::StateChartResult;
use crate::chart::StateChart;
use std::collections::HashMap;

/// Loads SCXML Files into StateChart

fn read<T: Read>(input: T) -> StateChartResult<()> {
    let ron_state_chart = ron::de::from_bytes::<RonStateChart>(bytes)?;
    let mut state_names = HashMap::new();
    let mut states = Vec::new();

    for state in &ron_state_chart.states {
        index_states(state);
    }


    Ok(())
}

/// Context flattens the tiered nature of the text file.
#[derive(Clone, Debug)]
struct Context {
    labels: HashMap<String, usize>,
    states: Vec<RonState>,
}

impl Default for Context {
    fn default() -> Context {
        Context {
            ..Default::default()
        }
    }
}

impl Context {
    fn handle_state(&mut self, state: &RonState) {
        match state {
            RonState::Atomic{label,.. } => {
                self.labels.insert(state.label.clone(), states.len());
                self.states.push(state.clone());
            }
            RonState::Compound{states, ..} => {
                self.labels.insert(state.label.clone(), states.len());
                self.states.push(state.clone());
                for substate in states {
                    self.handle_state(substate)
                }
            }
            RonState::Final => {
                self.labels.insert(state.label.clone(), states.len());
                self.states.push(state.clone());
            }
            RonState::Parallel => {
                self.labels.insert(state.label.clone(), states.len());
                self.states.push(state.clone());
                for substate in states {
                    self.handle_state(substate)
                }
            }
        }

    }
}

#[derive(Clone, Debug)]
struct RonStateChart {
    name: String,
    initial: String,
    states: Vec<RonState>,
}

#[derive(Clone, Debug)]
enum RonState {
    /// Atomic State
    Atomic {
        label: String,
        initial: String,
        on_entry: Vec<RonCommand>,
        on_exit: Vec<RonCommand>,
        transitions: Vec<RonTransistion>,
    },
    /// Compound State (I.E. State Sub Graph)
    Compound {
        label: String,
        initial: String,
        on_entry: Vec<RonCommand>,
        on_exit: Vec<RonCommand>,
        states: Vec<RonState>,
        transitions: Vec<RonTransistion>,
    },
    /// Compound State (I.E. Parallel Execution of Multiple SubState Graphs)
    Parallel {
        label: String,
        initial: Vec<String>,
        on_entry: Vec<RonCommand>,
        on_exit: Vec<RonCommand>,
        states: Vec<RonState>,
        transitions: Vec<RonTransistion>,
    },
    /// Final State (I.E. State Terminus, State Machine Halt. Must be reset after reaching here)
    Final {
        label: String,
        initial: String,
        on_entry: Vec<RonCommand>,
    },
}

/// Command
#[derive(Clone, Debug)]
struct RonCommand {
    command: String,
    args: Vec<String>,
}

/// State Transition
#[derive(Clone, Debug)]
struct RonTransition {
    target: String,
    condition: RonCondition,
    on_transition: Vec<RonCommand>,
}

#[derive(Clone, Debug)]
enum RonCondition {
    Delay(f32),
    Event(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
