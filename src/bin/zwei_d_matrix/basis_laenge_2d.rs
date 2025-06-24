/*
    Standardimplementation mit .len() im Schleifenkopf
*/
pub fn basis_länge_2d(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>) {
    for i in 0..a.len() {
        for j in 0..b.len() {
            let mut summe = 0.0;
            for k in 0..a.len() {
                summe = summe + a[i][k] * b[k][j];
            }
            c[i][j] = summe;
        }
    }
}
