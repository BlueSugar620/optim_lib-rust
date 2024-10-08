# Strongly Connected Component 

与えられた有向グラフを強連結成分分解します。
それを、強連結成分ごとにトポロジカルソートをします。
計算時間は、 $O(|V| + |E|)$ です。

## 細かい仕様
与えられた有向グラフを $G = (V(G), E(G))$ とします。
以下の条件を満たす列 $(V_i)_{i = 0}^{K - 1}$ を出力します。

- $\forall v \in V(G), \exists! i, v \in V_i$
- $\forall i, \forall v, w \in V_i, v \ から \ w \ へのパスが存在する$
- $\forall (v, w) \in E(G), v \in V_i, w \in V_j \implies i \le j$


