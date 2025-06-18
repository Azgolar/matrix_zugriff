use zahlentyp::Zahlentyp;

/*
    Variante mit der Standardimplementation
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
    Variante mit der Standardimplementation mit assert
*/
pub fn basis_assert<T: Zahlentyp>(a: &[Vec<T>], b: &[Vec<T>], c: &mut [Vec<T>], n: usize) {

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
    Variante mit Slice und assert
*/
pub fn slice<T: Zahlentyp>(a: &[Vec<T>], b: &[Vec<T>], c: &mut [Vec<T>], n: usize) {

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
    Variante mit Slice und assert
*/
pub fn slice_assert<T: Zahlentyp>(a: &[Vec<T>], b: &[Vec<T>], c: &mut [Vec<T>], n: usize) {

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
    Variante mit iterator
*/
pub fn iteartor<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize) {

    a.iter().enumerate().for_each(|i, zeile| {
        (0..n).for_each(|j| {
            let mut summe = T::default();
            zeile.iter().enumerate().for_each(|k, &wert| {
                summe = summe + wert * b[k][j];
            });
            c[i][j] = summe;
        });
    }); 
}

/*
    Variante mit iterator und assert
*/
pub fn iterator_assert<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize) {

    assert_eq!(a.len(), n);
    assert_eq!(b.len(), n);
    assert_eq!(c.len(), n);
    for i in 0..n {
        assert_eq!(a[i].len(), n);
        assert_eq!(b[i].len(), n);
        assert_eq!(c[i].len(), n);
    }

    a.iter().enumerate().for_each(|i, zeile| {
        (0..n).for_each(|j| {
            let mut summe = T::default();
            zeile.iter().enumerate().for_each(|k, &wert| {
                summe = summe + wert * b[k][j];
            });
            c[i][j] = summe;
        });
    }); 
}

/*
    Variante mit 1D Vektor
*/
pub fn einD<T: Zahlentyp>(a: &Vec<T>, b: &Vec<T>, c: &mut Vec<T>, n: usize) {
    for i in 0..n {
        let offset = i * n;
        for j in 0..n {
            let mut summe = T::default();
            for k in 0..n {
                summe = summe + a[offset + k] * b[k * n + j];
            }
            c[offset + j] = summe;
        }
    }
}

/*
    Variante mit 1D Vektor und assert
*/
pub fn einD_assert<T: Zahlentyp>(a: &Vec<T>, b: &Vec<T>, c: &mut Vec<T>, n: usize) {

    assert_eq!(a.len(), n * n);
    assert_eq!(b.len(), n * n);
    assert_eq!(c.len(), n * n);

    for i in 0..n {
        let offset = i * n;
        for j in 0..n {
            let mut summe = T::default();
            for k in 0..n {
                summe = summe + a[offset + k] * b[k * n + j];
            }
            c[offset + j] = summe;
        }
    }
}

/*
    Variante mit unsafe

    get_unchecked verhindert den Zugriffscheck bei Vektoren 
    --> Der Compiler kann nicht garantieren dass die Indizes innerhalb der Grenzen sind
    --> Sicherheit ist nicht garantiert
    --> unsafe Block notwendig

    falsche Indizes führe also zu undefiniertem Verhalten oder Programmabsturz.
*/
pub fn unsicher<T: Zahlentyp>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, c: &mut Vec<Vec<T>>, n: usize)  {
    unsafe {
        for i in 0..n {
            for j in 0..n {
                let mut summe = T::default();
                for k in 0..n {
                    summe = summe + a.get_unchecked(i).get_unchecked(k) * b.get_unchecked(k).get_unchecked(j);
                }
                *c.get_unchecked_mut(i).get_unchecked_mut(j) = summe;
            }
        }
    }
}