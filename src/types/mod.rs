use std::fmt;
use std::str::FromStr;
use serde::Serialize;

#[derive(Default, Debug, Eq, PartialEq,Serialize)]
pub enum ModuleState {
    #[default]
    Live,
    Loading,
    Unloading
}

impl FromStr for ModuleState {

    type Err = ();

    fn from_str(input: &str) -> Result<ModuleState, Self::Err> {
        match input {
            "Live"  => Ok(ModuleState::Live),
            "Loading"  => Ok(ModuleState::Loading),
            "Unloading"  => Ok(ModuleState::Unloading),
            _      => Err(()),
        }
    }
}

impl fmt::Display for ModuleState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Default, Debug,Serialize)]
pub struct Module {
    pub name: String,
    pub memory: String,
    pub instances: i32,
    pub depends_on: String,
    pub state: ModuleState,
    pub memory_offset: String,
}