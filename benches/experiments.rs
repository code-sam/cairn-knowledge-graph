// use criterion::{criterion_group, criterion_main, Criterion};
// use rand::distributions::{Distribution, Uniform};
// use rand::Rng;

// fn performance_experiments(c: &mut Criterion) {
//     c.bench_function("vector_indexing", |b| b.iter(|| bench_vector_indexing()));
//     c.bench_function("vector_abstract_indexing", |b| {
//         b.iter(|| bench_vector_indexing_with_abstracted_index())
//     });
// }

// criterion_group!(benches, performance_experiments);
// criterion_main!(benches);

// fn bench_vector_indexing() {
//     let number_of_data_points = 100000;
//     let mut data: Vec<usize> = Vec::with_capacity(number_of_data_points);
//     for i in 0..number_of_data_points {
//         data.push(i);
//     }

//     let number_of_samples = 1000000;
//     let random_distribution = Uniform::from(0..number_of_data_points);
//     let mut rng = rand::thread_rng();
//     let mut indices: Vec<usize> = Vec::with_capacity(number_of_samples);
//     for _i in 0..number_of_samples {
//         indices.push(random_distribution.sample(&mut rng))
//     }

//     let mut rng = rand::thread_rng();
//     for i in indices {
//         let value = Some(&data[i]);
//         // println!("{}",value)
//     }
// }

// struct AbstractIndex {
//     index: usize,
// }

// impl AbstractIndex {
//     fn new(index: usize) -> Self {
//         Self { index }
//     }
//     fn index_ref(&self) -> &usize {
//         &self.index
//     }
// }

// fn bench_vector_indexing_with_abstracted_index() {
//     let number_of_data_points = 100000;
//     let mut data: Vec<usize> = Vec::with_capacity(number_of_data_points);
//     for i in 0..number_of_data_points {
//         data.push(i);
//     }

//     let number_of_samples = 1000000;
//     let random_distribution = Uniform::from(0..number_of_data_points);
//     let mut rng = rand::thread_rng();
//     let mut indices: Vec<AbstractIndex> = Vec::with_capacity(number_of_samples);
//     for _i in 0..number_of_samples {
//         indices.push(AbstractIndex::new(random_distribution.sample(&mut rng)))
//     }

//     for i in indices {
//         let value = Some(&data[i.index_ref().clone()]);
//         // println!("{}",value)
//     }
// }
