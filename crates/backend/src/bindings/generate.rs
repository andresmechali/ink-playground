// Copyright 2018-2021 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unnameable_test_items)]

use crate::{
    bindings::utils::{
        clean_dir,
        generate_package_json_file,
        generate_ts_re_export_index,
        PackageJson,
    },
    services::compile::{
        CompilationRequest,
        CompilationResult,
    },
};
use std::{
    fs::remove_dir_all,
    io::Result,
    path::{
        Path,
        PathBuf,
    },
};
use ts_rs::export;

// -------------------------------------------------------------------------------------------------
// CODE GENERATION
// -------------------------------------------------------------------------------------------------

#[test]
fn clean() -> Result<()> {
    let project_dir_path = PathBuf::from("../../node_modules/@paritech/commontypes");
    let src_dir_path = project_dir_path.join(PathBuf::from("src"));

    let package_json = PackageJson {
        name: "@paritech/commontypes".to_string(),
        types: "src/index.d.ts".to_string(),
    };

    export! {
        (declare) CompilationResult => "../../node_modules/@paritech/commontypes/src/CompilationResult.d.ts",
        (declare) CompilationRequest => "../../node_modules/@paritech/commontypes/src/CompilationRequest.d.ts"
    }

    clean_dir(&project_dir_path)?;
    export_typescript();
    generate_package_json_file(&package_json, &src_dir_path)?;
    generate_ts_re_export_index(&src_dir_path)?;

    Ok(())
}
