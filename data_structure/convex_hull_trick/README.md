# Convex Hull Trick
直線の集合に対して、ある
$x$
に対するその最小値を求めます。


## 定義
以下の操作を高速にするためのデータ構造です。

まず、 
$S = \lbrace \rbrace$
と初期化します。
$M$
を可換環として、全順序が入っているものとします。
現状は、 
$M = \mathbb{Z}$
の場合のみの使用とします。

### 直線の追加 ( `insert((a, b))` )
$a, b \in M$
に対して、 
$S \to S \cup \lbrace ax + b \rbrace$
と更新します。
計算時間は、 
$O(\log \lvert S \rvert)$
です。

### 最小値を求める ( `min(x)` )
$\alpha \in M$
に対して、
$\min \lbrace a \alpha + b \in S\rbrace$
を計算します。
計算時間は、 
$O(\log \lvert S \rvert)$
です。

##  使用方法
`Op`の実装例が、`i64_op.rs`にあります。

