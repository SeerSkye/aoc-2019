pub fn day_12() {
    let io = Moon {
        position: (5, -1, 5),
        velocity: (0, 0, 0),
    };
    let europa = Moon {
        position: (0, -14, 2),
        velocity: (0, 0, 0),
    };
    let ganymede = Moon {
        position: (16, 4, 0),
        velocity: (0, 0, 0),
    };
    let callisto = Moon {
        position: (18, 1, 16),
        velocity: (0, 0, 0),
    };

    let mut moons = (io, europa, ganymede, callisto);

    Moon::run_timesteps(&mut moons, 1000);

    let total_energy = moons.0.get_total_energy()
        + moons.1.get_total_energy()
        + moons.2.get_total_energy()
        + moons.3.get_total_energy();

    println!("The total energy after 1000 steps is: {}", total_energy);

    let mut moons = (io, europa, ganymede, callisto);

    let initial_state = moons;

    let mut x_cycle_len = 0;
    let mut y_cycle_len = 0;
    let mut z_cycle_len = 0;
    let mut count: i64 = 0;

    while x_cycle_len == 0 || y_cycle_len == 0 || z_cycle_len == 0 {
        Moon::run_timesteps(&mut moons, 1);
        count += 1;

        if x_cycle_len == 0
            && moons.0.velocity.0 == initial_state.0.velocity.0
            && moons.1.velocity.0 == initial_state.1.velocity.0
            && moons.2.velocity.0 == initial_state.2.velocity.0
            && moons.3.velocity.0 == initial_state.3.velocity.0
            && moons.0.position.0 == initial_state.0.position.0
        {
            x_cycle_len = count;
        }
        if y_cycle_len == 0
            && moons.0.velocity.1 == initial_state.0.velocity.1
            && moons.1.velocity.1 == initial_state.1.velocity.1
            && moons.2.velocity.1 == initial_state.2.velocity.1
            && moons.3.velocity.1 == initial_state.3.velocity.1
            && moons.0.position.1 == initial_state.0.position.1
        {
            y_cycle_len = count;
        }
        if z_cycle_len == 0
            && moons.0.velocity.2 == initial_state.0.velocity.2
            && moons.1.velocity.2 == initial_state.1.velocity.2
            && moons.2.velocity.2 == initial_state.2.velocity.2
            && moons.3.velocity.2 == initial_state.3.velocity.2
            && moons.0.position.2 == initial_state.0.position.2
        {
            z_cycle_len = count;
        }
    }

    println!("The number of steps till a duplicate state is: {}", (lcm(lcm(x_cycle_len, y_cycle_len), z_cycle_len)));
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}

impl Moon {
    fn run_timesteps(moons: &mut (Moon, Moon, Moon, Moon), num_steps: usize) {
        for _ in 0..num_steps {
            //simulate gravity on each pair
            moons.0.apply_gravity(&mut moons.1);
            moons.0.apply_gravity(&mut moons.2);
            moons.0.apply_gravity(&mut moons.3);
            moons.1.apply_gravity(&mut moons.2);
            moons.1.apply_gravity(&mut moons.3);
            moons.2.apply_gravity(&mut moons.3);

            //then move
            moons.0.move_step();
            moons.1.move_step();
            moons.2.move_step();
            moons.3.move_step();
        }
    }

    fn move_step(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn apply_gravity(&mut self, other: &mut Moon) {
        if self.position.0 < other.position.0 {
            self.velocity.0 += 1;
            other.velocity.0 -= 1;
        } else if other.position.0 < self.position.0 {
            other.velocity.0 += 1;
            self.velocity.0 -= 1;
        }

        if self.position.1 < other.position.1 {
            self.velocity.1 += 1;
            other.velocity.1 -= 1;
        } else if other.position.1 < self.position.1 {
            other.velocity.1 += 1;
            self.velocity.1 -= 1;
        }

        if self.position.2 < other.position.2 {
            self.velocity.2 += 1;
            other.velocity.2 -= 1;
        } else if other.position.2 < self.position.2 {
            other.velocity.2 += 1;
            self.velocity.2 -= 1;
        }
    }

    fn get_total_energy(&self) -> i32 {
        let pot_energy = self.position.0.abs() + self.position.1.abs() + self.position.2.abs();
        let kin_energy = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();
        pot_energy * kin_energy
    }
}

fn gcd(in_1: i64, in_2: i64) -> i64 {
    let mut a = in_1.max(in_2);
    let mut b = in_1.min(in_2);

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a.abs()
}

fn lcm(in_1: i64, in_2: i64) -> i64 {
    (in_1 * in_2).abs() / gcd(in_1, in_2)
}
