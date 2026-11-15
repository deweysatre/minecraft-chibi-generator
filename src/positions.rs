use crate::cli::View;

#[derive(Clone, Copy)]
pub struct Rect(pub u32, pub u32, pub u32, pub u32);

#[derive(Clone, Copy)]
pub struct Part {
    pub base: Rect,
    pub overlay: Option<Rect>,
    pub destination: Rect,
    pub wide_destination: Option<Rect>,
    pub side: bool,
    pub mirror_x: bool,
}

/// Creates a drawable part without an overlay or mirrored rendering.
const fn part(base: Rect, destination: Rect) -> Part {
    Part {
        base,
        overlay: None,
        destination,
        wide_destination: None,
        side: false,
        mirror_x: false,
    }
}

/// Creates a drawable part composed of a base texture and an overlay texture.
const fn layered(base: Rect, overlay: Rect, destination: Rect) -> Part {
    Part {
        base,
        overlay: Some(overlay),
        destination,
        wide_destination: None,
        side: false,
        mirror_x: false,
    }
}

/// Creates a drawable side part composed of a base texture and an overlay.
const fn side(base: Rect, overlay: Rect, destination: Rect) -> Part {
    Part {
        base,
        overlay: Some(overlay),
        destination,
        wide_destination: None,
        side: true,
        mirror_x: false,
    }
}

/// Creates an arm part and stores its alternate destination for wide skins.
const fn arm(base: Rect, overlay: Rect, destination: Rect, wide_destination: Rect) -> Part {
    Part {
        base,
        overlay: Some(overlay),
        destination,
        wide_destination: Some(wide_destination),
        side: false,
        mirror_x: false,
    }
}

/// Marks a part for horizontal mirroring while preserving its other properties.
const fn mirrored(mut value: Part) -> Part {
    value.mirror_x = true;
    value
}

// These tables are the Classic renderer tables from the site. The order is
// significant: hidden/far parts must be drawn before the visible parts.
const FRONT: &[Part] = &[
    part(Rect(4, 20, 4, 12), Rect(6, 27, 4, 6)),
    part(Rect(20, 52, 4, 12), Rect(10, 27, 4, 6)),
    layered(Rect(4, 36, 4, 12), Rect(4, 52, 4, 12), Rect(6, 27, 4, 6)),
    layered(Rect(20, 36, 4, 12), Rect(52, 52, 4, 12), Rect(10, 27, 4, 6)),
    layered(Rect(20, 20, 8, 12), Rect(20, 36, 8, 12), Rect(6, 17, 8, 10)),
    arm(
        Rect(44, 20, 4, 12),
        Rect(44, 36, 4, 12),
        Rect(2, 17, 4, 10),
        Rect(2, 17, 4, 10),
    ),
    arm(
        Rect(36, 52, 4, 12),
        Rect(52, 52, 4, 12),
        Rect(14, 17, 4, 10),
        Rect(14, 17, 4, 10),
    ),
    part(Rect(8, 8, 8, 8), Rect(2, 1, 16, 16)),
    part(Rect(40, 8, 8, 8), Rect(1, 0, 18, 18)),
];

const BACK: &[Part] = &[
    part(Rect(12, 20, 4, 12), Rect(10, 27, 4, 6)),
    part(Rect(28, 52, 4, 12), Rect(6, 27, 4, 6)),
    layered(Rect(12, 36, 4, 12), Rect(12, 52, 4, 12), Rect(10, 27, 4, 6)),
    layered(Rect(28, 36, 4, 12), Rect(60, 52, 4, 12), Rect(6, 27, 4, 6)),
    layered(Rect(32, 20, 8, 12), Rect(32, 36, 8, 12), Rect(6, 17, 8, 10)),
    arm(
        Rect(52, 20, 4, 12),
        Rect(52, 36, 4, 12),
        Rect(14, 17, 4, 10),
        Rect(14, 17, 4, 10),
    ),
    arm(
        Rect(44, 52, 4, 12),
        Rect(60, 52, 4, 12),
        Rect(2, 17, 4, 10),
        Rect(2, 17, 4, 10),
    ),
    part(Rect(24, 8, 8, 8), Rect(2, 1, 16, 16)),
    part(Rect(56, 8, 8, 8), Rect(1, 0, 18, 18)),
];

const RIGHT_34: &[Part] = &[
    side(Rect(48, 20, 4, 12), Rect(48, 36, 4, 12), Rect(5, 17, 3, 10)),
    side(
        Rect(28, 20, 4, 12),
        Rect(28, 36, 4, 12),
        Rect(15, 17, 4, 10),
    ),
    side(Rect(8, 20, 4, 12), Rect(8, 36, 4, 12), Rect(10, 27, 4, 6)),
    part(Rect(4, 20, 4, 12), Rect(14, 27, 5, 6)),
    part(Rect(20, 52, 4, 12), Rect(9, 27, 5, 6)),
    side(Rect(24, 52, 4, 12), Rect(8, 52, 4, 12), Rect(5, 27, 4, 6)),
    side(Rect(28, 20, 4, 12), Rect(28, 36, 4, 12), Rect(8, 17, 1, 10)),
    layered(
        Rect(20, 20, 8, 12),
        Rect(20, 36, 8, 12),
        Rect(9, 17, 10, 10),
    ),
    arm(
        Rect(44, 20, 4, 12),
        Rect(44, 36, 4, 12),
        Rect(19, 17, 3, 10),
        Rect(19, 17, 4, 10),
    ),
    arm(
        Rect(36, 52, 4, 12),
        Rect(52, 52, 4, 12),
        Rect(5, 17, 3, 10),
        Rect(5, 17, 4, 10),
    ),
    side(Rect(40, 52, 4, 12), Rect(56, 52, 4, 12), Rect(2, 17, 3, 10)),
    part(Rect(8, 8, 8, 8), Rect(7, 1, 16, 16)),
    side(Rect(16, 8, 8, 8), Rect(48, 8, 8, 8), Rect(1, 1, 6, 16)),
    part(Rect(40, 8, 8, 8), Rect(6, 0, 18, 18)),
    side(Rect(48, 8, 8, 8), Rect(48, 8, 8, 8), Rect(0, 0, 6, 18)),
];

const LEFT_34: &[Part] = &[
    side(
        Rect(48, 20, 4, 12),
        Rect(48, 36, 4, 12),
        Rect(16, 17, 3, 10),
    ),
    side(Rect(28, 20, 4, 12), Rect(28, 36, 4, 12), Rect(5, 17, 4, 10)),
    side(Rect(8, 20, 4, 12), Rect(8, 36, 4, 12), Rect(10, 27, 4, 6)),
    part(Rect(4, 20, 4, 12), Rect(5, 27, 5, 6)),
    part(Rect(20, 52, 4, 12), Rect(10, 27, 5, 6)),
    side(Rect(24, 52, 4, 12), Rect(8, 52, 4, 12), Rect(15, 27, 4, 6)),
    side(
        Rect(28, 20, 4, 12),
        Rect(28, 36, 4, 12),
        Rect(15, 17, 1, 10),
    ),
    layered(
        Rect(20, 20, 8, 12),
        Rect(20, 36, 8, 12),
        Rect(5, 17, 10, 10),
    ),
    arm(
        Rect(44, 20, 4, 12),
        Rect(44, 36, 4, 12),
        Rect(2, 17, 3, 10),
        Rect(1, 17, 4, 10),
    ),
    arm(
        Rect(36, 52, 4, 12),
        Rect(52, 52, 4, 12),
        Rect(16, 17, 3, 10),
        Rect(15, 17, 4, 10),
    ),
    side(
        Rect(40, 52, 4, 12),
        Rect(56, 52, 4, 12),
        Rect(19, 17, 3, 10),
    ),
    part(Rect(8, 8, 8, 8), Rect(1, 1, 16, 16)),
    side(Rect(16, 8, 8, 8), Rect(48, 8, 8, 8), Rect(17, 1, 6, 16)),
    part(Rect(40, 8, 8, 8), Rect(0, 0, 18, 18)),
    side(Rect(48, 8, 8, 8), Rect(48, 8, 8, 8), Rect(18, 0, 6, 18)),
];

const BACK_RIGHT_34: &[Part] = &[
    side(
        Rect(32, 52, 4, 12),
        Rect(48, 52, 4, 12),
        Rect(16, 17, 3, 10),
    ),
    side(Rect(16, 20, 4, 12), Rect(16, 36, 4, 12), Rect(5, 17, 4, 10)),
    side(Rect(16, 52, 4, 12), Rect(0, 52, 4, 12), Rect(10, 27, 4, 6)),
    part(Rect(28, 52, 4, 12), Rect(5, 27, 5, 6)),
    part(Rect(12, 20, 4, 12), Rect(10, 27, 5, 6)),
    side(Rect(0, 20, 4, 12), Rect(0, 36, 4, 12), Rect(15, 27, 4, 6)),
    side(Rect(16, 20, 4, 12), Rect(16, 36, 4, 12), Rect(8, 17, 1, 10)),
    layered(
        Rect(32, 20, 8, 12),
        Rect(32, 36, 8, 12),
        Rect(9, 17, 10, 10),
    ),
    arm(
        Rect(44, 52, 4, 12),
        Rect(60, 52, 4, 12),
        Rect(2, 17, 3, 10),
        Rect(2, 17, 4, 10),
    ),
    arm(
        Rect(52, 20, 4, 12),
        Rect(52, 36, 4, 12),
        Rect(16, 17, 3, 10),
        Rect(15, 17, 4, 10),
    ),
    side(
        Rect(40, 20, 4, 12),
        Rect(40, 36, 4, 12),
        Rect(19, 17, 3, 10),
    ),
    part(Rect(24, 8, 8, 8), Rect(1, 1, 16, 16)),
    side(Rect(0, 8, 8, 8), Rect(32, 8, 8, 8), Rect(17, 1, 6, 16)),
    part(Rect(56, 8, 8, 8), Rect(0, 0, 18, 18)),
    side(Rect(32, 8, 8, 8), Rect(32, 8, 8, 8), Rect(18, 0, 6, 18)),
];

const BACK_LEFT_34: &[Part] = &[
    side(
        Rect(48, 20, 4, 12),
        Rect(48, 36, 4, 12),
        Rect(16, 17, 3, 10),
    ),
    side(Rect(28, 20, 4, 12), Rect(28, 36, 4, 12), Rect(5, 17, 4, 10)),
    side(Rect(8, 20, 4, 12), Rect(8, 36, 4, 12), Rect(10, 27, 4, 6)),
    side(Rect(24, 52, 4, 12), Rect(8, 52, 4, 12), Rect(5, 27, 4, 6)),
    part(Rect(28, 52, 4, 12), Rect(9, 27, 5, 6)),
    part(Rect(12, 20, 4, 12), Rect(14, 27, 5, 6)),
    side(Rect(28, 20, 4, 12), Rect(28, 36, 4, 12), Rect(8, 17, 1, 10)),
    layered(
        Rect(32, 20, 8, 12),
        Rect(32, 36, 8, 12),
        Rect(9, 17, 10, 10),
    ),
    side(Rect(40, 52, 4, 12), Rect(56, 52, 4, 12), Rect(2, 17, 3, 10)),
    arm(
        Rect(44, 52, 4, 12),
        Rect(60, 52, 4, 12),
        Rect(5, 17, 3, 10),
        Rect(5, 17, 4, 10),
    ),
    arm(
        Rect(52, 20, 4, 12),
        Rect(52, 36, 4, 12),
        Rect(19, 17, 3, 10),
        Rect(19, 17, 4, 10),
    ),
    side(Rect(16, 8, 8, 8), Rect(48, 8, 8, 8), Rect(1, 1, 6, 16)),
    part(Rect(24, 8, 8, 8), Rect(7, 1, 16, 16)),
    side(Rect(48, 8, 8, 8), Rect(48, 8, 8, 8), Rect(0, 0, 6, 18)),
    part(Rect(56, 8, 8, 8), Rect(6, 0, 18, 18)),
];

const RIGHT: &[Part] = &[
    side(Rect(8, 20, 4, 12), Rect(8, 36, 4, 12), Rect(8, 27, 4, 6)),
    side(Rect(28, 20, 4, 12), Rect(28, 36, 4, 12), Rect(8, 17, 4, 10)),
    side(Rect(48, 20, 4, 12), Rect(48, 36, 4, 12), Rect(8, 17, 4, 10)),
    part(Rect(16, 52, 4, 12), Rect(8, 27, 4, 6)),
    part(Rect(0, 20, 4, 12), Rect(8, 27, 4, 6)),
    part(Rect(32, 52, 4, 12), Rect(8, 17, 4, 10)),
    part(Rect(16, 20, 4, 12), Rect(8, 17, 4, 10)),
    side(Rect(0, 8, 8, 8), Rect(32, 8, 8, 8), Rect(2, 1, 16, 16)),
    part(Rect(32, 8, 8, 8), Rect(1, 0, 18, 18)),
];

const LEFT: &[Part] = &[
    side(Rect(24, 52, 4, 12), Rect(8, 52, 4, 12), Rect(8, 27, 4, 6)),
    side(Rect(28, 20, 4, 12), Rect(28, 36, 4, 12), Rect(8, 17, 4, 10)),
    side(Rect(40, 52, 4, 12), Rect(56, 52, 4, 12), Rect(8, 17, 4, 10)),
    part(Rect(20, 52, 4, 12), Rect(8, 27, 4, 6)),
    part(Rect(8, 20, 4, 12), Rect(8, 27, 4, 6)),
    part(Rect(40, 20, 4, 12), Rect(8, 17, 4, 10)),
    part(Rect(28, 20, 4, 12), Rect(8, 17, 4, 10)),
    side(Rect(16, 8, 8, 8), Rect(48, 8, 8, 8), Rect(2, 1, 16, 16)),
    part(Rect(48, 8, 8, 8), Rect(1, 0, 18, 18)),
];

const TOP: &[Part] = &[
    layered(Rect(20, 48, 4, 4), Rect(4, 48, 4, 4), Rect(10, 7, 4, 4)),
    layered(Rect(4, 16, 4, 4), Rect(4, 32, 4, 4), Rect(6, 7, 4, 4)),
    layered(Rect(20, 16, 8, 4), Rect(20, 32, 8, 4), Rect(6, 7, 8, 4)),
    arm(
        Rect(36, 48, 4, 4),
        Rect(52, 48, 4, 4),
        Rect(14, 7, 4, 4),
        Rect(14, 7, 4, 4),
    ),
    arm(
        Rect(44, 16, 4, 4),
        Rect(44, 32, 4, 4),
        Rect(2, 7, 4, 4),
        Rect(2, 7, 4, 4),
    ),
    part(Rect(8, 0, 8, 8), Rect(2, 1, 16, 16)),
    part(Rect(40, 0, 8, 8), Rect(1, 0, 18, 18)),
];

const BOTTOM: &[Part] = &[
    mirrored(part(Rect(16, 0, 8, 8), Rect(2, 1, 16, 16))),
    mirrored(part(Rect(48, 0, 8, 8), Rect(1, 0, 18, 18))),
    mirrored(layered(
        Rect(28, 16, 8, 4),
        Rect(28, 32, 8, 4),
        Rect(6, 7, 8, 4),
    )),
    mirrored(arm(
        Rect(40, 48, 4, 4),
        Rect(56, 48, 4, 4),
        Rect(2, 7, 4, 4),
        Rect(2, 7, 4, 4),
    )),
    mirrored(arm(
        Rect(48, 16, 4, 4),
        Rect(48, 32, 4, 4),
        Rect(14, 7, 4, 4),
        Rect(14, 7, 4, 4),
    )),
    mirrored(part(Rect(24, 48, 4, 4), Rect(6, 7, 4, 4))),
    mirrored(part(Rect(8, 16, 4, 4), Rect(10, 7, 4, 4))),
];

/// Returns the texture mapping associated with a requested camera view.
pub fn mapping(view: View) -> &'static [Part] {
    match view {
        View::Right34 => RIGHT_34,
        View::BackRight34 => BACK_RIGHT_34, //OK
        View::Right => RIGHT,               //OK

        View::Left34 => LEFT_34,          //OK
        View::BackLeft34 => BACK_LEFT_34, //OK
        View::Left => LEFT,               //OK

        View::Front => FRONT,   //OK
        View::Back => BACK,     //OK
        View::Bottom => BOTTOM, //OK
        View::Top => TOP,       //OK
        _ => unreachable!(),
    }
}
