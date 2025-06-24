/*
    1D Vektor mit unsafe
*/
pub fn unsicher_1d(a: &Vec<f64>, b: &Vec<f64>, c: &mut Vec<f64>, n: usize) {
    unsafe {
        for i in 0..n {
            // startindex der i-ten Zeile
            let start: usize = i * n;
            for j in 0..n {
                let mut summe: f64 = 0.0;
                for k in 0..n {
                    summe = summe + *a.get_unchecked(start + k) * *b.get_unchecked(k * n + j);
                }
                *c.get_unchecked_mut(start + j) = summe;
            }
        }
    }
}
