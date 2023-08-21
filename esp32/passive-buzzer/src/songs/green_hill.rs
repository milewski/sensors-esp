use crate::song::*;

pub struct GreenHill {}

impl GreenHill {
    pub fn new() -> Self {
        GreenHill {}
    }
}

impl Song for GreenHill {
    fn tempo(&self) -> f32 {
        140.0
    }

    fn notes(&self) -> &[(Note, i8)] {
        &[
            (REST, 2), (D5, 8), (B4, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            (REST, 8), (A4, 8), (FS5, 8), (E5, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            (REST, 4), (D5, 8), (B4, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            //--------------------------------------------------------------------------------------
            (REST, 8), (B4, 8), (B4, 8), (G4, 4), (B4, 8),
            (A4, 4), (B4, 8), (A4, 4), (D4, 2),
            (REST, 4), (D5, 8), (B4, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            (REST, 8), (A4, 8), (FS5, 8), (E5, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            //--------------------------------------------------------------------------------------
            (REST, 4), (D5, 8), (B4, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            (REST, 8), (B4, 8), (B4, 8), (G4, 4), (B4, 8),
            (A4, 4), (B4, 8), (A4, 4), (D4, 8), (D4, 8), (FS4, 8),
            (E4, -1),
            (REST, 8), (D4, 8), (E4, 8), (FS4, -1),
            //--------------------------------------------------------------------------------------
            (REST, 8), (D4, 8), (D4, 8), (FS4, 8), (F4, -1),
            (REST, 8), (D4, 8), (F4, 8), (E4, -1),
            //--------------------------------------------------------------------------------------
            (REST, 2), (D5, 8), (B4, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            (REST, 8), (A4, 8), (FS5, 8), (E5, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            (REST, 4), (D5, 8), (B4, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            //--------------------------------------------------------------------------------------
            (REST, 8), (B4, 8), (B4, 8), (G4, 4), (B4, 8),
            (A4, 4), (B4, 8), (A4, 4), (D4, 2),
            (REST, 4), (D5, 8), (B4, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            (REST, 8), (A4, 8), (FS5, 8), (E5, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            //--------------------------------------------------------------------------------------
            (REST, 4), (D5, 8), (B4, 4), (D5, 8),
            (CS5, 4), (D5, 8), (CS5, 4), (A4, 2),
            (REST, 8), (B4, 8), (B4, 8), (G4, 4), (B4, 8),
            (A4, 4), (B4, 8), (A4, 4), (D4, 8), (D4, 8), (FS4, 8),
            (E4, -1),
            (REST, 8), (D4, 8), (E4, 8), (FS4, -1),
            //--------------------------------------------------------------------------------------
            (REST, 8), (D4, 8), (D4, 8), (FS4, 8), (F4, -1),
            (REST, 8), (D4, 8), (F4, 8), (E4, 8),
            (E4, -2), (A4, 8), (CS5, 8),
            (FS5, 8), (E5, 4), (D5, 8), (A5, -4),
        ]
    }
}