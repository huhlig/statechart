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

use std::collections::HashMap;
use std::hash::Hash;

pub struct ActionId(u64);
pub struct ActionLabel(String);

/// StateChart Output Action
/// TODO: Reevaluate and determine if correct solution
pub trait Action {
    /// Unique ID for this Action
    fn id(&self) -> &ActionId;
    /// Unique Name for this Action
    fn name(&self) -> &ActionLabel;
    /// Perform this action on the provided context.
    fn execute<C>(&self, ctx: C);
}

pub struct EventId(u64);
pub struct EventLabel(String);

/// StateChart Input Event
/// TODO: Reevaluate and determine if correct solution
pub trait Event {
    /// Deterministic id
    fn id(&self) -> &EventId;
    /// Deterministic name
    fn name(&self) -> &EventLabel;
    /// Event properties
    /// TODO: Don't like this, need a better way
    fn properties(&self) -> HashMap<String, String>;
}
