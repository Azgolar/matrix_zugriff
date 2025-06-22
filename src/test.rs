#[cfg(test)]
mod tests {
    use crate::zahlentyp::Zahlentyp;
    use crate::varianten_1d::*;
    use crate::varianten_2d::*;

    // wandelt einen 1D Vektor zu nxn 2D Matrix um
    fn umwandeln(v: &Vec<i64>, n: usize) -> Vec<Vec<i64>> {
        let mut matrix: Vec<Vec<i64>> = vec![vec![0i64; n]; n];
        for i in 0..n {
            for j in 0..n {
                matrix[i][j] = v[i * n + j];
            }
        }
        matrix
    } 

    // vergleicht zwei 2D Matrizen 
    fn vergleich(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>, n: usize) -> bool {
        for i in 0..n {
            for j in 0..n {
                if a[i][j] != b[i][j] {
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

        for &n in &größen {
            println!("1D: n = {}", n);
            let a: Vec<Vec<i64>> = i64::zufallswerte(n);
            let b: Vec<Vec<i64>> = i64::zufallswerte(n);
            let mut c_1d: Vec<i64> = vec![0; n * n];

            // vergleich aller Implemtierungen mit der Standardimplementierung im 2D Fall
            let mut c: Vec<Vec<i64>> = vec![vec![0; n]; n];
            basis_2d(&a, &b, &mut c, n);

            // umwandeln in 1d Matrizen
            let a_1d: Vec<i64> = a.iter().flatten().copied().collect();
            let b_1d: Vec<i64> = b.iter().flatten().copied().collect();

            basis_lenge_1d(&a_1d, &b_1d, &mut c_1d);
            assert!(vergleich(&umwandeln(&c_1d, n), &c, n), "basis_lenge_1d ist falsch für n = {}", n);

            slice_1d(&a_1d, &b_1d, &mut c_1d, n);
            assert!(vergleich(&umwandeln(&c_1d, n), &c, n), "slice_at_1d ist falsch für n = {}", n);

            iterator_1d(&a_1d, &b_1d, &mut c_1d, n);
            assert!(vergleich(&umwandeln(&c_1d, n), &c, n), "iterator_1d ist falsch für n = {}", n);

            split_at_1d(&a_1d, &b_1d, &mut c_1d, n);
            assert!(vergleich(&umwandeln(&c_1d, n), &c, n), "slit_at_1d ist falsch für n = {}", n);

            unsicher_1d(&a_1d, &b_1d, &mut c_1d, n);
            assert!(vergleich(&umwandeln(&c_1d, n), &c, n), "assert_1d ist falsch für n = {}", n);
        }

        for &n in &größen {
            println!("2D: n = {}", n);
            let a: Vec<Vec<i64>> = i64::zufallswerte(n);
            let b: Vec<Vec<i64>> = i64::zufallswerte(n);
            let mut c: Vec<Vec<i64>> = vec![vec![0; n]; n];

            // vergleich aller Implemtierungen mit der Standardimplementierung im 2D Fall
            let mut c_vergleich: Vec<Vec<i64>> = vec![vec![0; n]; n];
            basis_2d(&a, &b, &mut c_vergleich, n);

            basis_lenge_2d(&a, &b, &mut c);
            assert!(vergleich(&c, &c_vergleich, n), "basis_lenge_2d ist falsch für n = {}", n);

            slice_2d(&a, &b, &mut c, n);
            assert!(vergleich(&c, &c_vergleich, n), "slice_2d ist falsch für n = {}", n);

            iterator_2d(&a, &b, &mut c, n);
            assert!(vergleich(&c, &c_vergleich, n), "iterator_2d ist falsch für n = {}", n);

            split_at_2d(&a, &b, &mut c, n);
            assert!(vergleich(&c, &c_vergleich, n), "split_at_2d ist falsch für n = {}", n);

            unsicher_2d(&a, &b, &mut c, n);
            assert!(vergleich(&c, &c_vergleich, n), "unsicher_2d ist falsch für n = {}", n);
        }

        println!("\nAlle Funktionen sind korrekt");
    }
}