//! Handle deps
//!
//! mm12 was... ok this is getting old now.

extern crate serde;
extern crate serde_json;

use serde_json::Value;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub enum InstanceDep {

    Java {version: i32},
    Minecraft {version: String},
    Forge {version: String},
    File {file_type: FileType, name: String, location: String}

}

#[derive(Serialize, Deserialize)]
pub enum FileType {

    Map,
    Mod,
    ResourcePack,
    Arbitrary {location: String}

}

