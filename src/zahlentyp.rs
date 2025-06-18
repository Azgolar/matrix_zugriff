use std::ops::{Add, Mul};
use rand::random_range;

/*
    Trait der von allen Zahlentypen erfüllt sein muss:
    Copy: Werte werden nur per copy statt Move verwendet
    default: Standard Wert
    Add<Output = Self>: Das addieren zweier Zahlen eines Datentyps ergibt den gleichen Datentyp
    Mul<Output = Self>: Das multiplizieren zweier Zahlen eines Datentyps ergibt den gleichen Datentyp
*/
pub trait Zahlentyp: Copy + Default + Add<Output = Self> + Mul<Output = Self> {
    // gibt einen String mit Datentyp zurück
    fn type_name() -> &'static str;

    // erzeugt für eine Matriz alle benötigten Zufallswerte des jeweiligen Datentyps
    fn zufallswerte(anzahl: usize) -> Vec<Vec<Self>>;
} 

// Implemtierung des Traits für unsigned 32 bit 
impl Zahlentyp  for u32 {
    fn type_name() -> &'static str{"u32"}

    fn zufallswerte(anzahl: usize) -> Vec<Vec<Self>> {
        let mut zufall: Vec<Vec<u32>> = vec![vec![0; anzahl]; anzahl];
        for i in 0..anzahl {
            for j in 0.. anzahl {
                zufall[i][j] = random_range(1..=9);  
            }   
        }
        zufall
    }
}

// Implemtierung des Traits für unsigned 64 bit 
impl Zahlentyp  for u64 {
    fn type_name() -> &'static str{"u64"}

    fn zufallswerte(anzahl: usize) -> Vec<Vec<Self>> {
        let mut zufall: Vec<Vec<u64>> = vec![vec![0; anzahl]; anzahl];
        for i in 0..anzahl {
            for j in 0.. anzahl {
                zufall[i][j] = random_range(1..=99);  
            }   
        }
        zufall
    }
}

// Implemtierung des Traits für signed 32 bit 
impl Zahlentyp  for i32 {
    fn type_name() -> &'static str{"i32"}

    fn zufallswerte(anzahl: usize) -> Vec<Vec<Self>> {
        let mut zufall: Vec<Vec<i32>> = vec![vec![0; anzahl]; anzahl];
        for i in 0..anzahl {
            for j in 0.. anzahl {
                zufall[i][j] = random_range(1..=9);  
            }   
        }
        zufall
    }
}

// Implemtierung des Traits für signed 64 bit 
impl Zahlentyp  for i64 {
    fn type_name() -> &'static str{"i64"}

    fn zufallswerte(anzahl: usize) -> Vec<Vec<Self>> {
        let mut zufall: Vec<Vec<i64>> = vec![vec![0; anzahl]; anzahl];
        for i in 0..anzahl {
            for j in 0.. anzahl {
                zufall[i][j] = random_range(1..=99);  
            }   
        }
        zufall
    }
}

// Implemtierung des Traits für float 32 bit 
impl Zahlentyp  for f32 {
    fn type_name() -> &'static str{"f32"}

    fn zufallswerte(anzahl: usize) -> Vec<Vec<Self>> {
        let mut zufall: Vec<Vec<f32>> = vec![vec![0.0; anzahl]; anzahl];
        for i in 0..anzahl {
            for j in 0.. anzahl {
                zufall[i][j] = random_range(-1.0..=1.0);  
            }   
        }
        zufall
    }
}

// Implemtierung des Traits für float 64 bit 
impl Zahlentyp  for f64 {
    fn type_name() -> &'static str{"f64"}

    fn zufallswerte(anzahl: usize) -> Vec<Vec<Self>> {
        let mut zufall: Vec<Vec<f64>> = vec![vec![0.0; anzahl]; anzahl];
        for i in 0..anzahl {
            for j in 0.. anzahl {
                zufall[i][j] = random_range(-1.0..=1.0);  
            }   
        }
        zufall
    }
}
