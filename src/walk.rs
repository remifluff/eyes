use chrono::offset;
use nannou::{
    lyon::algorithms::walk,
    rand::{random, random_range, SeedableRng},
};
use randomwalk::generators::NormalGenerator;

pub struct Walk {
    current_time: f64,
    current_value: f64,
    normal_gen: NormalGenerator,
    offset: f64,
}
impl Walk {
    pub fn new(salt: u64) -> Walk {
        let norm_dist = rand_distr::Normal::new(0.0, 1.0).unwrap();

        // exponential distribution with lambda of 0.0055
        let exp_dist = rand_distr::Exp::new(1.0 / 180.0).unwrap();

        // uniform distribution between 0 and 1
        let unif_dist = rand_distr::Uniform::new(0.0, 1.0);

        let now = || chrono::Utc::now();

        let mut rng = rand_hc::Hc128Rng::from_entropy();

        let utc_now = now();
        let mut normal_gen = NormalGenerator::new(
            0.0,
            55001.0,
            0.05,
            utc_now.timestamp() as u64 + salt,
        );

        let current_time = utc_now.timestamp_millis() as f64;
        let current_value = normal_gen.next(current_time).unwrap();

        Walk {
            normal_gen,
            current_time,
            current_value,
            offset: 0.0,
        }
    }

    pub fn update(&mut self) {
        // # Setup a random walk with mean 100.0 and variance 250.0.
        // # The variance controls how tightly the walk is held
        // # close to its mean and sigma_xx controls how rapidly
        // # it can wander away from its starting point.

        // for i in 1..10 {
        //     // # pretend time lapsed

        if random_range(0, 250) < 1 {
            self.offset = random_range(-1000000.0, 1000000.0);
        }

        self.current_time =
            (chrono::Utc::now().timestamp_millis() as f64) + self.offset;
        self.current_value =
            self.normal_gen.next(self.current_time).unwrap();

        //     println!(
        //         "Normal walk value {} time {}",
        //         current_value, current_time,
    }
    pub fn val(&self) -> f32 {
        self.current_value as f32
    }
}
