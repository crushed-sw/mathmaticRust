use cpython::{PyResult, Python};
use crate::{
    entity::chem::{Atom, Smiles}, service::CONTEXT, util::error::Error
};

pub struct ChemService;

impl ChemService {
    fn get_mol(py: Python, smile: &Smiles) -> PyResult<String> {
        let chem = py.import("rdkit.Chem")?;
        let all_chem = py.import("rdkit.Chem.AllChem")?;

        let mol = chem.call(py, "MolFromSmiles", (smile.value.as_str(),), None)?;
        let mol = chem.call(py, "AddHs", (mol,), None)?;

        all_chem.call(py, "EmbedMolecule", (&mol,), None)?;
        all_chem.call(py, "MMFFOptimizeMolecule", (&mol,), None)?;

        let result = chem.call(py, "MolToMolBlock", (mol,), None)?;

        Ok(result.to_string())
    }

    pub fn get_mol_service(smile: &Smiles) -> String {
        let gil = Python::acquire_gil();
        let py = gil.python();

        match Self::get_mol(py, smile) {
            Ok(value) => value,
            Err(_) => String::from(""),
        }
    }

    pub fn get_real_surface_atom(n: &str, l: &str, m: &str) -> Result<String, Error> {
        let name = format!("{}{}{}", n, l, m);
        let value = CONTEXT.atom_real_surface_map.get(&name);
        match value {
            None => Err(Error::from("")),
            Some(str) => Ok(str.clone()),
        }
    }

    pub fn get_real_points_atom(n: &str, l: &str, m: &str) -> Result<String, Error> {
        let name = format!("{}{}{}", n, l, m);
        let value = CONTEXT.atom_real_points_map.get(&name);
        match value {
            None => Err(Error::from("")),
            Some(str) => Ok(str.clone()),
        }
    }


    pub fn get_complex_atom(n: &str, l: &str, m: &str) -> Result<String, Error> {
        let name = format!("{}{}{}", n, l, m);
        let value = CONTEXT.atom_complex_map.get(&name);
        match value {
            None => Err(Error::from("")),
            Some(str) => Ok(str.clone()),
        }
    }

    pub fn get_orbit(type_name: &str) -> Result<String, Error> {
        let name = format!("{}", type_name);
        let value = CONTEXT.orbit_map.get(&name);
        match value {
            None => Err(Error::from("")),
            Some(str) => Ok(str.clone()),
        }
    }

}
