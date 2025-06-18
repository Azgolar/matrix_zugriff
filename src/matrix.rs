use rand::random_range;
use crate::zahlentyp::Zahlentyp;

pub fn zufallsmatrix<T: Zahlentyp>(n: usize) -> Vec<Vec<T>> {
    let mut matrix: Vec<Vec<T>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = random_range
        }
    }
}

/*
    Erzeugt einen Vektor im Bereich [anfang, ende]
    - Die Schrittweite wird adaptiv größer
    - Matrixgrößen mit 2^x Potenzen sind immer enthalten
*/
fn n_erzeugen(anfang: u32, ende: u32) -> Vec<u32> {
    let mut liste: Vec<u32> = Vec::new();

    liste.push(anfang);

    let mut letztes: u32 = anfang;

    // nächste Zweierpotenz
    let mut zweier: u32 = 4;

    // Schrittweite festlegen
    while letztes < ende {
        let schritt: u32 = match letztes {
            2..=9       => 4,
            10..=99     => 6,
            100..=999   => 100,
            1000..=9999 => 500,
            _           => 1000  
        };

        let aktuell: u32 = letztes + schritt;

        // Falls nötig Zweierpotenz hinzufügen
        if zweier > letztes && zweier <= ende && zweier < aktuell {
            liste.push(zweier);
            zweier = zweier * 2;
        }

        // Prüfen ob obere Grenze überschritten wurde
        if aktuell >= ende {
            // Ende erreicht
            liste.push(ende);
            break;
        }
        else {
            // Ende noch nicht erreicht
            liste.push(aktuell);
            letztes = aktuell;
        }
    }
    liste
}