use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::hint::black_box;
use zugriffszeit::zahlentyp::Zahlentyp;
use zugriffszeit::varianten_1D::*;
use zugriffszeit::varianten_2D::*;
use zugriffszeit::eingabe::erzeugen;

/*
    Benchmark mit Criterion für 2D Matrixmultiplikation durchführen
*/
fn benchmark_2d<T: Zahlentyp>(einstellungen: &mut Criterion) {
    // Benchmarks zu einer Gruppe zusammenfasen für Auswertung
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group(format!("2D Matrix: {}", T::type_name()));
    // Jeder Benchmark wird 100 mal durchgeführt
    gruppe.sample_size(100);

    // Matrixgrößen für Benchmark erzeugen
    let zu_testen: Vec<usize> = erzeugen(1, 256);
    
    for n in zu_testen {
        // Matrizen mit Zufallswerten füllen
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
            messen.iter(|| {
                basis_2d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit Zeilenformat mit len() im Schleifenkopf
        gruppe.bench_with_input(BenchmarkId::new("Variante len()", n), &n, |messen, _| {
            messen.iter(|| {
                basis_lenge_2d(black_box(&a), black_box(&b,), black_box(&mut c));
            });
        });

        // Variante mit Slice
        gruppe.bench_with_input(BenchmarkId::new("Slice", n), &n, |messen, &n| {
            messen.iter(|| {
                slice_2d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit Iterator
        gruppe.bench_with_input(BenchmarkId::new("Iterator", n), &n, |messen, &n| {
            messen.iter(|| {
                iteartor_2d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit split_at
        gruppe.bench_with_input(BenchmarkId::new("Split", n), &n, |messen, &n| {
            messen.iter(|| {
                split_2d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });

        // Variante mit unsafe
        gruppe.bench_with_input(BenchmarkId::new("unsafe", n), &n, |messen, &n| {
            messen.iter(|| {
                unsicher_2d(black_box(&a), black_box(&b,), black_box(&mut c), black_box(n));
            });
        });
    }

    // Messungen aller Gruppen beenden und Auswertungen erzeugen
    gruppe.finish();
}
