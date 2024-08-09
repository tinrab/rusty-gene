use crate::chromosome::Chromosome;

pub trait Crossover<C>
where
    C: Chromosome,
{
    fn parent_count(&self) -> usize;

    fn crossover(&self, parents: Vec<C>) -> C;
}
