// use criterion::{
//     criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
// };
// use crypto::dbs::*;
// use rand::{rngs::StdRng, SeedableRng};

// const SEED: u64 = 42;
// static TEST_POINTS: [usize; 7] = [3, 10, 20, 30, 50, 75, 100];
// const BENCH_COUNT: usize = 10;

// pub fn fsbp_generation(c: &mut Criterion) {
//     let mut group = c.benchmark_group("fsbp_generation");
//     BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
//     for &n in &TEST_POINTS {
//         let rng = &mut StdRng::seed_from_u64(SEED);
//         let t = (n + 1) / 2;
//         group.throughput(Throughput::Bytes(n as u64));
//         group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
//             b.iter(|| {
//                 let keys: Vec<_> =
//                     (0..n).map(|_| generate_keypair(rng)).collect();
//                 let public_keys: Vec<PublicKey> = (0..n).map(|i| keys[i].1).collect();
//                 generate_shares(n, t, &public_keys, rng)
//             });
//         });
//     }
//     group.finish();
// }

// pub fn fsbp_aggregation(c: &mut Criterion) {
//     let mut group = c.benchmark_group("fsbp_aggregation");
//     BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
//     for &n in &TEST_POINTS {
//         let rng = &mut StdRng::seed_from_u64(SEED);
//         let t = (n + 1) / 2;
//         let keys: Vec<_> = (0..n).map(|_| generate_keypair(rng)).collect();
//         let public_keys: Vec<PublicKey> = (0..n).map(|i| keys[i].1).collect();
//         let generated: Vec<_> = (0..t)
//             .map(|_| generate_shares(n, t, &public_keys, rng))
//             .collect();
//         group.throughput(Throughput::Bytes(n as u64));
//         group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
//             b.iter(|| {
//                 aggregate(
//                     n,
//                     t,
//                     &generated.iter().map(|g| g.1.clone()).collect::<Vec<_>>(),
//                     &generated.iter().map(|g| g.2.clone()).collect::<Vec<_>>(),
//                     &generated.iter().map(|g| g.3.clone()).collect::<Vec<_>>(),
//                 )
//             });
//         });
//     }
//     group.finish();
// }

// pub fn fsbp_proof_verification(c: &mut Criterion) {
//     let mut group = c.benchmark_group("fsbp_proof_verification");
//     BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
//     for &n in &TEST_POINTS {
//         let rng = &mut StdRng::seed_from_u64(SEED);
//         let t = (n + 1) / 2;
//         let keys: Vec<_> = (0..n).map(|_| generate_keypair(rng)).collect();
//         let public_keys: Vec<PublicKey> = (0..n).map(|i| keys[i].1).collect();
//         let generated: Vec<_> = (0..t)
//             .map(|_| generate_shares(n, t, &public_keys, rng))
//             .collect();
//         let (shares, commitments, proof) = aggregate(
//             n,
//             t,
//             &generated.iter().map(|g| g.1.clone()).collect::<Vec<_>>(),
//             &generated.iter().map(|g| g.2.clone()).collect::<Vec<_>>(),
//             &generated.iter().map(|g| g.3.clone()).collect::<Vec<_>>(),
//         );
//         let private_commitments: Vec<_> = generated.iter().map(|g| g.2[0]).collect();
//         group.throughput(Throughput::Bytes(n as u64));
//         group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
//             b.iter(|| {
//                 verify(
//                     n,
//                     t,
//                     &public_keys,
//                     &shares,
//                     &commitments,
//                     &private_commitments,
//                     &proof[0],
//                     rng,
//                 )
//             });
//         });
//     }
//     group.finish();
// }

// pub fn fsbp_share_verification(c: &mut Criterion) {
//     let mut group = c.benchmark_group("fsbp_share_verification");
//     BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
//     for &n in &TEST_POINTS {
//         let rng = &mut StdRng::seed_from_u64(SEED);
//         let t = (n + 1) / 2;
//         let keys: Vec<_> = (0..n).map(|_| generate_keypair(rng)).collect();
//         let public_keys: Vec<PublicKey> = (0..n).map(|i| keys[i].1).collect();
//         let (_, shares, commitments, _) = generate_shares(n, t, &public_keys, rng);
//         let decrypted_shares: Vec<Share> = (0..n)
//             .map(|i| decrypt_share(keys[i].0, shares[i]))
//             .collect();
//         group.throughput(Throughput::Bytes(n as u64));
//         group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
//             b.iter(|| {
//                 (0..n)
//                     .map(|i| verify_share(decrypted_shares[i], commitments[i]))
//                     .collect::<Vec<_>>()
//             });
//         });
//     }
//     group.finish();
// }

// pub fn fsbp_reconstruction(c: &mut Criterion) {
//     let mut group = c.benchmark_group("fsbp_reconstruction");
//     BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
//     for &n in &TEST_POINTS {
//         let rng = &mut StdRng::seed_from_u64(SEED);
//         let t = (n + 1) / 2;
//         let keys: Vec<_> = (0..n).map(|_| generate_keypair(rng)).collect();
//         let public_keys: Vec<PublicKey> = (0..n).map(|i| keys[i].1).collect();
//         let (_, shares, _, _) = generate_shares(n, t, &public_keys, rng);
//         let decrypted_shares: Vec<_> = (0..n)
//             .map(|i| Some(decrypt_share(keys[i].0, shares[i])))
//             .collect();
//         group.throughput(Throughput::Bytes(n as u64));
//         group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
//             b.iter(|| reconstruct(n, &decrypted_shares));
//         });
//     }
//     group.finish();
// }

// criterion_group!(
//     name = benches;
//     config = Criterion::default().sample_size(BENCH_COUNT);
//     targets = fsbp_generation, fsbp_aggregation, fsbp_proof_verification, fsbp_share_verification, fsbp_reconstruction);
// criterion_main!(benches);