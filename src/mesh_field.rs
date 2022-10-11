use crate::particle::Particle;
use crate::line::Line;
use crate::math::Vector2D;

pub struct MeshField {
    particles: Vec<Particle>,
    lines: Vec<Line>,
    max_line_len: f32,
    size: Vector2D,
}

impl MeshField {
    pub fn new(particle_count: usize, size: Vector2D, max_line_len: f32) -> Self {
        let particles = (0..particle_count)
            .map(|_| Particle::new_random(size))
            .collect();

        Self {
            particles,
            lines: vec![],
            max_line_len,
            size,
        }
    }

    pub fn particles(&self) -> Vec<Particle> {
        self.particles.clone()
    }

    pub fn lines(&self) -> Vec<Line> {
        self.lines.clone()
    }

    pub fn update_all(&mut self) {
        self.update_particles();
        self.lines = Self::connections(self.particles.clone(), self.max_line_len)
    }

    fn update_particles(&mut self) {
        for h in 0..self.particles.len() {
            self.particles[h].field_size = self.size;
            self.particles[h].update();
        }
    }

    pub fn connections(particles: Vec<Particle>, max_line_len: f32) -> Vec<Line> {
        let mut lines = vec![];

        for h in 0..particles.len() {
            for k in 0..particles.len() {
                if h != k {
                    let ph = particles[h].position;
                    let pk = particles[k].position;
                    let line = Line::new(ph, pk);

                    if line.len() <= max_line_len {
                        lines.push(line)
                    }
                }
            }
        }

        lines
    }

    pub fn update_screen_size(&mut self, size: Vector2D) {
        self.size = size;
    }
}