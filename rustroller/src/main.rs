// use rand::Rng;
use rand::prelude::*;
use rand_pcg::Pcg64;

fn main() {
    let mut rng = Pcg64::seed_from_u64(0);
    let mut d = Dice::new(&mut rng);
    d.roll();
    d.roll();
    d.roll();
    println!("{:?}", d.roll_values);
    println!("{}", d.check_if_rolled(vec![2, 1]));
    println!("{}", d.check_if_rolled(vec![1, 2, 5]));
    println!("{}", d.check_if_rolled(vec![1, 2, 3, 4, 5, 6]));

    let (_, average_rolls) = simulate_rolling_every_number(500_000, 0);
    println!("Average number of trys {}", average_rolls);
}

pub fn simulate_rolling_every_number(n_trys: usize, seed: u64) -> (Vec<usize>, f64) {
    let mut trys = Vec::<usize>::new();
    let mut rng = Pcg64::seed_from_u64(seed);
    for _ in 0..=n_trys {
        let mut d = Dice::new(&mut rng);
        while !d.check_if_rolled((1..=6).collect()) {
            d.roll();
        }
        trys.push(d.roll_values.len());
    }
    let count = (trys.iter().sum::<usize>()) as f64;
    let mean: f64 = count / (trys.len() as f64);
    (trys, mean)
}

pub struct Dice<'a> {
    roll_values: Vec<i8>,
    rng: &'a mut rand_pcg::Pcg64,
}

impl<'a> Dice<'a> {
    pub fn new(rng: &'a mut rand_pcg::Pcg64) -> Self {
        Dice {
            roll_values: Vec::new(),
            rng,
        }
    }
    pub fn roll(&mut self) -> i8 {
        let num = self.rng.gen_range(1..7);
        self.roll_values.push(num);
        num
    }

    pub fn check_if_rolled(&self, values: Vec<i8>) -> bool {
        values.iter().all(|item| self.roll_values.contains(item))
    }
}
