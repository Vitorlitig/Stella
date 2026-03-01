// src/math/chaos.rs

pub struct HenonMap {
    x: f64,
    y: f64,
    a: f64,
    b: f64,
}

impl HenonMap {
    /// Initializes a new Hénon Map with classical chaotic parameters (a=1.4, b=0.3).
    /// Runs a 10,000 cycle warm-up to ensure the state is fully immersed in the strange attractor.
    pub fn new(seed_x: f64, seed_y: f64) -> Self {
        let mut map = Self {
            x: seed_x,
            y: seed_y,
            a: 1.4,
            b: 0.3,
        };

        // Warm-up phase: discard the initial transient trajectory
        for _ in 0..10_000 {
            map.step();
        }

        map
    }

    /// Advances the chaotic system by one discrete time step.
    fn step(&mut self) {
        let next_x = 1.0 - self.a * self.x * self.x + self.y;
        let next_y = self.b * self.x;
        self.x = next_x;
        self.y = next_y;
    }

    /// Extracts entropy from the chaotic state to generate a pseudo-random usize.
    pub fn next_usize(&mut self) -> usize {
        self.step();

        // The x coordinate bounds are roughly [-1.5, 1.5].
        // We multiply by a large scalar and extract the fractional entropy.
        let entropy = (self.x.abs() * 1_000_000_000.0).fract() * 1_000_000_000.0;

        entropy as usize
    }
}
