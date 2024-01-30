#![allow(dead_code)]
use std::usize;

#[derive(Debug, PartialEq)]

enum Kort {
    Ess,
    To,
    Tre,
    Fire,
    Fem,
    Seks,
    Syv,
    Åtte,
    Ni,
    Knekt,
    Dame,
    Konge,
}

struct Hånd {
    kortene: Vec<Kort>,
}

impl Hånd {
    fn new() -> Self {
        Hånd { kortene: vec![] }
    }

    fn leggtil(&mut self, kort: Kort) {
        self.kortene.push(kort);
    }
    fn verdi(&self) -> usize {
        use Kort::*;

        let mut subtotal = 0;
        let mut ess_sett = 0;

        for kort in self.kortene.iter() {
            subtotal += match kort {
                Ess => {
                    ess_sett += 1;
                    0
                }
                To => 2,
                Tre => 3,
                Fire => 4,
                Fem => 5,
                Seks => 6,
                Syv => 7,
                Åtte => 8,
                Ni => 9,
                Knekt => 10,
                Dame => 10,
                Konge => 10,
            };
        }
        for _ in 0..ess_sett {
            let ess_verdi = if subtotal <= 10 { 11 } else { 1 };
            subtotal += ess_verdi;
        }
        subtotal
    }
    fn er_tapp_hånd(&self) -> bool {
        self.verdi() > 21
    }
}

fn main() {
    let mut hånd = Hånd::new();
    hånd.leggtil(Kort::Konge);
    hånd.leggtil(Kort::Ess);
}

#[test]
fn tom_hånd() {
    let hånd = Hånd::new();
    assert_eq!(hånd.verdi(), 0);
}

#[test]
fn sterk_hånd() {
    let mut hånd = Hånd::new();
    hånd.leggtil(Kort::Dame);
    hånd.leggtil(Kort::Ess);
    assert_eq!(hånd.verdi(), 21);
}

#[test]
fn risikabel_hånd() {
    let mut hånd = Hånd::new();
    hånd.leggtil(Kort::Konge);
    hånd.leggtil(Kort::Dame);
    hånd.leggtil(Kort::Ess);
    assert_eq!(hånd.verdi(), 21);
}

#[test]
fn oops() {
    let mut hånd = Hånd::new();
    hånd.leggtil(Kort::Konge);
    hånd.leggtil(Kort::Syv);
    hånd.leggtil(Kort::Fem);
    assert!(hånd.er_tapp_hånd());
    assert_eq!(hånd.verdi(), 22);
}
