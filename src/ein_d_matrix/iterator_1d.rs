/*
    1D Vektor mit Iterator
*/
pub fn iterator_1d(a: &Vec<f64>, b: &Vec<f64>, c: &mut Vec<f64>, n: usize) {

    // über alle Zeilen in Stücken der Größen n iterieren, dabei gilt (zeilenindex, aktuelle zeile)
    for (i, a_zeile) in a.chunks(n).enumerate() {
        for j in 0..n {
            let mut summe: f64 = 0.0;
            for k in 0..n {
                summe = summe + a_zeile[k] * b[k * n + j];
            }
            c[i * n + j] = summe;
        }
    }
}