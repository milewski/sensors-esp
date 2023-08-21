use crate::song::*;

pub struct Tetris {}

impl Tetris {
    pub fn new() -> Self {
        Tetris {}
    }
}

impl Song for Tetris {
    fn tempo(&self) -> f32 {
        144.0
    }

    fn notes(&self) -> &[(Note, i8)] {
        &[
            (E5, 4), (B4, 8), (C5, 8), (D5, 4), (C5, 8), (B4, 8),
            (A4, 4), (A4, 8), (C5, 8), (E5, 4), (D5, 8), (C5, 8),
            (B4, -4), (C5, 8), (D5, 4), (E5, 4),
            (C5, 4), (A4, 4), (A4, 8), (A4, 4), (B4, 8), (C5, 8),
            // -------------------------------------------------------------------------------------
            (D5, -4), (F5, 8), (A5, 4), (G5, 8), (F5, 8),
            (E5, -4), (C5, 8), (E5, 4), (D5, 8), (C5, 8),
            (B4, 4), (B4, 8), (C5, 8), (D5, 4), (E5, 4),
            (C5, 4), (A4, 4), (A4, 4), (REST, 4),
            // -------------------------------------------------------------------------------------
            (E5, 4), (B4, 8), (C5, 8), (D5, 4), (C5, 8), (B4, 8),
            (A4, 4), (A4, 8), (C5, 8), (E5, 4), (D5, 8), (C5, 8),
            (B4, -4), (C5, 8), (D5, 4), (E5, 4),
            (C5, 4), (A4, 4), (A4, 8), (A4, 4), (B4, 8), (C5, 8),
            // -------------------------------------------------------------------------------------
            (D5, -4), (F5, 8), (A5, 4), (G5, 8), (F5, 8),
            (E5, -4), (C5, 8), (E5, 4), (D5, 8), (C5, 8),
            (B4, 4), (B4, 8), (C5, 8), (D5, 4), (E5, 4),
            (C5, 4), (A4, 4), (A4, 4), (REST, 4),
            // -------------------------------------------------------------------------------------
            (E5, 2), (C5, 2),
            (D5, 2), (B4, 2),
            (C5, 2), (A4, 2),
            (GS4, 2), (B4, 4), (REST, 8),
            (E5, 2), (C5, 2),
            (D5, 2), (B4, 2),
            (C5, 4), (E5, 4), (A5, 2),
            (GS5, 2),
        ]
    }
}