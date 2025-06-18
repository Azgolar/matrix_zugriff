use criterion::{criterion_group, Criterion, criterion_main, BenchmarkId};
use zugriffszeit::varianten::{basis, basis_assert, slice, slice_assert, mit_iteartor, mit_iterator_assert, einD, einD_assert, unsicher};
use zugriffszeit::zahlentyp::Zahlentyp;
use core_affinity::{CoreId, set_for_current};

/*
    Steuerung des Benchmarks mit Criterion
*/
fn benchmark<T: Zahlentyp>(einstellung: &mut Criterion) {
    }
}