# Potentialized UnionFind
重み付きUnionFindです。

## 定義
以下の操作を行うことができるデータ構造です。
ここで 
$S$
を集合とし、
$(G, +)$
を可換群とします。 

$\mathfrak{U} = \lbrace \lbrace s \rbrace : s \in S \rbrace$
と初期化します。
関数
$D: S \to G$
を
$D = 0$
と初期化します。

操作より、常に以下が成り立つことが保証されます。

$$
\forall u \in S, \exists! U \in \mathfrak{U}, u \in U
$$

これより
$u$
に対する
$U$
を
$T(u)$
とする。

### 合併 ( `unite(u, v, d)` ) 
$u, v \in S, d \in G$
に対して、 

$$
\mathfrak{U} \to \mathfrak{U} - \{ T(u), T(v) \} + \{ T(u) \cup T(v) \}
\\
D(v) - D(u) = d
$$

とします。つまり、 
$u, v$
を距離
$d$
でマージします。
計算時間は
$O(\log \lvert S \rvert)$
です。

もし、すでに
$D(v) - D(u) \neq d$
となっている場合、
`false`
を返し操作を行いません。
そうでなく正しく操作を終えた場合は、
`true`
を返します。

### 距離 ( `dist(u, v)` )
$u, v \in S$
に対して、 
$D(v) - D(u)$
を返します。
計算時間は、
$O( \log \lvert S \rvert )$
です。

もし、 
$T(u) \neq T(v)$
、つまり
$u, v$
間の距離が未定義の場合は
`None`
を返します。

### 要素の大きさ ( `size(u)` )
$u \in S$
に対して、
$\lvert T(u) \rvert$
を返します。
計算時間は、
$O( \log \lvert S \rvert )$
です。

### 集合の大きさ ( `count()` )
$\lvert \mathfrak{U} \rvert$
を返します。
計算時間は、
$O(1)$
です。

