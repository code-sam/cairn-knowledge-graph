// // use hashbrown::HashMap;
// use std::collections::VecDeque;

// use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
// use rand::distributions::{Distribution, Uniform};
// use rand::Rng;
// use rustc_hash::{FxHashMap, FxHashSet, FxHasher};

// use nohash_hasher::NoHashHasher;
// // use std::hash::BuildHasherDefault;
// use std::{collections::HashMap, hash::BuildHasherDefault};

// use graphblas_sparse_linear_algebra::context::{Context, Mode};
// use graphblas_sparse_linear_algebra::value_types::sparse_vector::{
//     GetVectorElementValue, SetVectorElement, SparseVector,
// };

// // use graph_computing::util::indexed_data_store::IndexedDataStore;

// fn sparse_vector_indexing_benchmark(c: &mut Criterion) {
//     c.bench_function("vector", |b| b.iter(|| bench_vector()));

//     c.bench_function("struct_with_vector", |b| {
//         b.iter(|| bench_struct_with_vector())
//     });

//     c.bench_function("hashmap", |b| b.iter(|| bench_hashmap()));

//     c.bench_function("no_hash_hashmap", |b| b.iter(|| bench_no_hash_hashmap()));

//     let context = Context::init_ready(Mode::NonBlocking).unwrap();
//     let mut data = SparseVector::<i32>::new(&context, &(100000 * 10)).unwrap();
//     for i in 0..100000 {
//         data.set_element((i * 10 as usize, (i * 10) as i32).into())
//             .unwrap();
//     }
//     c.bench_with_input(
//         BenchmarkId::new("sparse_vector_lookup", data.clone()),
//         &data,
//         |b, data| b.iter(|| bench_sparse_vector_lookup(data.clone())),
//     );

//     // c.bench_function("bench_indexed_data_store", |b| {
//     //     b.iter(|| bench_indexed_data_store())
//     // });
// }

// criterion_group!(benches, sparse_vector_indexing_benchmark);
// criterion_main!(benches);

// fn bench_vector() {
//     let mut data: Vec<i32> = Vec::new();
//     for i in 0..100000 {
//         data.push(i);
//     }

//     let random_distribution = Uniform::from(0..data.len());
//     let mut rng = rand::thread_rng();
//     for _i in 0..100000 {
//         let value = Some(&data[random_distribution.sample(&mut rng)]);
//         // println!("{}",value)
//     }
// }
// // fn bench_vector() {
// //     let mut data: Vec<String> = Vec::new();
// //     for i in 0..100000 {
// //         data.push(String::from("test"));
// //     }

// //     let random_distribution = Uniform::from(0..data.len());
// //     let mut rng = rand::thread_rng();
// //     for _i in 0..100000 {
// //         let value = &data[random_distribution.sample(&mut rng)];
// //         // println!("{}",value)
// //     }
// // }

// struct StructWithVector<T> {
//     data: Vec<T>,
// }

// impl<T> StructWithVector<T> {
//     fn get(&self, index: usize) -> Option<&T> {
//         Some(&self.data[index])
//     }
// }

// fn bench_struct_with_vector() {
//     let mut data = StructWithVector {
//         data: Vec::<i32>::new(),
//     };
//     for i in 0..100000 {
//         data.data.push(i);
//     }

//     let random_distribution = Uniform::from(0..data.data.len());
//     let mut rng = rand::thread_rng();
//     for _i in 0..100000 {
//         let value = data.get(random_distribution.sample(&mut rng));
//         // println!("{}",value)
//     }
// }

// fn bench_hashmap() {
//     let mut data: FxHashMap<usize, i32> = FxHashMap::default();
//     for i in 0..100000 {
//         data.insert(i, i as i32);
//     }

//     let random_distribution = Uniform::from(0..data.len());
//     let mut rng = rand::thread_rng();
//     for _i in 0..100000 {
//         let value = data.get(&random_distribution.sample(&mut rng)).unwrap();
//         // println!("{}",value)
//     }
// }
// // fn bench_hashmap() {
// //     let mut data: FxHashMap<usize, String> = FxHashMap::default();
// //     for i in 0..100000 {
// //         data.insert(i,String::from("test"));
// //     }

// //     let random_distribution = Uniform::from(0..data.len());
// //     let mut rng = rand::thread_rng();
// //     for _i in 0..100000 {
// //         let value = data.get(&random_distribution.sample(&mut rng)).unwrap();
// //         // println!("{}",value)
// //     }
// // }

// fn bench_no_hash_hashmap() {
//     let mut data: HashMap<usize, i32, BuildHasherDefault<NoHashHasher<i32>>> =
//         HashMap::with_hasher(BuildHasherDefault::default());

//     for i in 0..100000 {
//         data.insert(i, i as i32);
//     }

//     let random_distribution = Uniform::from(0..data.len());
//     let mut rng = rand::thread_rng();
//     for _i in 0..100000 {
//         let value = data.get(&random_distribution.sample(&mut rng)).unwrap();
//         // println!("{}",value)
//     }
// }

// fn bench_sparse_vector_lookup(data: SparseVector<i32>) {
//     // let context = Context::init_ready(Mode::NonBlocking).unwrap();

//     // let mut data = SparseVector::<i32>::new(&context, &(100000 * 10)).unwrap();

//     // for i in 0..100000 {
//     //     data.set_element((i * 10 as usize, (i * 10) as i32).into())
//     //         .unwrap();
//     // }

//     let random_distribution = Uniform::from(0..data.number_of_stored_elements().unwrap());
//     let mut rng = rand::thread_rng();
//     for _i in 0..100000 {
//         let value = data
//             .get_element_value(&(10 * random_distribution.sample(&mut rng)))
//             .unwrap();
//         // println!("{}",value)
//     }
// }

// // fn bench_indexed_data_store() {
// //     let mut data = IndexedDataStore::<i32>::with_capacity(10);

// //     for i in 0..100000 {
// //         data.push(i);
// //     }

// //     let random_distribution = Uniform::from(0..10000);
// //     let mut rng = rand::thread_rng();
// //     for _i in 0..100000 {
// //         let value = data.get(random_distribution.sample(&mut rng));
// //         // println!("{}",value)
// //     }
// // }
