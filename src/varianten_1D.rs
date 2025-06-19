use crate::zahlentyp::Zahlentyp;

/*
    1D Vektor im Zeilenformat
*/
pub fn basis_1D<T: Zahlentyp>(a: &Vec<T>, b: &Vec<T>, c: &mut Vec<T>, n: usize) {
    for i in 0..n {
        // startindex der i-ten Zeile
        let start: usize = i * n;
        for j in 0..n {
            let mut summe: T = T::default();
            for k in 0..n {
                summe = summe + a[start + k] * b[k * n + j];
            }
            c[start + j] = summe;
        }
    }
}

/*
    1D Vektor im Zeilenformat mit len()
*/
pub fn basis_len_1D<T: Zahlentyp>(a: &Vec<T>, b: &Vec<T>, c: &mut Vec<T>) {
    let n: usize = (a.len() as f64).sqrt() as usize;
    for i in 0..n {
        // startindex der i-ten Zeile
        let start: usize = i * n;
        for j in 0..n {
            let mut summe: T = T::default();
            for k in 0..n {
                summe = summe + a[start + k] * b[k * n + j];
            }
            c[start + j] = summe;
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
pub fn split_at_1D<T: Zahlentyp>(mut a: &[T], b: &[T], mut c: &mut [T], n: usize) {
    for _ in 0..n {
        // aufteilen der aktuellen Zeilen in a und c
        let (a_zeile, a_rest): (&[T], &[T]) = a.split_at(n);
        let (ergebnis, c_rest): (&mut [T], &mut [T]) = c.split_at_mut(n); 

        for j in 0..n {
            let mut summe: T = T::default();
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

/*
    1D Vektor mit Slice
*/
pub fn slice_1D<T: Zahlentyp>(a: &[T], b: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        // startindex der i-ten Zeile
        let start: usize = i * n;
        for j in 0..n {
            let mut summe: T = T::default();
            for k in 0..n {
                summe = summe + a[start + k] * b[k * n + j];
            }
            c[start + j] = summe;
        }
    }
}

/*
    1D Vektor mit Iterator
*/
pub fn iterator_1D<T: Zahlentyp>(a: &Vec<T>, b: &Vec<T>, c: &mut Vec<T>, n: usize) {

    // über alle Zeilen in Stücken der Größen n iterieren, dabei gilt (zeilenindex, aktuelle zeile)
    for (i, a_zeile) in a.chunks(n).enumerate() {
        for j in 0..n {
            let mut summe: T = T::default();
            for k in 0..n {
                summe = summe + a_zeile[k] * b[k * n + j];
            }
            c[i * n + j] = summe;
        }
    }
}

/*
    1D Vektor mit unsafe
*/
pub fn unsicher_1D<T: Zahlentyp>(a: &Vec<T>, b: &Vec<T>, c: &mut Vec<T>, n: usize) {
    unsafe {
        for i in 0..n {
            // startindex der i-ten Zeile
            let start: usize = i * n;
            for j in 0..n {
                let mut summe: T = T::default();
                for k in 0..n {
                    summe = summe + *a.get_unchecked(start + k) * *b.get_unchecked(k * n + j);
                }
                *c.get_unchecked_mut(start + j) = summe;
            }
        }
    }
}
