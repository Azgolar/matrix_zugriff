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

    // erzeugt einen Zufallswert des jeweiligen Datentyps
    fn zufallswert(anzahl: usize) -> Vec<Self>;
} 

// Implemtierung des Traits für unsigned 32 bit 
impl Zahlentyp  for u32 {
    fn type_name() -> &'static str{"u32"}

    fn zufallswert() -> Self {
        random_range(1..9)
    }
}

// Implemtierung des Traits für unsigned 64 bit 
impl Zahlentyp  for u64 {
    fn type_name() -> &'static str{"u64"}

    fn zufallswert() -> Self {
        random_range(1..99)
    }
}

// Implemtierung des Traits für signed 32 bit 
impl Zahlentyp  for i32 {
    fn type_name() -> &'static str{"i32"}

    fn zufallswert() -> Self {
        random_range(1..9)
    }
}

// Implemtierung des Traits für signed 64 bit 
impl Zahlentyp  for i64 {
    fn type_name() -> &'static str{"i64"}

    fn zufallswert() -> Self {
        random_range(1..99)
    }
}

// Implemtierung des Traits für float 32 bit 
impl Zahlentyp  for f32 {
    fn type_name() -> &'static str{"f32"}

    fn zufallswert() -> Self {
        random_range(-1.0..1.0)
    }
}

// Implemtierung des Traits für float 64 bit 
impl Zahlentyp  for f64 {
    fn type_name() -> &'static str{"f64"}

    fn zufallswert() -> Self {
        random_range(-1.0..1.0)
    }
}
