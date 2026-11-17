use crate::cli::View;

pub const DARK_LUMINANCE: f32 = 0.8;

#[derive(Clone, Copy)]
pub struct Rect(pub u32, pub u32, pub u32, pub u32);

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct Part {
    pub source: Rect,
    pub target: Rect,
    pub x_flipped: Option<bool>,
    pub y_flipped: Option<bool>,
    pub luminance: Option<f32>,
}

impl Part {
    const fn new(source: Rect, target: Rect) -> Self {
        Self {
            source,
            target,
            x_flipped: None,
            y_flipped: None,
            luminance: None,
        }
    }

    const fn darken(mut self, luminance: f32) -> Self {
        self.luminance = Some(luminance);
        self
    }
}

const RIGHT_34: &[Part] = &[
    // Head //
    Part::new(Rect(8, 8, 8, 8), Rect(7, 1, 16, 16)), // Front
    Part::new(Rect(0, 8, 8, 8), Rect(1, 1, 6, 16)).darken(DARK_LUMINANCE), // Right side
    // Hairs //
    Part::new(Rect(40, 8, 8, 8), Rect(6, 0, 18, 18)), // Front
    Part::new(Rect(32, 8, 8, 8), Rect(0, 0, 7, 17)).darken(DARK_LUMINANCE), // Right side
    // Torso //
    Part::new(Rect(20, 20, 8, 12), Rect(9, 17, 10, 10)), // Front
    // Arms //
    Part::new(Rect(44, 20, 4, 12), Rect(5, 17, 4, 10)), // Right front
    Part::new(Rect(40, 20, 4, 12), Rect(2, 17, 3, 10)).darken(DARK_LUMINANCE), // Right side
    Part::new(Rect(36, 52, 4, 12), Rect(19, 17, 4, 10)), // Left front
    // Legs //
    Part::new(Rect(4, 20, 4, 12), Rect(9, 27, 5, 6)), // Right front
    Part::new(Rect(0, 20, 4, 12), Rect(5, 27, 4, 6)).darken(DARK_LUMINANCE), // Right side
    Part::new(Rect(20, 52, 4, 12), Rect(14, 27, 5, 6)), // Left front
];

pub fn mapping(view: View) -> &'static [Part] {
    match view {
        View::Right34 => RIGHT_34,
        _ => unreachable!(),
    }
}
