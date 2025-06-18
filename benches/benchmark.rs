use criterion::{criterion_group, criterion_mrain, BenchmarkId};
use matrix::{n_erzeugen, zufallsmatrix};
use varianten::{basis, assert, iterator, einD, unsicher};


use matrix_zugriffszeit::*;

/*
    Steuerung des Benchmarks mit Criterion
*/
fn benchmark<T: Zahlentyp>(einstellung: &mut Criterion) {
    // Benchmark Gruppe für jeden Zahlentyp anlegen
    let mut group = einstellung.benchmark_group(format!("Matrixmultiplikation mit Typ {}", T::type_name()));

    // Auf Kern 1 pinnen für bessere Vergleichbarkeit
    let kern = get_core_ids().unwrap();
    set_for_current(kern[1]);

    let größen: Vec<u32> = n_erzeugen(6,200);
    
    for n in größen {

        // Zufallsmatrizen erzeugen
        let a: Vec<Vec<T>> = zufallsmatrix(n);
        let b: Vec<Vec<T>> = zufallsmatrix(n);

        // Ergebnismatrix mit 0 intialisieren
        let c: Vec<Vec<T>> = vec![vec![0; n]; n];

        // Benchmarken dar Variante mit standard Implementation
        group.bench_with_input(BenchmarkId::new("Variante: basis", n) &n, |starten, &n|
        {
            laufen.iter(|| {
                let mut c = c.clone();
                basis(&a, &b, &mut c, n);
                black_box(&c);
            });
        });

        // Benchmarken dar Variante mit assert 
        group.bench_with_input(BenchmarkId::new("Variante: assert", n) &n, |starten, &n|
        {
            laufen.iter(|| {
                let mut c = c.clone();
                assert(&a, &b, &mut c, n);
                black_box(&c);
            });
        });

        // Benchmarken dar Variante mit iterator
        group.bench_with_input(BenchmarkId::new("Variante: iterator", n) &n, |starten, &n|
        {
            laufen.iter(|| {
                let mut c = c.clone();
                iterator(&a, &b, &mut c, n);
                black_box(&c);
            });
        });

        // Benchmarken dar Variante mit unsafe
        group.bench_with_input(BenchmarkId::new("Variante: unsafe", n) &n, |starten, &n|
        {
            laufen.iter(|| {
                let mut c = c.clone();
                unsicher(&a, &b, &mut c, n);
                black_box(&c);
            });
        });

        // Benchmarken dar Variante mit 1D Vektoren
        let a_1d = a.flatten();
        let b_1d = b.flatten();
        let c_1d = c.flatten();
        group.bench_with_input(BenchmarkId::new("Variante: 1D Vektoren", n) &n, |starten, &n|
        {
            laufen.iter(|| {
                let mut c = c.clone();
                einD(&a, &b, &mut c, n);
                black_box(&c);
            });
        });

        group.finish();
    }
}