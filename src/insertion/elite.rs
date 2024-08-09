use crate::{
    chromosome::{sort_chromosomes, Chromosome},
    insertion::Insertion,
    population::Population,
};

pub struct EliteInsertion {}

impl EliteInsertion {
    pub fn new() -> Self {
        Self {}
    }
}

impl<C> Insertion<C> for EliteInsertion
where
    C: Chromosome,
{
    fn select(
        &self,
        population: &Population<C>,
        mut parents: Vec<C>,
        mut children: Vec<C>,
    ) -> Vec<C> {
        if children.len() < population.min_size() {
            sort_chromosomes(&mut parents);

            for _ in 0..(population.min_size() - children.len()) {
                if let Some(parent) = parents.pop() {
                    children.push(parent);
                }
            }
        }
        children
    }
}
