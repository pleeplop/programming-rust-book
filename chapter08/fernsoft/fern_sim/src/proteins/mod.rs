use self::AminoAcid::*;
use self::synthesis::synthesize;

pub enum AminoAcid {
    Lys
}

pub mod synthesis;

impl AminoAcid {
    pub fn lys() -> AminoAcid {
        Lys
    }
}