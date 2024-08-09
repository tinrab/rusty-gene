use std::cmp::Ordering;

use crate::chromosome::Chromosome;

pub struct Population<C> {
    chromosomes: Vec<C>,
    min_size: usize,
    max_size: usize,
}

impl<C> Population<C>
where
    C: Chromosome,
{
    pub fn new(mut chromosomes: Vec<C>, min_size: usize, max_size: usize) -> Self {
        assert!(
            min_size <= max_size,
            "min_size must be less than or equal to max_size"
        );
        assert!(!chromosomes.is_empty(), "chromosomes must not be empty");

        chromosomes.reserve(max_size - chromosomes.len() + 1);
        while chromosomes.len() < min_size {
            chromosomes.push(C::create());
        }

        Self {
            chromosomes,
            min_size,
            max_size,
        }
    }

    pub fn set_chromosomes(&mut self, chromosomes: Vec<C>) {
        self.chromosomes = chromosomes;
    }

    pub fn find_best(&self) -> Option<&C> {
        self.chromosomes.iter().max_by(|a, b| {
            a.fitness()
                .partial_cmp(&b.fitness())
                .unwrap_or(Ordering::Equal)
        })
    }

    pub fn get(&self, index: usize) -> Option<&C> {
        self.chromosomes.get(index)
    }

    pub fn chromosomes(&self) -> &[C] {
        &self.chromosomes
    }

    pub fn min_size(&self) -> usize {
        self.min_size
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    pub fn size(&self) -> usize {
        self.chromosomes.len()
    }
}
