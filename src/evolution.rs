use crate::{
    chromosome::Chromosome, crossover::Crossover, insertion::Insertion, population::Population,
    selection::Selection,
};

#[derive(Debug, Clone, PartialEq)]
pub struct EvolutionParameters {
    pub crossover_rate: f64,
    pub mutation_rate: f64,
}

pub struct Evolution<C, S, X, I> {
    population: Population<C>,
    selection: S,
    crossover: X,
    insertion: I,
    parameters: EvolutionParameters,
    children: Vec<C>,
}

impl<C, S, X, I> Evolution<C, S, X, I>
where
    C: Chromosome + Clone,
    S: Selection<C>,
    X: Crossover<C>,
    I: Insertion<C>,
{
    pub fn new(
        population: Population<C>,
        selection: S,
        crossover: X,
        insertion: I,
        parameters: EvolutionParameters,
    ) -> Self {
        Self {
            population,
            selection,
            crossover,
            insertion,
            parameters,
            children: vec![],
        }
    }

    pub fn begin_generation(&mut self) {}

    fn select(&self) -> Vec<C> {
        self.selection.select(
            self.population.chromosomes().to_vec(),
            self.population.min_size(),
        )
    }

    fn mutate(&self)
}

// #[derive(Clone, PartialEq)]
// pub struct Gene<T> {
//     value: T,
// }

// #[derive(Clone, PartialEq)]
// pub struct Chromosome<T> {
//     genes: Vec<Gene<T>>,
// }

// fn demo_select<I, C, G>(c: I)
// where
//     I: IntoIterator<Item = C>,
//     C: Chromosome<Gene = G>,
//     G: Clone,
// {
//     println!("c: {}", c.into_iter().count());
// }

#[cfg(test)]
mod tests {
    use crate::{insertion::elite::EliteInsertion, selection::elite::EliteSelection};

    use super::*;

    #[test]
    fn it_works() {
        // #[derive(Clone, PartialEq)]
        // struct XorGene {}

        // let g: Box<dyn Gene> = Box::new(XorGene {});

        #[derive(Clone, PartialEq)]
        struct BingoChromosome {
            genes: Vec<BingoGene>,
        }

        type BingoGene = i32;

        impl Chromosome for BingoChromosome {
            type Gene = BingoGene;

            fn create() -> Self {
                BingoChromosome { genes: vec![] }
            }

            fn genes(&self) -> Vec<Self::Gene> {
                vec![]
            }

            fn fitness(&self) -> f64 {
                0.0
            }
        }

        struct BingoCrossover {}

        impl<C> Crossover<C> for BingoCrossover
        where
            C: Chromosome,
        {
            fn parent_count(&self) -> usize {
                2
            }

            fn crossover(&self, parents: Vec<C>) -> C {
                todo!()
            }
        }

        // let c: Box<dyn Chromosome<Gene = i32>> = Box::new(BingoChromosome {});

        // demo_select(vec![c]);

        let mut evolution = Evolution::new(
            Population::new(vec![BingoChromosome::create()], 5, 100),
            EliteSelection::new(),
            BingoCrossover {},
            EliteInsertion::new(),
            EvolutionParameters {
                crossover_rate: 0.8f64,
                mutation_rate: 0.8f64,
            },
        );
        evolution.begin();
    }
}
