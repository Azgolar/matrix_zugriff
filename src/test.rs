#[cfg(test)]
mod tests {
    use crate::matrix::zufallsmatrix_2d;

    // Import für 1D Fall
    use crate::bin::ein_d_matrix::basis_1d::basis_1d;
    use crate::bin::ein_d_matrix::basis_laenge_1d::basis_länge_1d;
    use crate::bin::ein_d_matrix::iterator_1d::iterator_1d;
    use crate::bin::ein_d_matrix::slice_1d::slice_1d;
    use crate::bin::ein_d_matrix::split_at_1d::split_at_1d;
    use crate::bin::ein_d_matrix::unsicher_1d::unsicher_1d;

    // Import für 2D Fall
    use crate::bin::zwei_d_matrix::basis_2d::basis_2d;
    use crate::bin::zwei_d_matrix::basis_laenge_2d::basis_länge_2d;
    use crate::bin::zwei_d_matrix::iterator_2d::iterator_2d;
    use crate::bin::zwei_d_matrix::slice_2d::slice_2d;
    use crate::bin::zwei_d_matrix::split_at_2d::split_at_2d;
    use crate::bin::zwei_d_matrix::unsicher_2d::unsicher_2d;
    

    // wandelt einen 1D Vektor zu nxn 2D Matrix um
    fn umwandeln(v: &Vec<f64>, n: usize) -> Vec<Vec<f64>> {
        let mut matrix: Vec<Vec<f64>> = vec![vec![0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                matrix[i][j] = v[i * n + j];
            }
        }
        matrix
    } 

    // vergleicht zwei 2D Matrizen 
    fn vergleich(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, n: usize) -> bool {
        let genauigkeit = 1e-10;
        for i in 0..n {
            for j in 0..n {
                if (a[i][j] - b[i][j]).abs() > genauigkeit {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn testen() {
        // Mix aus Matrizen mit gerader und ungerader Größe
        let größen: Vec<usize> = vec![6, 7, 14, 15, 32, 37, 50, 53, 64, 77, 90, 107, 126, 141, 180, 187, 200, 211, 256, 273, 300];
        println!("\nTesten mit Matrizen der Größe = {:?}\n", größen);

        // testen der Funktionen für 1D Matrizen 
        for &n in &größen {
            println!("1D: n = {}", n);
            let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
            let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);

            // vergleich aller Implemtierungen mit der Standardimplementierung im 2D Fall
            let mut v: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
            basis_2d(&a, &b, &mut v, n);

            // umwandeln in 1d Matrizen
            let d: Vec<f64> = a.iter().flatten().copied().collect();
            let e: Vec<f64> = b.iter().flatten().copied().collect();
            let mut ergebnis: Vec<f64> = vec![0.0; n * n];

            basis_1d(&d, &e, &mut ergebnis, n);
            assert!(vergleich(&umwandeln(&ergebnis, n), &v, n), "basis_1d.rs ist falsch für n = {}", n);

            ergebnis = vec![0.0; n * n];
            basis_länge_1d(&d, &e, &mut ergebnis);
            assert!(vergleich(&umwandeln(&ergebnis, n), &v, n), "basis_laenge_1d.rs ist falsch für n = {}", n);

            ergebnis = vec![0.0; n * n];
            slice_1d(&d, &e, &mut ergebnis, n);
            assert!(vergleich(&umwandeln(&ergebnis, n), &v, n), "slice_at_1d.rs ist falsch für n = {}", n);

            ergebnis = vec![0.0; n * n];
            iterator_1d(&d, &e, &mut ergebnis, n);
            assert!(vergleich(&umwandeln(&ergebnis, n), &v, n), "iterator_1d.rs ist falsch für n = {}", n);

            ergebnis = vec![0.0; n * n];
            split_at_1d(&d, &e, &mut ergebnis, n);
            assert!(vergleich(&umwandeln(&ergebnis, n), &v, n), "split_at_1d.rs ist falsch für n = {}", n);

            ergebnis = vec![0.0; n * n];
            unsicher_1d(&d, &e, &mut ergebnis, n);
            assert!(vergleich(&umwandeln(&ergebnis, n), &v, n), "unsicher_1d.rs ist falsch für n = {}", n);
        }

        // testen der Funktionen für 2D Matrizen
        for &n in &größen {
            println!("2D: n = {}", n);
            let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
            let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
            let mut ergebnis: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

            // vergleich aller Implemtierungen mit der Standardimplementierung im 2D Fall
            let mut v: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
            basis_2d(&a, &b, &mut v, n);

            basis_länge_2d(&a, &b, &mut ergebnis);
            assert!(vergleich(&ergebnis, &v, n), "basis_lenge_2d.rs ist falsch für n = {}", n);

            ergebnis = vec![vec![0.0; n]; n];
            slice_2d(&a, &b, &mut ergebnis, n);
            assert!(vergleich(&ergebnis, &v, n), "slice_2d.rs ist falsch für n = {}", n);

            ergebnis = vec![vec![0.0; n]; n];
            iterator_2d(&a, &b, &mut ergebnis, n);
            assert!(vergleich(&ergebnis, &v, n), "iterator_2d.rs ist falsch für n = {}", n);

            ergebnis = vec![vec![0.0; n]; n];
            split_at_2d(&a, &b, &mut ergebnis, n);
            assert!(vergleich(&ergebnis, &v, n), "split_at_2d.rs ist falsch für n = {}", n);

            ergebnis = vec![vec![0.0; n]; n];
            unsicher_2d(&a, &b, &mut ergebnis, n);
            assert!(vergleich(&ergebnis, &v, n), "unsicher_2d.rs ist falsch für n = {}", n);
        }

        println!("\nAlle Funktionen sind korrekt");
    }
}