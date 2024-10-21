# Minimum Spanning Tree (最小全域木)
最小全域木を求めます。

## 定義
無向グラフ
$G = (V, E)$
と、重み
$w: E \to \mathbb{R}$
に対して最小全域木とは、
$G$
の部分グラフかつ木
$H = (V, E')$
であり、
$\sum_{e \in E} w(e)$
が最小となるものである。

### 最小全域木の構築 ( `kruskal(n, e)` )
最小全域木をクラスカル法によって構築します。
外部ライブラリとして、
`unionfind`
が必要です。
存在しない時は、
`None`
を返します。
計算時間は、
$O(\lvert E \rvert \log \lvert V \lvert)$
です。

### 最小全域木の構築 ( `prim(n, e)` )
最小全域木をプリム法によって構築します。
存在しない時は、
`None`
を返します。
計算時間は、
$O(\lvert E \rvert \log \lvert E \rvert)$
です。
