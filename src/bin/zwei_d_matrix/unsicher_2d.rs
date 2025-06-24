/*
    unsafe

    get_unchecked verhindert den Zugriffscheck 
    --> Der Compiler kann nicht garantieren dass die Indizes innerhalb der Grenzen sind
    --> Sicherheit ist nicht garantiert
    --> unsafe notwendig

    falsche Indizes führt also zu undefiniertem Verhalten oder Programmabsturz
*/
pub fn unsicher_2d(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>, n: usize)  {
    unsafe {
        for i in 0..n {
            for j in 0..n {
                let mut summe = 0.0;
                for k in 0..n {
                    summe = summe + *a.get_unchecked(i).get_unchecked(k) * *b.get_unchecked(k).get_unchecked(j);
                }
                *c.get_unchecked_mut(i).get_unchecked_mut(j) = summe;
            }
        }
    }
}
