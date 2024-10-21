# Longest Increasing Subsequece
最長増加部分列を返します。

## 定義
### 構築 ( `lis(a, strict)` )
数列
$a = (a)_{i = 0}^{N - 1}$
の最長増加部分列を返します。
`strict`
によって、広義とするか狭義とするかを選択することができます。
計算時間は、
$O(N \log N)$
です。
