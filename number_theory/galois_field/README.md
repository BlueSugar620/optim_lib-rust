# GaloisField
有限体を扱います。以下、変数 `MOD` を $p$ と表記します。

有限体 $\mathbb{F}_p = \mathbb{Z} / p \mathbb{Z}$ の四則演算を $O(1)$ で行います。
また、以下の操作を高速に行います。

- $a \in \mathbb{F}_p, m \in \mathbb{Z}$ に対して、 $a^m$ を $O(\log m)$ で計算します。
- $a \in \mathbb{F}_p$ に対して、 $a^{-1}$ を $O(\log p)$ で計算します。


