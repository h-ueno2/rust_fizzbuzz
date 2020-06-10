# 本リポジトリについて

本リポジトリはRust言語の学習のために作成したリポジトリです。  
Rustで作成したFizzBuzzゲームです。  
Qiita記事：[【Rust】チュートリアルが終わったのでFizzBuzzを試しに書いてみた。](https://qiita.com/h-ueno2/items/3a520871009ac836e351)

## 実行方法

以下で実行できます。
```shell:コマンド例
cargo run
```
```shell:実行例
1
2
Fizz
4
Buzz
6
// -- 省略 --
```

また、コマンドライン引数によって以下を設定できます。
* FizzBuzzを続ける最大値。省略時には`100`となります。
* Fizzとなる基底値。省略時には`3`となります。
* Buzzとなる基底値。省略時には`5`となります。

```shell:コマンド例
cargo run 10 2 3
```

```shell:実行例
1
Fizz
Buzz
Fizz
5
FizzBuzz
7
Fizz
Buzz
Fizz
```
