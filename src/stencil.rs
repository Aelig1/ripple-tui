#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Stencil {
    /// Uses the four orthogonal neighbors equally.
    VonNeumann,
    /// Uses all eight surrounding neighbors equally.
    Moore,
    /// Uses an isotropic Laplacian, weighting orthogonal neighbors more heavily.
    #[default]
    Laplace,
}

impl Stencil {
    pub fn taps(self) -> &'static [Tap] {
        match self {
            Self::VonNeumann => &VON_NEUMANN,
            Self::Moore => &MOORE,
            Self::Laplace => &LAPLACE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tap {
    pub dx: isize,
    pub dy: isize,
    pub weight: f64,
}

const VON_NEUMANN: [Tap; 4] = [
    Tap {
        dx: 0,
        dy: -1,
        weight: 0.5,
    },
    Tap {
        dx: -1,
        dy: 0,
        weight: 0.5,
    },
    Tap {
        dx: 0,
        dy: 1,
        weight: 0.5,
    },
    Tap {
        dx: 1,
        dy: 0,
        weight: 0.5,
    },
];

const MOORE: [Tap; 8] = [
    Tap {
        dx: -1,
        dy: -1,
        weight: 0.25,
    },
    Tap {
        dx: 0,
        dy: -1,
        weight: 0.25,
    },
    Tap {
        dx: 1,
        dy: -1,
        weight: 0.25,
    },
    Tap {
        dx: -1,
        dy: 0,
        weight: 0.25,
    },
    Tap {
        dx: 1,
        dy: 0,
        weight: 0.25,
    },
    Tap {
        dx: -1,
        dy: 1,
        weight: 0.25,
    },
    Tap {
        dx: 0,
        dy: 1,
        weight: 0.25,
    },
    Tap {
        dx: 1,
        dy: 1,
        weight: 0.25,
    },
];

const LAPLACE: [Tap; 8] = [
    Tap {
        dx: -1,
        dy: -1,
        weight: 0.1,
    },
    Tap {
        dx: 0,
        dy: -1,
        weight: 0.4,
    },
    Tap {
        dx: 1,
        dy: -1,
        weight: 0.1,
    },
    Tap {
        dx: -1,
        dy: 0,
        weight: 0.4,
    },
    Tap {
        dx: 1,
        dy: 0,
        weight: 0.4,
    },
    Tap {
        dx: -1,
        dy: 1,
        weight: 0.1,
    },
    Tap {
        dx: 0,
        dy: 1,
        weight: 0.4,
    },
    Tap {
        dx: 1,
        dy: 1,
        weight: 0.1,
    },
];
