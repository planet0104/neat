#[cfg(all(target_arch="wasm32", target_os="unknown"))]
#[macro_use]
extern crate stdweb;
use serde::{Serialize, Deserialize};

pub mod ga;
pub mod genes;
pub mod params;
pub mod phenotype;
pub mod species;
pub mod utils;
pub use bincode;
use std::time::Instant;

fn elapsed_ms(tag: &str, t:&mut Instant){
    println!("{} 耗时{}ms", tag, t.elapsed().as_millis());
    *t = Instant::now();
}

fn reset_timer(t:&mut Instant){
    *t = Instant::now();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    /// default 0.2
    mutation_rate: f32,
    /// default 0.7
    crossover_rate: f32,
    // probability_weight_replaced: f32,
    // max_weight_perturbation: f64,
    // activation_mutation_rate: f32,
    // max_activation_perturbation: f32,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            mutation_rate: params::MUTATION_RATE,
            crossover_rate: params::CROSSOVER_RATE,
            // probability_weight_replaced: params::PROBABILITY_WEIGHT_REPLACED,
            // max_weight_perturbation: params::MAX_WEIGHT_PERTURBATION,
            // activation_mutation_rate: params::ACTIVATION_MUTATION_RATE,
            // max_activation_perturbation: params::MAX_ACTIVATION_PERTURBATION,
        }
    }
}

/// cargo test --release -- --nocapture
#[test]
fn test_xor() {
    use phenotype::RunType;
    let pop_size = 150;
    let mut ga = ga::GA::new(pop_size, 2, 1);
    let mut max_fitness = 0.0;
    let mut count = 0;
    while max_fitness < 15.9f64 {
        for p in 0..pop_size {
            let phenotype = ga.get_phenotype(p as usize);
            let mut distance: f64;

            let output = phenotype.update(&vec![0.0, 0.0], RunType::Active);
            distance = (0.0 - output[0]).abs();
            let output = phenotype.update(&vec![0.0, 1.0], RunType::Active);
            distance += (1.0 - output[0]).abs();
            let output = phenotype.update(&vec![1.0, 0.0], RunType::Active);
            distance += (1.0 - output[0]).abs();
            let output = phenotype.update(&vec![1.0, 1.0], RunType::Active);
            distance += (0.0 - output[0]).abs();

            let fitness = (4.0 - distance).powi(2);
            ga.fitness_scores()[p as usize] = fitness;
            if fitness > max_fitness {
                max_fitness = fitness;
            }
        }
        if count % 500 == 0 {
            println!("max_fitness={}", max_fitness);
        }
        ga.epoch();
        count += 1;
    }

    let brains: Vec<usize> = ga.get_best_phenotypes_from_last_generation();
    let brain = ga.get_phenotype(brains[0]);
    println!("generation:{}", count);
    println!("{:?}", brain.update(&[0.0, 0.0], RunType::Active));
    println!("{:?}", brain.update(&[0.0, 1.0], RunType::Active));
    println!("{:?}", brain.update(&[1.0, 0.0], RunType::Active));
    println!("{:?}", brain.update(&[1.0, 1.0], RunType::Active));
}
