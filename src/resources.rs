use bevy::prelude::*;
use d_vector::{DVector, Real};
use mol_job::{boundaries::Region, initial_state::cubic_lattice};

use crate::{DELTA_T, DENSITY, N_MOL, TAU};

pub struct Settings {
    pub n_mol: usize,
    pub density: Real,
    pub delta_t: Real,
    pub tau: Real,
}

pub struct Textures {
    pub atom: Handle<Image>,
}

pub type MolVector = DVector<3>;
pub struct Pos(pub Vec<MolVector>);

impl AsMut<[MolVector]> for Pos {
    fn as_mut(&mut self) -> &mut [MolVector] {
        self.0.as_mut()
    }
}

pub struct Vel(pub Vec<MolVector>);

impl AsMut<[MolVector]> for Vel {
    fn as_mut(&mut self) -> &mut [MolVector] {
        self.0.as_mut()
    }
}

pub struct Acc(pub Vec<MolVector>);

impl AsMut<[MolVector]> for Acc {
    fn as_mut(&mut self) -> &mut [MolVector] {
        self.0.as_mut()
    }
}

pub struct Wrapper(pub Region<3>);
pub struct TimeNow(pub Real);

pub fn init(commands: &mut Commands, asset_server: &AssetServer) {
    let settings = load_env();
    commands.insert_resource(Textures {
        atom: asset_server.load("sphere.png"),
    });
    let (boundaries, pos) = cubic_lattice::<3>(settings.n_mol, settings.density);
    let n_mol_actual = pos.len();
    commands.insert_resource(Pos(pos));
    commands.insert_resource(Vel(vec![MolVector::default(); n_mol_actual]));
    commands.insert_resource(Acc(vec![MolVector::default(); n_mol_actual]));
    commands.insert_resource(Wrapper(boundaries));
    commands.insert_resource(TimeNow(0_f32));
    commands.insert_resource(settings);
}

fn load_env() -> Settings {
    dotenv::dotenv().ok();
    let n_mol = std::env::var(N_MOL)
        .unwrap_or_default()
        .parse::<usize>()
        .unwrap_or(8_usize);
    let density = std::env::var(DENSITY)
        .unwrap_or_default()
        .parse::<Real>()
        .unwrap_or(1 as Real);
    let delta_t = std::env::var(DELTA_T)
        .unwrap_or_default()
        .parse::<Real>()
        .unwrap_or(0.005 as Real);
    let tau = std::env::var(TAU)
        .unwrap_or_default()
        .parse::<Real>()
        .unwrap_or(2.16e-12) * 1.0e+9;
    Settings {
        n_mol,
        density,
        delta_t,
        tau,
    }
}
