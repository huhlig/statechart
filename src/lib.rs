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

mod builder;
mod chart;
mod engine;
mod result;
mod traits;

pub use self::builder::{StateBuilder, StateChartBuilder};
pub use self::chart::{StateChart, StateId};
pub use self::engine::StateMachine;
pub use self::result::{StateMachineError, StateMachineResult};
pub use self::traits::{Action, Event};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building() {
        let mut builder = StateChart::build();

        builder.add_state().name("top_state_1").finish()
    }
}
