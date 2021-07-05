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

use crate::{Action, StateId};

pub struct StateChartBuilder<'a> {
    name: Option<String>,
    initial: Option<StateId>,
    states: Vec<StateBuilder<'a>>,
}

impl<'a> StateChartBuilder<'a> {
    pub fn build() -> StateChartBuilder<'a> {
        StateChartBuilder {
            name: None,
            initial: None,
            states: Vec::new(),
        }
    }
    pub fn add_state(&mut self) -> StateBuilder<'a> {
        StateBuilder {
            parent: None,
            id: None,
            label: None,
            on_entry: Vec::new(),
            on_exit: Vec::new(),
            initial: None,
            transitions: Vec::new(),
            parallel: false,
            states: Vec::new(),
        }
    }
}

pub struct StateBuilder<'a> {
    parent: Option<StateId>,
    id: Option<StateId>,
    label: Option<String>,
    on_entry: Vec<Box<dyn Action>>,
    on_exit: Vec<Box<dyn Action>>,
    initial: Option<usize>,
    transitions: Vec<TransitionBuilder<'a>>,
    parallel: bool,
    states: Vec<usize>,
}

pub struct TransitionBuilder<'a> {}
