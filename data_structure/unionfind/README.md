# Unionfind
集合の非交差集合族を管理します。

## 定義
以下の操作を行うことができるデータ構造です。
ここで 
$S$
を集合とします。 
$\mathfrak{U} = \lbrace \lbrace s \rbrace : s \in S \rbrace$
と初期化します。

以下の操作は常に、以下が成り立ちます。

$$
\forall u \in S, \exists! U \in \mathfrak{U}, u \in U
$$

よって、このような 
$s$
に対する
$U$
を
$T(s)$
と書きます。

### マージ ( `unite(u, v)` )
$u, v \in S$
に対して、 
$\mathfrak{U} \to \mathfrak{U} - \lbrace T(u), T(v) \rbrace + \lbrace T(u) \cup T(v) \rbrace$
とします。
つまり、 
$U, V$
をマージします。
計算時間は 
$O(\log \lvert S \rvert)$
です。

もしすでに同一の集合である、つまり操作前の時点で
$T(u) = T(v)$
の時は、マージが行われなかったとして 
`false`
を返します。
無事マージが行われた時は、
`true`
を返します。

### 同一判定 ( `is_same(u, v)` )
$u, v \in S$ 
に対して、 
$T(u) = T(v)$
を判定します。
計算時間は 
$O( \log \lvert S \rvert )$
です。

### 要素の大きさ ( `size(u)` ) 
$u \in S$ 
に対して、 
$\lvert T(u) \rvert$
を返します。
計算時間は 
$O( \log \lvert S \rvert)$ です。

### 集合の大きさ ( `count()` )
$\lvert \mathfrak{U} \rvert$
を返します。
$O(1)$
です。


