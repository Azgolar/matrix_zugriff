use rand::random_range;

/*
    Erstellt eine 1D Matrix mit Datentyp f64 mit Zufallswerten im Bereich [-1.0,1.0]
*/
pub fn zufallsmatrix_1d(n: usize) -> Vec<f64> {
    let mut matrix: Vec<f64> = vec![0.0; n * n];
    for i in 0..(n * n) {
            matrix[i] = random_range(-1.0..=1.0);
        }
    matrix
}

/*
    Erstellt eine 2D Matrix mit Zufallswerten im Bereich [-1.0,1.0]
*/
pub fn zufallsmatrix_2d(n: usize) -> Vec<Vec<f64>> {
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = random_range(-1.0..=1.0);
        }
    }
    matrix
}