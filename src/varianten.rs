use crate::zahlentyp::Zahlentyp;

/*
    Standardimplementation
*/
pub fn basis<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize) {
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
    Standardimplementation mit assert
*/
pub fn assert<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize) {

    assert_eq!(a.len(), n);
    assert_eq!(b.len(), n);
    assert_eq!(c.len(), n);
    for i in 0..n {
        assert_eq!(a[i].len(), n);
        assert_eq!(b[i].len(), n);
        assert_eq!(c[i].len(), n);
    }

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
    Standardimplementation mit Slice
*/
pub fn slice<T: Zahlentyp>(a: &[Vec<T>], b: &[Vec<T>], c: &mut [Vec<T>], n: usize) {


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
pub fn mit_iteartor<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize) {
    /*
        Es werden die ersten n Zeilen durchlaufen, dabei gilt (zeilenindex, Referenz auf aktuelle Zeile)
    */
    for (i, zeile) in a.iter().enumerate().take(n) {
        /*
            Schleife über die Spalten von c, dabei gilt (spaltenindex, Referenz auf aktuelle Element in c)
         */
        for (j, ergebnis) in c[i].iter_mut().enumerate().take(n) {
            let mut summe = T::default();
            for k in 0..n {
                summe = summe + zeile[k] * b[k][j];
            }
            *ergebnis = summe;
        }
    }
}

/*
    1D Vektor im Zeilenformat
*/
pub fn ein_dimensional_zeilenformat<T: Zahlentyp>(a: &Vec<T>, b: &Vec<T>, c: &mut Vec<T>, n: usize) {
    for i in 0..n {
        // startindex der i-ten Zeile
        let start: usize = i * n;
        for j in 0..n {
            let mut summe = T::default();
            for k in 0..n {
                summe = summe + a[start + k] * b[k * n + j];
            }
            c[start + j] = summe;
        }
    }
}

/*
    1D Vektor im Spaltenformat
*/
pub fn ein_dimensional_spaltenformat<T: Zahlentyp>(a: &Vec<T>, b: &Vec<T>, c: &mut Vec<T>, n: usize) {
    for i in 0..n {
        for j in 0..n {
            // startindex der j-ten Spalte
            let start: usize = j * n;
            let mut summe = T::default();
            for k in 0..n {
                summe = summe + a[k * n + i] * b[start + k];
            }
            c[start + i] = summe;
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
pub fn teilen<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize) {
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
pub fn unsicher<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize)  {
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