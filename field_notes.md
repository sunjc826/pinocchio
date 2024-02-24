Note that currently, 
```rust
    type GroupElement = <E1 as Engine>::GE;
    type FieldElement = <GroupElement as Group>::Scalar;
```
`FieldElement` resolves to `Fr` (see halocurves crate `bn256::fr`).

By my basic understanding, `Fr` is a prime field $\mathbb{F}_r$, where $r$ is some large prime. (Note, we're not raising $r$ to any power here.) The underlying representation uses montgomery form, presumably for efficient computation. However, since montgomery form is a bijection between $\{x\mod r : x\}$ and $\{x\cdot 2^{256} \mod r : x\}$, when we use a method like `to_le_bits` on `FieldElement::from(x)`, we can always recover the original bitwise representation of $x \mod r$.