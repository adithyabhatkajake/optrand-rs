use std::collections::HashMap;

use criterion::{
    criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use crypto::*;
use rand::{rngs::StdRng, SeedableRng};

const SEED: u64 = 42;
static TEST_POINTS: [usize; 7] = [3, 10, 20, 30, 50, 75, 100];
const BENCH_COUNT: usize = 10;

pub fn pvss_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("pvss_generation");
    BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
    for &n in &TEST_POINTS {
        
        // Start prepping for your test
        let mut rng = &mut StdRng::seed_from_u64(SEED);
        let t = (n - 1) / 2;
        
        let mut public_keys = Vec::new();
        let mut secret_keys = Vec::new();
        let mut dss_kpair = Vec::new();
        let mut dss_pk = Vec::new();
        for _i in 0..n {
            let kpair = Keypair::generate_keypair(&mut rng);
            secret_keys.push(kpair.0);
            public_keys.push(kpair.1);
            let dsskpair = crypto_lib::Keypair::generate_ed25519();
            dss_pk.push(dsskpair.public());
            dss_kpair.push(dsskpair);
        }
        let h2 = G2::prime_subgroup_generator().mul(Scalar::rand(&mut rng));

        let dbs_ctx = DbsContext::new(&mut rng, h2,n, t, 0, public_keys, secret_keys[0]);

        // We are ready to start testing now
        group.throughput(Throughput::Bytes(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_n| {
            b.iter(|| {
                // Insert code that you want tested here
                dbs_ctx.generate_shares(&dss_kpair[0],&mut rng)
            });
        });
    }
    group.finish();
}

pub fn pvss_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("pvss_verification");
    BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
    for &n in &TEST_POINTS {
        
        // Start prepping for your test
        let mut rng = &mut StdRng::seed_from_u64(SEED);
        let t = (n - 1) / 2;
        
        let mut public_keys = Vec::new();
        let mut secret_keys = Vec::new();
        let mut dss_kpair = Vec::new();
        let mut dss_pk = Vec::new();
        for _i in 0..n {
            let kpair = Keypair::generate_keypair(&mut rng);
            secret_keys.push(kpair.0);
            public_keys.push(kpair.1);
            let dsskpair = crypto_lib::Keypair::generate_ed25519();
            dss_pk.push(dsskpair.public());
            dss_kpair.push(dsskpair);
        }
        let h2 = G2::prime_subgroup_generator().mul(Scalar::rand(&mut rng));

        let dbs_ctx = DbsContext::new(&mut rng, h2,n, t, 0, public_keys, secret_keys[0]);
        let idx = 0;
        let (v,c,pi) = 
            dbs_ctx.generate_shares(&dss_kpair[idx], &mut rng);

        // We are ready to start testing now
        group.throughput(Throughput::Bytes(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_n| {
            b.iter(|| {
                // Insert code that you want tested here
                let _ = dbs_ctx.verify_sharing(&v, &c, &pi, &dss_pk[0]);
            });
        });
    }
    group.finish();
}

pub fn pvss_aggregation(c: &mut Criterion) {
    let mut group = c.benchmark_group("pvss_aggregation");
    BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
    for &n in &TEST_POINTS {
        
        // Start prepping for your test
        let mut rng = &mut StdRng::seed_from_u64(SEED);
        let t = (n - 1) / 2;
        
        let mut public_keys = Vec::new();
        let mut secret_keys = Vec::new();
        let mut dss_kpair = Vec::new();
        let mut dss_pk = Vec::new();
        for _i in 0..n {
            let kpair = Keypair::generate_keypair(&mut rng);
            secret_keys.push(kpair.0);
            public_keys.push(kpair.1);
            let dsskpair = crypto_lib::Keypair::generate_ed25519();
            dss_pk.push(dsskpair.public());
            dss_kpair.push(dsskpair);
        }
        let h2 = G2::prime_subgroup_generator().mul(Scalar::rand(&mut rng));

        let dbs_ctx:Vec<_> = (0..t+1).map(|i| {
            DbsContext::new(&mut rng, h2,n, t, 0, public_keys.clone(), secret_keys[i].clone())
        }).collect();
        let mut comms = Vec::with_capacity(t+1); 
        let mut encs = Vec::with_capacity(t+1); 
        let mut proofs = Vec::with_capacity(t+1); 
        for i in 0..t+1 {
            let (v,c,pi) = 
            dbs_ctx[i].generate_shares(&dss_kpair[i], &mut rng);
            comms.push(v);
            encs.push(c);
            proofs.push(pi);
        }

        let indices:Vec<_> = (0..t+1).map(|i| i as u16).collect();
        // We are ready to start testing now
        group.throughput(Throughput::Bytes(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(t), &t, |b, &_t| {
            let indices = indices.clone();
            b.iter(|| {
                // Insert code that you want tested here
                let _ = dbs_ctx[0].aggregate(&indices, &encs, &comms, &proofs);
            });
        });
    }
    group.finish();
}

pub fn pvss_pverify(c: &mut Criterion) {
    let mut group = c.benchmark_group("pvss_pverify");
    BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
    for &n in &TEST_POINTS {
        
        // Start prepping for your test
        let mut rng = &mut StdRng::seed_from_u64(SEED);
        let t = (n - 1) / 2;
        
        let mut public_keys = Vec::new();
        let mut secret_keys = Vec::new();
        let mut dss_kpair = Vec::new();
        let mut dss_pk = Vec::new();
        for _i in 0..n {
            let kpair = Keypair::generate_keypair(&mut rng);
            secret_keys.push(kpair.0);
            public_keys.push(kpair.1);
            let dsskpair = crypto_lib::Keypair::generate_ed25519();
            dss_pk.push(dsskpair.public());
            dss_kpair.push(dsskpair);
        }
        let h2 = G2::prime_subgroup_generator().mul(Scalar::rand(&mut rng));

        let dbs_ctx:Vec<_> = (0..t+1).map(|i| {
            DbsContext::new(&mut rng, h2,n, t, 0, public_keys.clone(), secret_keys[i].clone())
        }).collect();
        let mut comms = Vec::with_capacity(t+1); 
        let mut encs = Vec::with_capacity(t+1); 
        let mut proofs = Vec::with_capacity(t+1); 
        for i in 0..t+1 {
            let (v,c,pi) = 
            dbs_ctx[i].generate_shares(&dss_kpair[i], &mut rng);
            comms.push(v);
            encs.push(c);
            proofs.push(pi);
        }

        let indices:Vec<_> = (0..t+1).map(|i| i as u16).collect();
        let (agg_pvss, _agg_pi) = dbs_ctx[0].aggregate(&indices, &encs, &comms, &proofs);

        // We are ready to start testing now
        group.throughput(Throughput::Bytes(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_t| {
            b.iter(|| {
                // Insert code that you want tested here
                let _ = dbs_ctx[0].pverify(&agg_pvss);
            });
        });
    }
    group.finish();
}

pub fn pvss_decomposition_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("pvss_decomposition_verify");
    BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
    for &n in &TEST_POINTS {
        
        // Start prepping for your test
        let mut rng = &mut StdRng::seed_from_u64(SEED);
        let t = (n - 1) / 2;
        
        let mut public_keys = Vec::new();
        let mut secret_keys = Vec::new();
        let mut dss_kpair = Vec::new();
        let mut dss_pk = HashMap::new();
        for i in 0..n {
            let kpair = Keypair::generate_keypair(&mut rng);
            secret_keys.push(kpair.0);
            public_keys.push(kpair.1);
            let dsskpair = crypto_lib::Keypair::generate_ed25519();
            dss_pk.insert(i as u16, dsskpair.public());
            dss_kpair.push(dsskpair);
        }
        let h2 = G2::prime_subgroup_generator().mul(Scalar::rand(&mut rng));

        let dbs_ctx:Vec<_> = (0..t+1).map(|i| {
            DbsContext::new(&mut rng, h2,n, t, 0, public_keys.clone(), secret_keys[i].clone())
        }).collect();
        let mut comms = Vec::with_capacity(t+1); 
        let mut encs = Vec::with_capacity(t+1); 
        let mut proofs = Vec::with_capacity(t+1); 
        for i in 0..t+1 {
            let (v,c,pi) = 
            dbs_ctx[i].generate_shares(&dss_kpair[i], &mut rng);
            comms.push(v);
            encs.push(c);
            proofs.push(pi);
        }

        let indices:Vec<_> = (0..t+1).map(|i| i as u16).collect();
        let (agg_pvss, agg_pi) = dbs_ctx[0].aggregate(&indices, &encs, &comms, &proofs);

        // We are ready to start testing now
        group.throughput(Throughput::Bytes(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_t| {
            b.iter(|| {
                // Insert code that you want tested here
                let _ = dbs_ctx[0].decomp_verify(&agg_pvss, &agg_pi[0], &dss_pk);
            });
        });
    }
    group.finish();
}

pub fn pvss_decryption(c: &mut Criterion) {
    let mut group = c.benchmark_group("pvss_decryption");
    BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
    for &n in &TEST_POINTS {
        
        // Start prepping for your test
        let mut rng = &mut StdRng::seed_from_u64(SEED);
        let t = (n - 1) / 2;
        
        let mut public_keys = Vec::new();
        let mut secret_keys = Vec::new();
        let mut dss_kpair = Vec::new();
        let mut dss_pk = HashMap::new();
        for i in 0..n {
            let kpair = Keypair::generate_keypair(&mut rng);
            secret_keys.push(kpair.0);
            public_keys.push(kpair.1);
            let dsskpair = crypto_lib::Keypair::generate_ed25519();
            dss_pk.insert(i as u16, dsskpair.public());
            dss_kpair.push(dsskpair);
        }
        let h2 = G2::prime_subgroup_generator().mul(Scalar::rand(&mut rng));

        let dbs_ctx:Vec<_> = (0..t+1).map(|i| {
            DbsContext::new(&mut rng, h2,n, t, 0, public_keys.clone(), secret_keys[i].clone())
        }).collect();
        let mut comms = Vec::with_capacity(t+1); 
        let mut encs = Vec::with_capacity(t+1); 
        let mut proofs = Vec::with_capacity(t+1); 
        for i in 0..t+1 {
            let (v,c,pi) = 
            dbs_ctx[i].generate_shares(&dss_kpair[i], &mut rng);
            comms.push(v);
            encs.push(c);
            proofs.push(pi);
        }

        let indices:Vec<_> = (0..t+1).map(|i| i as u16).collect();
        let (agg_pvss, _agg_pi) = dbs_ctx[0].aggregate(&indices, &encs, &comms, &proofs);

        // We are ready to start testing now
        group.throughput(Throughput::Bytes(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_t| {
            b.iter(|| {
                // Insert code that you want tested here
                let _ = dbs_ctx[0].decrypt_share(&agg_pvss.encs[0], &dss_kpair[0], &mut rng);
            });
        });
    }
    group.finish();
}

pub fn pvss_verify_decryption(c: &mut Criterion) {
    let mut group = c.benchmark_group("pvss_decryption_verify");
    BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
    for &n in &TEST_POINTS {
        
        // Start prepping for your test
        let mut rng = &mut StdRng::seed_from_u64(SEED);
        let t = (n - 1) / 2;
        
        let mut public_keys = Vec::new();
        let mut secret_keys = Vec::new();
        let mut dss_kpair = Vec::new();
        let mut dss_pk = HashMap::new();
        for i in 0..n {
            let kpair = Keypair::generate_keypair(&mut rng);
            secret_keys.push(kpair.0);
            public_keys.push(kpair.1);
            let dsskpair = crypto_lib::Keypair::generate_ed25519();
            dss_pk.insert(i as u16, dsskpair.public());
            dss_kpair.push(dsskpair);
        }
        let h2 = G2::prime_subgroup_generator().mul(Scalar::rand(&mut rng));

        let dbs_ctx:Vec<_> = (0..n).map(|i| {
            DbsContext::new(&mut rng, h2,n, t, i as u16, public_keys.clone(), secret_keys[i].clone())
        }).collect();
        let mut comms = Vec::with_capacity(t+1); 
        let mut encs = Vec::with_capacity(t+1); 
        let mut proofs = Vec::with_capacity(t+1); 
        for i in 0..t+1 {
            let (v,c,pi) = 
            dbs_ctx[i].generate_shares(&dss_kpair[i], &mut rng);
            comms.push(v);
            encs.push(c);
            proofs.push(pi);
        }

        let indices:Vec<_> = (0..t+1).map(|i| i as u16).collect();
        let (agg_pvss, _agg_pi) = dbs_ctx[0].aggregate(&indices, &encs, &comms, &proofs);
        let mut decs = Vec::with_capacity(n);
        let mut dec_pi = Vec::with_capacity(n);
        for i in 0..n {
            let (d,pi) = dbs_ctx[i].decrypt_share(&agg_pvss.encs[0], &dss_kpair[0], &mut rng);
            decs.push(d);
            dec_pi.push(pi);
        }

        // We are ready to start testing now
        group.throughput(Throughput::Bytes(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_t| {
            b.iter(|| {
                // Insert code that you want tested here
                for i in 0..t+1 {
                    dbs_ctx[0].verify_share(i, &decs[i], &agg_pvss.encs[i], &dec_pi[i], &dss_pk[&(i as u16)]);
                }
            });
        });
    }
    group.finish();
}

pub fn pvss_reconstruction(c: &mut Criterion) {
    let mut group = c.benchmark_group("pvss_reconstruction");
    BenchmarkGroup::sampling_mode(&mut group, criterion::SamplingMode::Flat);
    for &n in &TEST_POINTS {
        
        // Start prepping for your test
        let mut rng = &mut StdRng::seed_from_u64(SEED);
        let t = (n - 1) / 2;
        
        let mut public_keys = Vec::new();
        let mut secret_keys = Vec::new();
        let mut dss_kpair = Vec::new();
        let mut dss_pk = HashMap::new();
        for i in 0..n {
            let kpair = Keypair::generate_keypair(&mut rng);
            secret_keys.push(kpair.0);
            public_keys.push(kpair.1);
            let dsskpair = crypto_lib::Keypair::generate_ed25519();
            dss_pk.insert(i as u16, dsskpair.public());
            dss_kpair.push(dsskpair);
        }
        let h2 = G2::prime_subgroup_generator().mul(Scalar::rand(&mut rng));

        let dbs_ctx:Vec<_> = (0..n).map(|i| {
            DbsContext::new(&mut rng, h2,n, t, i as u16, public_keys.clone(), secret_keys[i].clone())
        }).collect();
        let mut comms = Vec::with_capacity(t+1); 
        let mut encs = Vec::with_capacity(t+1); 
        let mut proofs = Vec::with_capacity(t+1); 
        for i in 0..t+1 {
            let (v,c,pi) = 
            dbs_ctx[i].generate_shares(&dss_kpair[i], &mut rng);
            comms.push(v);
            encs.push(c);
            proofs.push(pi);
        }

        let indices:Vec<_> = (0..t+1).map(|i| i as u16).collect();
        let (agg_pvss, _agg_pi) = dbs_ctx[0].aggregate(&indices, &encs, &comms, &proofs);
        let mut decs = Vec::with_capacity(n);
        let mut dec_pi = Vec::with_capacity(n);
        for i in 0..n {
            let (d,pi) = dbs_ctx[i].decrypt_share(&agg_pvss.encs[0], &dss_kpair[0], &mut rng);
            decs.push(Some(d));
            dec_pi.push(pi);
        }

        for i in 0..n-t-1 {
            decs[i] = None;
        }
        // We are ready to start testing now
        group.throughput(Throughput::Bytes(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_t| {
            b.iter(|| {
                // Insert code that you want tested here
                let _ = dbs_ctx[0].reconstruct(&decs);
            });
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(BENCH_COUNT);
    targets = pvss_generation, 
    pvss_verification,
    pvss_aggregation,
    pvss_pverify,
    pvss_decomposition_verify,
    pvss_decryption,
    pvss_verify_decryption,
    pvss_reconstruction,
);
criterion_main!(benches);