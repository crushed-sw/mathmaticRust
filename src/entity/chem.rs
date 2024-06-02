use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Smiles {
    pub value: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Atom {
    pub n: String,
    pub l: String,
    pub m: String,
    pub atom_type: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Orbit {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct ChemResponse {
    pub surface: String,
    pub points: String,
}
