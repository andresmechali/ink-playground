use std::ops::Index;

use base_db::{CrateData, CrateGraph, Env};
use cfg::CfgOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CrateGraphJson {
    crates: Vec<(u32, CrateDataJson)>,
    deps: Vec<DepJson>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct CrateDataJson {
    root_file_id: u32,
    edition: String,
    display_name: Option<String>,
    cfg_options: CfgOptionsJson,
    potential_cfg_options: CfgOptionsJson,
    env: EnvJson,
    proc_macro: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct CfgOptionsJson {
    options: Vec<(String, Vec<String>)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct EnvJson {
    env: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DepJson {
    from: u32,
    name: String,
    to: u32,
}

impl CrateGraphJson {
    pub fn from(crate_graph: CrateGraph) -> Self {
        let mut deps: Vec<DepJson> = Vec::new();
        let crates = crate_graph
            .iter()
            .map(|id| (id, crate_graph.index(id)))
            .map(|(id, crate_data)| {
                (id.0, {
                    let crate_deps = crate_data
                        .dependencies
                        .iter()
                        .map(|dep| DepJson {
                            from: id.0,
                            name: dep.name.to_string(),
                            to: dep.crate_id.0,
                        })
                        .collect::<Vec<_>>();
                    deps.append(&mut crate_deps);
                    CrateDataJson::from(crate_data)
                })
            })
            .collect::<Vec<_>>();
        CrateGraphJson { crates, deps }
    }
    pub fn to_crate_graph(&self) -> CrateGraph {
        CrateGraph::default()
    }
}

impl CrateDataJson {
    fn from(crate_data: &CrateData) -> Self {
        let mut json = CrateDataJson::default();
        let root_file_id = crate_data.root_file_id.0;
        let edition = crate_data.edition.to_string();
        let display_name = match crate_data.display_name {
            Some(name) => Some(name.to_string()),
            None => None,
        };
        let cfg_options = CfgOptionsJson::from(crate_data.cfg_options);
        let potential_cfg_options = CfgOptionsJson::from(crate_data.potential_cfg_options);
        let env = EnvJson::from(crate_data.env);
        let proc_macro = Vec::new();
        CrateDataJson {
            root_file_id,
            edition,
            display_name,
            cfg_options,
            potential_cfg_options,
            env,
            proc_macro,
        }
    }
}

impl CfgOptionsJson {
    pub fn from(cfg_options: CfgOptions) -> Self {
        let options = cfg_options
            .get_cfg_keys()
            .iter()
            .map(|key| {
                (
                    String::from(key.as_str()),
                    cfg_options
                        .get_cfg_values(key)
                        .iter()
                        .map(|val| String::from(val.as_str()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        CfgOptionsJson { options }
    }
}

impl EnvJson {
    fn from(env: Env) -> Self {
        let env = env
            .iter()
            .map(|(a, b)| (String::from(a), String::from(b)))
            .collect::<Vec<(String, String)>>();
        EnvJson { env }
    }
}
