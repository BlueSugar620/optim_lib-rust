# Unionfind

以下の操作を行うことができるデータ構造です。ここで $S$ を集合とします。 $\mathfrak{U} = \{ \{ s \} : s \in S \}$ と初期化します。


- $u, v \in S$ に対して、 $u \in U \in \mathfrak{U}, v \in V \in \mathfrak{U}$ が存在して $\mathfrak{U} = \mathfrak{U} - \{ U, V \} + \{ U \cup V \}$ とします。つまり、 $U, V$ をマージします。計算時間は $O(\log |S|)$ です。

- $u, v \in S$ に対して、 $\exists U, \{ u, v \} \in U \in \mathfrak{U}$ を判定します。計算時間は $O( \log |S| )$ です。

- $u \in S$ に対して、 $u \in U \in \mathfrak{U}$ が存在して $|U|$ を返します。計算時間は $O( \log |S| )$ です。

- $|\mathfrak{U}|$ を返します。 $O(1)$ です。

ここで、操作より常に $\forall s \in S, \exists! U \in \mathfrak{U}, s \in U$ が成り立ちます。
