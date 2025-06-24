/*
    split_at für sicheren Zugriff

    split_at(n) gibt ein Tupel aus Slices zurück:
    - Das erste Element ist ein Slice (Referenz) auf die Zeilen 0 bis n-1
    - Das zweite Element ist ein Slice (Referenz) ab Zeile n bis zur letzten Zeile der Matrix

    --> da die Matrizen immer nur n Zeilen haben, ist das zweite Slice überflüssig
    
*/
pub fn split_at_1d(mut a: &[f64], b: &[f64], mut c: &mut [f64], n: usize) {
    for _ in 0..n {
        // aufteilen der aktuellen Zeilen in a und c
        let (a_zeile, a_rest): (&[f64], &[f64]) = a.split_at(n);
        let (ergebnis, c_rest): (&mut [f64], &mut [f64]) = c.split_at_mut(n); 

        for j in 0..n {
            let mut summe: f64 = 0.0;
            for k in 0..n {
                // a_zeile[k] enthält das k-te Element in Zeile i
                // für b lohnt sich split_at nicht da wir spaltenweise und nicht zeilenweise zugreifen
                summe = summe + a_zeile[k] * b[k * n + j];
            }
            // Ersetzen des aktuellen Werts in Zeile in bei Position j
            ergebnis[j] = summe;
        }
        // Zeiger updaten für nächste Iteration
        a = a_rest;
        c = c_rest;
    }
}