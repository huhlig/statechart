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

use crate::{StateChartResult, StateChart};
use std::io::Read;

pub fn read<T: Read>(file: T) -> StateChartResult<StateChart> {
    let parser = xml::reader::EventReader::new(file);
}

pub enum SCXMLIdRef {
    RefName(String),
    RefId(usize),
}

pub enum SCXMLBinding {
    Early,
    Late,
}

pub struct SCXMLFile {

}
/// The top-level wrapper element, which carries version information. The actual state machine
/// consists of its children. Note that only one of the children is active at any one time. See
/// [3.11 Legal State Configurations and Specifications](https://www.w3.org/TR/scxml/#LegalStateConfigurations)
/// for details.
pub struct SCXMLRoot {
    /// The id of the initial state(s) for the document. If not specified, the default initial
    /// state is the first child state in document order.
    _initial: Option<SCXMLIdRef>,
    /// The name of this state machine. It is for purely informational purposes.
    _name: Option<String>,
    _xmlns: Option<String>,
    _version: Option<String>,
    /// The datamodel that this document requires. "null" denotes the Null datamodel, "ecmascript"
    /// the ECMAScript datamodel, and "xpath" the XPath datamodel, as defined in B Data Models.
    _datamodel: Option<String>,
    _binding: Option<String>,
    states: Vec<SCXMLState>,
    parallel: Vec<SCXMLParallel>,
    finals: Vec<SCXMLFinal>,
    datamodel: Option<SCXMLDataModel>,
    script: Option<SCXMLBinding>,
}

pub struct SCXMLState {
    _id: Option<String>,
    _initial: Option<String>,
    on_entry: Vec<String>,
    on_exit: Vec<String>,
    transition: Vec<SCXMLTransition>,
    initial: Option<SCXMLState>,
    states: Vec<SCXMLState>,
    parallel_states: Vec<SCXMLParallel>,
    final_states: Vec<SCXMLFinal>,
    history: Vec<SCXMLHistory>,

}