/*
    Erzeugt einen Vektor im Bereich [anfang, ende]
    - Die Schrittweite wird adaptiv größer
    - Matrixgrößen mit 2^x Potenzen sind immer enthalten
*/
pub fn erzeugen(anfang: usize, ende: usize) -> Vec<usize> {
    let mut liste: Vec<usize> = Vec::new();

    liste.push(anfang);

    let mut letztes: usize  = anfang as usize;

    // nächste Zweierpotenz
    let mut zweier: usize = 2;

    // Schrittweite festlegen
    while letztes < ende {
        let schritt: usize = match letztes {
            2..=9       => 4,
            10..=99     => 6,
            100..=999   => 100,
            1000..=9999 => 500,
            _           => 1000  
        };

        let aktuell: usize = letztes + schritt;

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