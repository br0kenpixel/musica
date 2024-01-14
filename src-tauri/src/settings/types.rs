use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Display, Serialize, Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[display(fmt = "light")]
    #[default]
    Light,
    #[display(fmt = "dark")]
    Dark,
}

#[derive(Display, Serialize, Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ScanPreference {
    #[display(fmt = "manual")]
    #[default]
    Manual,
    #[display(fmt = "on-start")]
    OnStart,
}

#[derive(Display, Serialize, Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum HomePage {
    #[display(fmt = "history")]
    #[default]
    History,
    #[display(fmt = "library")]
    Library,
}
