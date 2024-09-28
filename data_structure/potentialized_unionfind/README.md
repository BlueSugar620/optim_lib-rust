# Potentialized UnionFind

以下の操作を行うことができるデータ構造です。
ここで $S$ を集合とし、 $(G, +)$ を可換群とします。 

$\mathfrak{U} = \{ \{ s \} : s \in S \}$ と初期化します。
関数 $D: S \to G$ を $D = 0$ と初期化します。

以下、一般に変数`from, to`を $u, v$ とします。


- $u, v \in S, d \in G$ に対して、 $u \in U \in \mathfrak{U}, v \in V \in \mathfrak{U}$ が存在して $\mathfrak{U} = \mathfrak{U} - \{ U, V \} + \{ U \cup V \}, D(v) - D(u) = d$ とします。つまり、 $U, V$ を距離 $d$ でマージします。計算時間は $O(\log |S|)$ です。もし、すでに $D(v) - D(u) \neq d$ となっている場合、`false`を返します。

- $u, v \in S$ に対して、 $\exists U, \{ u, v \} \in U \in \mathfrak{U}$ を判定し、 $D(v) - D(u)$ を返します。計算時間は $O( \log |S| )$ です。

- $u \in S$ に対して、 $u \in U \in \mathfrak{U}$ が存在して $|U|$ を返します。計算時間は $O( \log |S| )$ です。

- $|\mathfrak{U}|$ を返します。 $O(1)$ です。

ここで、操作より常に $\forall s \in S, \exists! U \in \mathfrak{U}, s \in U$ が成り立ちます。

## Note

`Op`の実装テンプレートです。
```rust
enum O {}
impl Op for O {
    ...
}
```
