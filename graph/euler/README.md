# Euler 路
Euler 路を求めます。

## 定義
グラフを
$G = (V, E)$
とします。

オイラー路とは、長さ
$\lvert E \rvert$
の路であり、全ての辺をちょうど
$1$
回ずつ通るものです。

### オイラー路を返す ( `hierholzer(n, e)` )
オイラー路を返します。
オイラー路は頂点列と辺列の組で表されます。
存在しない場合は、
`None`
が返されます。
計算時間は、
$O(\lvert V \rvert + \lvert E \rvert)$
です。

モジュールとして、
`directed`
と
`undirected`
が用意されています、

