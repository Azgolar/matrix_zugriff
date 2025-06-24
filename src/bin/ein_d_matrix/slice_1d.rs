/*
    1D Vektor mit Slice
*/
pub fn slice_1d(a: &[f64], b: &[f64], c: &mut [f64], n: usize) {
    for i in 0..n {
        // startindex der i-ten Zeile
        let start: usize = i * n;
        for j in 0..n {
            let mut summe: f64 = 0.0;
            for k in 0..n {
                summe = summe + a[start + k] * b[k * n + j];
            }
            c[start + j] = summe;
        }
    }
}