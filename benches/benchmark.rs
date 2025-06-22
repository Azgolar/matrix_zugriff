use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::{hint::black_box, thread::sleep, time::Duration};
use zugriffszeit::zahlentyp::Zahlentyp;
use zugriffszeit::varianten_1d::*;
use zugriffszeit::varianten_2d::*;
use core_affinity::{CoreId, get_core_ids, set_for_current};

/*
    Benchmark mit Criterion für 2D Matrixmultiplikation durchführen
*/
fn benchmark_2d<T: Zahlentyp>(einstellungen: &mut Criterion) {
    // Benchmarks zu einer Gruppe zusammenfasen für Auswertung
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group(format!("2D Matrix: {}", T::type_name()));
    // Jeder Benchmark wird 100 mal durchgeführt
    gruppe.sample_size(50);

    // 3 Stunden zeit
    gruppe.measurement_time(Duration::from_secs(3 * 60 * 60));

    // Kerne zum pinnen der Funktionen beim Benchmarking
    let kern: Vec<CoreId> = get_core_ids().unwrap();

    // Matrixgrößen für Benchmark erzeugen
    let testen: Vec<usize> = vec![4, 8, 11, 16, 25, 32, 64, 91, 128, 256, 357, 512, 787, 1024, 2189, 3100, 4096];
    
    for n in testen {
        // 2D Matrizen mit Zufallswerten füllen
        let a: Vec<Vec<T>> = T::zufallswerte(n);
        let b: Vec<Vec<T>> = T::zufallswerte(n);
        // Ergebnismatrix mit null initialisieren. Jeder Benchmarkdurchlauf überschreibt die Werte
        let mut c: Vec<Vec<T>> = vec![vec![T::default(); n]; n];

        /*  Benchmark durchführen für die einzelnen Varianten 

            Dabei wird immer Black Box verwendet da es den Compiler daran hindern soll zu erkennen dass 
            sich die Eingaben A, B und C während den Durchläufen nicht ändern. 
            
            --> Optimierungen wie redundanten Berechnungen entfernen und das Cachen von Konstanten werden
                verhindert. Denn dies würde den Benchmark verfälschen
        */ 

        // Basis Variante
        gruppe.bench_with_input(BenchmarkId::new("Basis", n), &n, |messen, &n| {
            set_for_current(kern[0]);
            messen.iter(|| {
                basis_2d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit Zeilenformat mit len() im Schleifenkopf
        gruppe.bench_with_input(BenchmarkId::new("Variante len()", n), &n, |messen, _| {
            set_for_current(kern[0]);
            messen.iter(|| {
                basis_lenge_2d(black_box(&a), black_box(&b,), black_box(&mut c));
            });
        });

        // Variante mit Slice
        gruppe.bench_with_input(BenchmarkId::new("Slice", n), &n, |messen, &n| {
            set_for_current(kern[0]);
            messen.iter(|| {
                slice_2d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit Iterator
        gruppe.bench_with_input(BenchmarkId::new("Iterator", n), &n, |messen, &n| {
            set_for_current(kern[0]);
            messen.iter(|| {
                iterator_2d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit split_at
        gruppe.bench_with_input(BenchmarkId::new("Split", n), &n, |messen, &n| {
            set_for_current(kern[0]);
            messen.iter(|| {
                split_at_2d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit unsafe
        gruppe.bench_with_input(BenchmarkId::new("unsafe", n), &n, |messen, &n| {
            set_for_current(kern[0]);
            messen.iter(|| {
                unsicher_2d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });
    }

    // Messungen aller Gruppen abschließen und Auswertungen erzeugen
    gruppe.finish();
}


/*
    Benchmark mit Criterion für 1D Matrixmultiplikation durchführen
*/
fn benchmark_1d<T: Zahlentyp>(einstellungen: &mut Criterion) {
    // Benchmarks zu einer Gruppe zusammenfasen für Auswertung
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group(format!("1D Matrix: {}", T::type_name()));
    // Jeder Benchmark wird 100 mal durchgeführt
    gruppe.sample_size(50);

    // 3 Stunden zeit
    gruppe.measurement_time(Duration::from_secs(3 * 60 * 60));

    // Kerne zum pinnen der Funktionen beim Benchmarking
    let kern: Vec<CoreId> = get_core_ids().unwrap();

    // Matrixgrößen für Benchmark erzeugen
    let testen: Vec<usize> = vec![4, 8, 11, 16, 25, 32, 64, 91, 128, 256, 357, 512, 787, 1024, 2048, 3103, 4096];
    
    for n in testen {
        // 1D Matrizen mit Zufallswerten füllen
        let a: Vec<T> = T::zufallswerte(n).iter().flatten().copied().collect();
        let b: Vec<T> = T::zufallswerte(n).iter().flatten().copied().collect();
        // Ergebnismatrix mit null initialisieren. Jeder Benchmarkdurchlauf überschreibt die Werte
        let mut c: Vec<T> = vec![T::default(); n * n];

        /*  Benchmark durchführen für die einzelnen Varianten 

            Dabei wird immer Black Box verwendet da es den Compiler daran hindern soll zu erkennen dass 
            sich die Eingaben A, B und C während den Durchläufen nicht ändern. 
            
            --> Optimierungen wie redundanten Berechnungen entfernen und das Cachen von Konstanten werden
                verhindert. Denn dies würde den Benchmark verfälschen
        */ 

        // Basis Variante
        gruppe.bench_with_input(BenchmarkId::new("Basis", n), &n, |messen, &n| {
            set_for_current(kern[0]);
            messen.iter(|| {
                basis_1d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit Zeilenformat mit len() im Schleifenkopf
        gruppe.bench_with_input(BenchmarkId::new("Variante len()", n), &n, |messen, _| {
            set_for_current(kern[0]);
            messen.iter(|| {
                basis_lenge_1d(black_box(&a), black_box(&b,), black_box(&mut c));
            });
        });

        // Variante mit Slice
        gruppe.bench_with_input(BenchmarkId::new("Slice", n), &n, |messen, &n| {
            set_for_current(kern[0]);
            messen.iter(|| {
                slice_1d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit Iterator
        gruppe.bench_with_input(BenchmarkId::new("Iterator", n), &n, |messen, &n| {
            set_for_current(kern[0]);
            messen.iter(|| {
                iterator_1d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit split_at
        gruppe.bench_with_input(BenchmarkId::new("Split", n), &n, |messen, &n| {
            set_for_current(kern[0]);
            messen.iter(|| {
                split_at_1d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit unsafe
        gruppe.bench_with_input(BenchmarkId::new("unsafe", n), &n, |messen, &n| {
            set_for_current(kern[0]);
            messen.iter(|| {
                unsicher_1d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });
    }

    // Messungen aller Gruppen abschließen und Auswertungen erzeugen
    gruppe.finish();
}

// Benchmark mit verschiedenen Datentypen ausführen
fn benchmark_u32(einstellungen: &mut Criterion) {
    println!("\nBenchmark mit Datentyp u32\n");
    sleep(Duration::from_secs(5));

    benchmark_2d::<u32>(einstellungen);
    benchmark_1d::<u32>(einstellungen);
}

// Benchmark mit verschiedenen Datentypen ausführen
fn benchmark_u64(einstellungen: &mut Criterion) {
    println!("\nBenchmark mit Datentyp u64\n");
    sleep(Duration::from_secs(5));

    benchmark_2d::<u64>(einstellungen);
    benchmark_1d::<u64>(einstellungen);
}

// Benchmark mit verschiedenen Datentypen ausführen
fn benchmark_i32(einstellungen: &mut Criterion) {
    println!("\nBenchmark mit Datentyp i32\n");
    sleep(Duration::from_secs(5));

    benchmark_2d::<i32>(einstellungen);
    benchmark_1d::<i32>(einstellungen);
}

// Benchmark mit verschiedenen Datentypen ausführen
fn benchmark_i64(einstellungen: &mut Criterion) {
    println!("\nBenchmark mit Datentyp i64\n");
    sleep(Duration::from_secs(5));

    benchmark_2d::<i64>(einstellungen);
    benchmark_1d::<i64>(einstellungen);
}

// Benchmark mit verschiedenen Datentypen ausführen
fn benchmark_f32(einstellungen: &mut Criterion) {
    println!("\nBenchmark mit Datentyp f32\n");
    sleep(Duration::from_secs(5));

    benchmark_2d::<f32>(einstellungen);
    benchmark_1d::<f32>(einstellungen);
}

// Benchmark mit verschiedenen Datentypen ausführen
fn benchmark_f64(einstellungen: &mut Criterion) {
    println!("\nBenchmark mit Datentyp f64\n");
    sleep(Duration::from_secs(5));

    benchmark_2d::<f64>(einstellungen);
    benchmark_1d::<f64>(einstellungen);
}

// Benchmarkgruppe namens Matrix mit den entsprechenden Funktionen erstellen
criterion_group!(matrix, benchmark_u32, benchmark_u64, benchmark_i32, benchmark_i64, benchmark_f32, benchmark_f64);

// führt die Benchmarks in der Gruppe matrix aus  
criterion_main!(matrix);