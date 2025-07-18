/*
    1D Vektor im Zeilenformat mit len() im Schleifenkopf
*/
pub fn basis_länge_1d(a: &Vec<f64>, b: &Vec<f64>, c: &mut Vec<f64>) {
    let n: usize = (a.len() as f64).sqrt() as usize;
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
