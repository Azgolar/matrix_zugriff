/*
    Variante mit iterator
*/
pub fn iterator_2d(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>, n: usize) {
    /*
        Es werden die ersten n Zeilen durchlaufen, dabei gilt (zeilenindex, Referenz auf aktuelle Zeile)
    */
    for (i, zeile) in a.iter().enumerate() {
        /*
            Schleife über die Spalten von c, dabei gilt (spaltenindex, Referenz auf aktuelle Element in c)
         */
        for (j, ergebnis) in c[i].iter_mut().enumerate() {
            let mut summe = 0.0;
            for k in 0..n {
                summe = summe + zeile[k] * b[k][j];
            }
            *ergebnis = summe;
        }
    }
}