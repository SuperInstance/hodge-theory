//! Hodge Theory
//! Structures from Hodge decomposition and harmonic forms.

use std::collections::BTreeMap;

/// A bigraded piece of a Hodge diamond.
#[derive(Debug, Clone)]
pub struct HodgeDiamond {
    /// Map from (p,q) to dimension.
    pieces: BTreeMap<(i32, i32), u32>,
    weight: i32,
}

impl HodgeDiamond {
    pub fn new(weight: i32) -> Self {
        Self {
            pieces: BTreeMap::new(),
            weight,
        }
    }

    pub fn set(&mut self, p: i32, q: i32, dim: u32) {
        self.pieces.insert((p, q), dim);
    }

    pub fn get(&self, p: i32, q: i32) -> u32 {
        *self.pieces.get(&(p, q)).unwrap_or(&0)
    }

    /// Check Hodge symmetry: h^{p,q} == h^{q,p}.
    pub fn check_symmetry(&self) -> bool {
        for (&(p, q), &dim) in &self.pieces {
            if self.get(q, p) != dim {
                return false;
            }
        }
        true
    }

    /// Total Hodge number sum.
    pub fn total_dimension(&self) -> u32 {
        self.pieces.values().sum()
    }
}

/// Build a K3 surface Hodge diamond.
pub fn k3_hodge_diamond() -> HodgeDiamond {
    let mut h = HodgeDiamond::new(2);
    h.set(0, 0, 1);
    h.set(1, 0, 0); h.set(0, 1, 0);
    h.set(2, 0, 1); h.set(1, 1, 20); h.set(0, 2, 1);
    h.set(2, 1, 0); h.set(1, 2, 0);
    h.set(2, 2, 1);
    h
}

/// Build a CP^n Hodge diamond.
pub fn cpn_hodge_diamond(n: i32) -> HodgeDiamond {
    let mut h = HodgeDiamond::new(n);
    for p in 0..=n {
        h.set(p, p, 1);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k3() {
        let k3 = k3_hodge_diamond();
        assert!(k3.check_symmetry());
        assert_eq!(k3.total_dimension(), 24);
    }

    #[test]
    fn test_cp2() {
        let cp2 = cpn_hodge_diamond(2);
        assert!(cp2.check_symmetry());
        assert_eq!(cp2.total_dimension(), 3);
    }
}
