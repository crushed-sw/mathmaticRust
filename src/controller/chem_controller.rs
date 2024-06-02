use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    entity::{chem::{Atom, ChemResponse, Orbit, Smiles}, result::Result, result_code::{self, ResultCode}},
    service::chem_service::ChemService,
};

#[get("/molecule")]
pub async fn molecule(smiles: web::Query<Smiles>) -> impl Responder {
    let result = ChemService::get_mol_service(&smiles);
    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result)
}

#[get("/atom")]
pub async fn atom(inform: web::Query<Atom>) -> impl Responder {
    let result = if inform.atom_type == "real" {
        let atom_surface_result = ChemService::get_real_surface_atom(inform.n.as_str(), inform.l.as_str(), inform.m.as_str());
        let atom_points_result = ChemService::get_real_points_atom(inform.n.as_str(), inform.l.as_str(), inform.m.as_str());

        match [atom_surface_result, atom_points_result] {
            [Ok(surface), Ok(points)] => Result::success(200, ResultCode::get_message(200), ChemResponse {surface, points}),
            _ => Result::error(1, ResultCode::get_message(1)),
        }
    } else if inform.atom_type == "complex" {
        let atom_result = ChemService::get_complex_atom(inform.n.as_str(), inform.l.as_str(), inform.m.as_str());
        match atom_result {
            Err(_) => Result::error(1, ResultCode::get_message(1)),
            Ok(value) => Result::success(200, ResultCode::get_message(200), value),
        }
    } else {
        Result::error(1, ResultCode::get_message(1))
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/orbit")]
pub async fn orbit(inform: web::Query<Orbit>) -> impl Responder {
    let atom_result = ChemService::get_orbit(inform.name.as_str());
    let result = match atom_result {
        Err(_) => Result::error(1, ResultCode::get_message(1)),
        Ok(value) => Result::success(200, ResultCode::get_message(200), value),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}
