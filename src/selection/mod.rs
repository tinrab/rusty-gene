use crate::chromosome::Chromosome;

pub mod elite;

pub trait Selection<C>
where
    C: Chromosome,
{
    fn select(&self, chromosomes: Vec<C>, count: usize) -> Vec<C>;
}
