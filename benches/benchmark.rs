use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::{hint::black_box, time::Duration};
use core_affinity::{CoreId, get_core_ids, set_for_current};
use zugriffszeit::{matrix::{zufallsmatrix_1d, zufallsmatrix_2d}};

// Import für 1D Fall
use zugriffszeit::bin::ein_d_matrix::basis_1d::basis_1d;
use zugriffszeit::bin::ein_d_matrix::basis_laenge_1d::basis_länge_1d;
use zugriffszeit::bin::ein_d_matrix::iterator_1d::iterator_1d;
use zugriffszeit::bin::ein_d_matrix::slice_1d::slice_1d;
use zugriffszeit::bin::ein_d_matrix::split_at_1d::split_at_1d;
use zugriffszeit::bin::ein_d_matrix::unsicher_1d::unsicher_1d;

// Import für 2D Fall
use zugriffszeit::bin::zwei_d_matrix::basis_2d::basis_2d;
use zugriffszeit::bin::zwei_d_matrix::basis_laenge_2d::basis_länge_2d;
use zugriffszeit::bin::zwei_d_matrix::iterator_2d::iterator_2d;
use zugriffszeit::bin::zwei_d_matrix::slice_2d::slice_2d;
use zugriffszeit::bin::zwei_d_matrix::split_at_2d::split_at_2d;
use zugriffszeit::bin::zwei_d_matrix::unsicher_2d::unsicher_2d;

/*
    Einstellungen für alle Benchmarks
    globale Variablen müssen const oder static sein
*/ 
const ANZAHL: usize = 10;     // Anzahl der Durchläufe
const ZEIT: u64 = 60;       // maixmale Zeit in Sekunden je Durchlauf
const MATRITZEN: &[usize] = &[4, 8, 11, 16, 25, 32, 64, 91, 128, 256, 357, 512, 787, 1024, 2189, 3100, 4096]; // Matrixgrößen

/*
    1D Benchmarks
*/

fn eins_1d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    // Anzahl der Durchläufe
    gruppe.sample_size(ANZAHL);
    // maximal erlaubte Zeit je durchlauf
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    // alle logischen Kerne für Pinning
    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    // Benchmark mit den einzelnen Matrixgrößen durchführen
    for &n in MATRITZEN {
        // Matrizen intialisieren
        let a: Vec<f64> = zufallsmatrix_1d(n);
        let b: Vec<f64> = zufallsmatrix_1d(n);
        let mut c: Vec<f64> = vec![0.0; n * n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, &n| {
            // Benchmark auf logischen Kern 0 pinnen
            set_for_current(kerne[0]);

            // ausführen
            // Black Box wird verwendet um den Compiler an ungewünschten Optimierungen zu hindern
            messen.iter(|| {
                basis_1d(black_box(&a), black_box(&b), black_box(&mut c), black_box(n));
            });
        });
    }

    // benchmark abschließen und Statistiken erstellen
    gruppe.finish();
}

fn zwei_1d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<f64> = zufallsmatrix_1d(n);
        let b: Vec<f64> = zufallsmatrix_1d(n);
        let mut c: Vec<f64> = vec![0.0; n * n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, _| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                basis_länge_1d(black_box(&a), black_box(&b), black_box(&mut c));
            });
        });
    }
    gruppe.finish();
}

fn drei_1d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<f64> = zufallsmatrix_1d(n);
        let b: Vec<f64> = zufallsmatrix_1d(n);
        let mut c: Vec<f64> = vec![0.0; n * n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, &n| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                split_at_1d(black_box(&a), black_box(&b), black_box(&mut c), black_box(n));
            });
        });
    }
    gruppe.finish();
}

fn vier_1d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<f64> = zufallsmatrix_1d(n);
        let b: Vec<f64> = zufallsmatrix_1d(n);
        let mut c: Vec<f64> = vec![0.0; n * n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, &n| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                slice_1d(black_box(&a), black_box(&b), black_box(&mut c), black_box(n));
            });
        });
    }
    gruppe.finish();
}

fn fünf_1d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<f64> = zufallsmatrix_1d(n);
        let b: Vec<f64> = zufallsmatrix_1d(n);
        let mut c: Vec<f64> = vec![0.0; n * n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, &n| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                iterator_1d(black_box(&a), black_box(&b), black_box(&mut c), black_box(n));
            });
        });
    }
    gruppe.finish();
}

fn sechs_1d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<f64> = zufallsmatrix_1d(n);
        let b: Vec<f64> = zufallsmatrix_1d(n);
        let mut c: Vec<f64> = vec![0.0; n * n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, &n| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                unsicher_1d(black_box(&a), black_box(&b), black_box(&mut c), black_box(n));
            });
        });
    }
    gruppe.finish();
}





/*
    2D Benchmarks
*/

fn eins_2d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, &n| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                basis_2d(black_box(&a), black_box(&b), black_box(&mut c), black_box(n));
            });
        });
    }
    gruppe.finish();
}

fn zwei_2d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, _| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                basis_länge_2d(black_box(&a), black_box(&b), black_box(&mut c));
            });
        });
    }
    gruppe.finish();
}

fn drei_2d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, &n| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                slice_2d(black_box(&a), black_box(&b), black_box(&mut c), black_box(n));
            });
        });
    }
    gruppe.finish();
}

fn vier_2d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, &n| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                iterator_2d(black_box(&a), black_box(&b), black_box(&mut c), black_box(n));
            });
        });
    }
    gruppe.finish();
}

fn fünf_2d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, &n| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                split_at_2d(black_box(&a), black_box(&b), black_box(&mut c), black_box(n));
            });
        });
    }
    gruppe.finish();
}

fn sechs_2d(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("1D basis");
    
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    let kerne: Vec<CoreId> = get_core_ids().unwrap();

    for &n in MATRITZEN {
        let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

        gruppe.bench_with_input(BenchmarkId::new("basis_1d", n), &n, | messen, &n| {
            set_for_current(kerne[0]);

            messen.iter(|| {
                unsicher_2d(black_box(&a), black_box(&b), black_box(&mut c), black_box(n));
            });
        });
    }
    gruppe.finish();
}




criterion_group!(erster_benchmark, eins_1d, zwei_1d, drei_1d, vier_1d, fünf_1d, sechs_1d);

criterion_group!(zweiter_benchmark, eins_2d, zwei_2d, drei_2d, vier_2d, fünf_2d, sechs_2d);

criterion_main!(erster_benchmark, zweiter_benchmark);