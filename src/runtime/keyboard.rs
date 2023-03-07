use serde::{Deserialize, Serialize};

//TODO : remove partial eq
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]

pub enum KeyModes {
    ALT,
    CTRL,
    SHIFT,
}
