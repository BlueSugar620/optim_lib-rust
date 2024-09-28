# Segment Tree

セグメント木です。

$(M, \cdot)$ をモノイドの集合とします。
$A = (A_i)_{i = 0}^{N - 1} \subset M$ とします。以下の操作を高速でします。

- $i \in [0, N), x \in M$ に対して、 $A_i = x$ と更新します。計算時間は、 $O(\log N)$ です。

- $0 \le l \le r \le N$ に対して、 $\prod_{i = l}^{r - 1} A_i$ を計算します。計算時間は、 $O(\log N)$ です。

- 現在の $A$ を出力します。計算時間は、 $O(N)$ です。

## Note

モノイドを用意するときのテンプレートです。

```rust
enum O {}
impl Monoid for O {
    ...
}
```
