use crate::{
    chromosome::{sort_chromosomes, Chromosome},
    selection::Selection,
};

pub struct EliteSelection {}

impl EliteSelection {
    pub fn new() -> Self {
        Self {}
    }
}

impl<C> Selection<C> for EliteSelection
where
    C: Chromosome,
{
    fn select(&self, mut chromosomes: Vec<C>, count: usize) -> Vec<C> {
        sort_chromosomes(&mut chromosomes);
        chromosomes.truncate(count);
        chromosomes
    }
}
