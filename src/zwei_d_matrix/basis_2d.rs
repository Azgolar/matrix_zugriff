/*
    Standardimplementation
*/
pub fn basis_2d(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>, n: usize) {
    for i in 0..n {
        for j in 0..n {
            let mut summe = 0.0;
            for k in 0..n {
                summe = summe + a[i][k] * b[k][j];
            }
            c[i][j] = summe;
        }
    }
}