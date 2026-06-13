# Hodge Theory

**hodge-theory** is a Rust library for representing and computing with Hodge diamonds — the bigraded structure encoding the complex geometry of Kähler manifolds. It provides constructors for canonical spaces (K3 surfaces, ℂℙⁿ), Hodge symmetry verification, and dimension computations.

## Why It Matters

Hodge theory is the bridge between topology and geometry — it refines the topological Betti numbers into the finer Hodge numbers *h^{p,q}*, revealing the complex structure of a manifold. The Hodge diamond of a K3 surface (*h^{1,1} = 20*) encodes the existence of 20 independent complex curves, a fact that connects algebraic geometry to string theory compactifications. In physics, Hodge numbers determine the number of moduli in Calabi-Yau compactifications; in mathematics, they constrain which manifolds can admit Kähler metrics. This library makes these structures computable and verifiable.

## How It Works

### The Hodge Diamond

For a compact Kähler manifold *X* of complex dimension *n*, the Hodge cohomology groups *H^{p,q}(X)* are the *(p,q)*-components of the de Rham cohomology under the Hodge decomposition:

```
H^k(X, ℂ) = ⊕_{p+q=k} H^{p,q}(X)
```

The **Hodge diamond** arranges *h^{p,q} = dim H^{p,q}(X)* in a diamond grid:

```
            h^{0,0}
         h^{1,0}  h^{0,1}
      h^{2,0}  h^{1,1}  h^{0,2}
   h^{3,0}  h^{2,1}  h^{1,2}  h^{0,3}
      ...
```

### Hodge Symmetry

Serre duality and complex conjugation impose:

```
h^{p,q} = h^{q,p}      (complex conjugation)
h^{p,q} = h^{n-p,n-q}  (Serre duality)
```

The `check_symmetry()` method verifies the first condition: for every *(p,q)* in the diamond, *h^{q,p}* must equal *h^{p,q}*.

### Canonical Examples

**K3 Surface** (complex dimension 2, weight 2):

```
            1
         0     0
      1    20    1
         0     0
            1
```

The total dimension is *1 + 20 + 1 + 1 + 1 = 24*, matching the topological Euler characteristic.

**ℂℙⁿ** (complex projective space): *h^{p,p} = 1* for *p = 0, ..., n*, all others zero. The diamond is a diagonal line of 1's.

### Computations

| Space | Diamond Entries | Total Dimension |
|-------|----------------|-----------------|
| ℂℙ¹ | h⁰⁰ = h¹¹ = 1 | 2 |
| ℂℙ² | h⁰⁰ = h¹¹ = h²² = 1 | 3 |
| K3 | h⁰⁰ = h⁰² = h²⁰ = h²² = 1, h¹¹ = 20 | 24 |

## Quick Start

```rust
use hodge_theory::{k3_hodge_diamond, cpn_hodge_diamond, HodgeDiamond};

fn main() {
    // K3 surface
    let k3 = k3_hodge_diamond();
    println!("K3 Hodge diamond:");
    println!("  h^{{0,0}} = {}", k3.get(0, 0));
    println!("  h^{{1,1}} = {}", k3.get(1, 1));
    println!("  Total dimension: {}", k3.total_dimension());

    assert!(k3.check_symmetry());    // h^{p,q} = h^{q,p}
    assert_eq!(k3.total_dimension(), 24);

    // Complex projective space CP^3
    let cp3 = cpn_hodge_diamond(3);
    assert!(cp3.check_symmetry());
    assert_eq!(cp3.total_dimension(), 4); // h^{0,0} + h^{1,1} + h^{2,2} + h^{3,3}

    // Build a custom Hodge diamond
    let mut custom = HodgeDiamond::new(2);
    custom.set(0, 0, 1);
    custom.set(1, 1, 5);
    custom.set(2, 2, 1);
    custom.set(1, 0, 0);
    custom.set(0, 1, 0); // maintain symmetry
    assert!(custom.check_symmetry());
}
```

## API

### `HodgeDiamond`
- `new(weight: i32) → HodgeDiamond` — Create empty diamond of given weight
- `set(p, q, dim)` — Set h^{p,q}
- `get(p, q) → u32` — Read h^{p,q} (0 if unset)
- `check_symmetry() → bool` — Verify h^{p,q} == h^{q,p} for all entries
- `total_dimension() → u32` — Sum of all h^{p,q}

### Constructors
- `k3_hodge_diamond() → HodgeDiamond` — The K3 surface
- `cpn_hodge_diamond(n: i32) → HodgeDiamond` — ℂℙⁿ

## Architecture Notes

This crate provides the algebraic geometry layer for SuperInstance's topological data analysis. Hodge numbers characterize the moduli spaces used in the γ + η = C framework: the gamma layer uses topological invariants to classify data manifolds, while the eta layer uses Hodge decomposition to decompose differential forms on those manifolds.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for the full design.

## References

- Hodge, W. V. D. (1941). *The Theory and Applications of Harmonic Integrals*. Cambridge University Press.
- Griffiths, P., & Harris, J. (1978). *Principles of Algebraic Geometry*. Wiley. Chapter 0.
- Barth, W., Hulek, K., Peters, C., & Van de Ven, A. (2004). *Compact Complex Surfaces* (2nd ed.). Springer. §VIII on K3 surfaces.

## License

MIT
