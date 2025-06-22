use crate::zahlentyp::Zahlentyp;

/*
    Standardimplementation
*/
pub fn basis_2d<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize) {
    for i in 0..n {
        for j in 0..n {
            let mut summe = T::default();
            for k in 0..n {
                summe = summe + a[i][k] * b[k][j];
            }
            c[i][j] = summe;
        }
    }
}

/*
    Standardimplementation mit .len() im Schleifenkopf
*/
pub fn basis_lenge_2d<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>) {
    for i in 0..a.len() {
        for j in 0..b.len() {
            let mut summe = T::default();
            for k in 0..a.len() {
                summe = summe + a[i][k] * b[k][j];
            }
            c[i][j] = summe;
        }
    }
}

/*
    Standardimplementation mit Slice
*/
pub fn slice_2d<T: Zahlentyp>(a: &[Vec<T>], b: &[Vec<T>], c: &mut [Vec<T>], n: usize) {


    for i in 0..n {
        for j in 0..n {
            let mut summe = T::default();
            for k in 0..n {
                summe = summe + a[i][k] * b[k][j];
            }
            c[i][j] = summe;
        }
    }
}


/*
    Variante mit iterator
*/
pub fn iterator_2d<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize) {
    /*
        Es werden die ersten n Zeilen durchlaufen, dabei gilt (zeilenindex, Referenz auf aktuelle Zeile)
    */
    for (i, zeile) in a.iter().enumerate() {
        /*
            Schleife über die Spalten von c, dabei gilt (spaltenindex, Referenz auf aktuelle Element in c)
         */
        for (j, ergebnis) in c[i].iter_mut().enumerate() {
            let mut summe = T::default();
            for k in 0..n {
                summe = summe + zeile[k] * b[k][j];
            }
            *ergebnis = summe;
        }
    }
}


/*
    split_at für sicheren Zugriff

    split_at(n) gibt ein Tupel aus Slices zurück:
    - Das erste Element ist ein Slice (Referenz) auf die Zeilen 0 bis n-1
    - Das zweite Element ist ein Slice (Referenz) ab Zeile n bis zur letzten Zeile der Matrix

    --> da die Matrizen immer nur n Zeilen haben, ist das zweite Slice überflüssig
    
*/
pub fn split_at_2d<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize) {
    let (b_geteilt, _): (&[Vec<T>], &[Vec<T>]) = b.split_at(n);

    for i in 0..n {
        // aufteilen der aktuellen Zeilen in a und c
        let (a_zeile, _): (&[T], &[T]) = a[i].split_at(n);
        let (c_zeile, _): (&mut [T], &mut [T]) = c[i].split_at_mut(n);

        for j in 0..n {
            let mut summe = T::default();
            for k in 0..n {
                // a_zeile[k] enthält das k-te Element in Zeile i
                summe = summe + a_zeile[k] * b_geteilt[k][j];
            }
            // Ersetzen des aktuellen Werts in Zeile in bei Position j
            c_zeile[j] = summe;
        }
    }
}


/*
    unsafe

    get_unchecked verhindert den Zugriffscheck 
    --> Der Compiler kann nicht garantieren dass die Indizes innerhalb der Grenzen sind
    --> Sicherheit ist nicht garantiert
    --> unsafe notwendig

    falsche Indizes führt also zu undefiniertem Verhalten oder Programmabsturz
*/
pub fn unsicher_2d<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize)  {
    unsafe {
        for i in 0..n {
            for j in 0..n {
                let mut summe = T::default();
                for k in 0..n {
                    summe = summe + *a.get_unchecked(i).get_unchecked(k) * *b.get_unchecked(k).get_unchecked(j);
                }
                *c.get_unchecked_mut(i).get_unchecked_mut(j) = summe;
            }
        }
    }
}

