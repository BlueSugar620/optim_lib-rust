# Wavelet Matrix
Wavelet Matrixです。`data_structure/static_bit_vec`を利用します。

## 定義
以下の操作を高速にします。
$(A_i)_{i = 0}^{N - 1}$
を与えられた非負整数列とします。

### 構築 ( `new(b, a)` )
Wavelet Matrixを構築します。
このとき、
$\max A \le 2 ^b$
が保証されているとします。
計算時間は、
$O(bN)$
となります。

### アクセス ( `get_at(i)` )
$A_i$
を返します。
計算時間は、 
$O(b)$
です。

### ランク (`rank(range, v)`)
$0 \le l \le r \le N, v \ge 0$
に対して、

$$
\# \lbrace i \in [l, r): A_i = v \rbrace |
$$

を計算します。計算時間は、 $O(b)$ となります。

### 第k最小値 ( `quantile(range, k)` )
$0 \le l \le r \le N, k \in \mathbb{N}$
に対して、 
$\lbrace A_i : i \in [l, r) \rbrace$
のなかで
$k$
番目に小さなものを出力します。
計算時間は、
$O(b)$
です。

## todo
他にも様々な機能を追加することができますが、技術不足により実装できませんでした。
