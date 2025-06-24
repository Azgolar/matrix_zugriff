/*
    split_at für sicheren Zugriff

    split_at(n) gibt ein Tupel aus Slices zurück:
    - Das erste Element ist ein Slice (Referenz) auf die Zeilen 0 bis n-1
    - Das zweite Element ist ein Slice (Referenz) ab Zeile n bis zur letzten Zeile der Matrix

    --> da die Matrizen immer nur n Zeilen haben, ist das zweite Slice überflüssig
    
*/
pub fn split_at_2d(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>, n: usize) {
    let (b_geteilt, _): (&[Vec<f64>], &[Vec<f64>]) = b.split_at(n);

    for i in 0..n {
        // aufteilen der aktuellen Zeilen in a und c
        let (a_zeile, _): (&[f64], &[f64]) = a[i].split_at(n);
        let (c_zeile, _): (&mut [f64], &mut [f64]) = c[i].split_at_mut(n);

        for j in 0..n {
            let mut summe = 0.0;
            for k in 0..n {
                // a_zeile[k] enthält das k-te Element in Zeile i
                summe = summe + a_zeile[k] * b_geteilt[k][j];
            }
            // Ersetzen des aktuellen Werts in Zeile in bei Position j
            c_zeile[j] = summe;
        }
    }
}