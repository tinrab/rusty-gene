use crate::{chromosome::Chromosome, population::Population};

pub mod elite;

pub trait Insertion<C>
where
    C: Chromosome,
{
    fn select(&self, population: &Population<C>, parents: Vec<C>, children: Vec<C>) -> Vec<C>;
}
