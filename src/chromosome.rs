use std::cmp::Ordering;

pub trait Chromosome {
    type Gene: Clone;

    fn create() -> Self;

    fn genes(&self) -> Vec<Self::Gene>;

    fn fitness(&self) -> f64;
}

pub fn sort_chromosomes(chromosomes: &mut Vec<impl Chromosome>) {
    chromosomes.sort_by(|a, b| {
        a.fitness()
            .partial_cmp(&b.fitness())
            .unwrap_or(Ordering::Equal)
    });
}
