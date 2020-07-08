use rand::{self, Rng};

#[derive(Clone)]
pub struct Gene(pub i32);

#[derive(Clone)]
pub struct Individual {
    pub rank: i32,
    pub genes: Vec<Gene>,
}

#[derive(Clone)]
pub struct Population {
    pub gene_length: usize,
    pub mutate_rate: f64,
    pub elite_rate: f64,
    pub individuals: Vec<Individual>,
}

impl Gene {
    pub fn new() -> Gene {
        let mut rng = rand::thread_rng();
        Gene(rng.gen_range(0, 2))
    }
}

impl Individual {
    pub fn new(gene_length: usize) -> Individual {
        let mut genes = Vec::new();
        for _ in 0..gene_length {
            genes.push(Gene::new());
        }
        let rank = genes.iter().map(|g| g.0).fold(0, |sum, i| sum + i);
        Individual { rank, genes }
    }

    fn fitness(&mut self) {
        self.rank = self.genes.iter().map(|g| g.0).fold(0, |sum, i| sum + i);
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0, self.genes.len());
        self.genes[r] = match self.genes[r].0 {
            1 => Gene(0),
            _ => Gene(1),
        };
        self.fitness();
    }

    pub fn crossover(&mut self, other: &Individual) {
        let mut rng = rand::thread_rng();
        let r1 = rng.gen_range(0, self.genes.len() - 1);
        let r2 = rng.gen_range(r1, self.genes.len() - 1);
        for i in r1..r2 {
            self.genes[i].0 = (&other.genes).clone()[i].0;
        }
        self.fitness();
    }
}

impl Population {
    pub fn new(
        gene_length: usize,
        individual_length: usize,
        mutate_rate: f64,
        elite_rate: f64,
    ) -> Population {
        let mut individuals = Vec::new();
        for _ in 0..individual_length {
            individuals.push(Individual::new(gene_length));
        }
        Population {
            gene_length,
            mutate_rate,
            elite_rate,
            individuals,
        }
    }

    pub fn evaluate(&mut self) {
        self.individuals.sort_by(|a, b| b.rank.cmp(&a.rank))
    }

    pub fn elite(&mut self) {
        self.evaluate();
        let n = ((self.individuals.len() as f64) * self.elite_rate) as usize;
        self.individuals = self.individuals[0..n].to_vec();
    }

    pub fn max(&self) -> &Individual {
        self.individuals.first().unwrap()
    }

    pub fn min(&self) -> &Individual {
        self.individuals.last().unwrap()
    }

    pub fn evolution(&mut self) {
        let mut rng = rand::thread_rng();

        self.evaluate();
        self.elite();

        if self.individuals.len() <= 1 {
            panic!("Gene length that cannot be crossover...");
        }

        while self.individuals.len() < self.gene_length {
            let i = rng.gen_range(0, self.individuals.len() - 1);
            let mut child = self.individuals[i].clone();
            let m: f64 = rng.gen();
            if m <= self.mutate_rate {
                child.mutate();
            } else {
                let i = rng.gen_range(0, self.individuals.len() - 1);
                child.crossover(&self.individuals[i]);
            }
            self.individuals.push(child);
        }
        self.evaluate();
    }
}
