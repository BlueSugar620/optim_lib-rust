# Strongly Connected Component 

与えられた有向グラフを強連結成分分解します。
それを、強連結成分ごとにトポロジカルソートをします。

## 定義
有向グラフ
$G = (V, E)$
の強連結成分分解とは、

$$
\lbrace U \subset V : \forall u, v \in U, d(u, v) \lt \infty \rbrace
$$

を指します。
さらに頂点を上のように縮約するとDAGとなり、そのトポロジカルソートを返します。

### 強連結成分分解 ( `scc(n, e)` )
強連結成分分解をし、そのトポロジカルソートを返します。
計算時間は、 
$O(\lvert V \rvert + \lvert E \rvert)$
です。
