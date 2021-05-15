
# aktis work log
edited: 2021-05-15

---

# TODO

## 最小機能
- HTML パーサの追加
- Markdown ジェネレータの追加
- CUI の追加

## 追加機能
- 自作 TCP
- 自作 DNS
- POST メソッド
- (GUI)

## 改善
- エラーハンドリング
- コメントの追加
- `http::client::Client::get()` の整理
- URL パーサの改善
- HTTP レスポンス パーサの改善
- `Consumer` 名前が適切か否か考える

---

# Consumer について

Rust のイテレータが言語処理で使うには少し使いづらかった (1字ずつしか消費できないなど). 

# HTTP リクエスト

## 必須な要素
- 一番上の行 (リクエストライン, start line)
- `Host` フィールド (`HTTP/1.1` 以上) 

telnet で実験 (リクエストの送信先: [example.com](https://example.com))

### 1. リクエストラインのみを送信

400 Bad Request が返される

```sh
% telnet example.com 80
Trying 93.184.216.34...
Connected to example.com.
Escape character is '^]'.
GET / HTTP/1.1 # リクエストラインを入力

HTTP/1.1 400 Bad Request
Content-Type: text/html
Content-Length: 349
Connection: close
Date: Sat, 15 May 2021 05:48:04 GMT
Server: ECSF (mic/9B12)

# レスポンスボディ省略
Connection closed by foreign host.
```
### 2. リクエストライン + Host の送信

200 OK が返ってくる

```sh
% telnet example.com 80
Trying 93.184.216.34...
Connected to example.com.
Escape character is '^]'.
GET / HTTP/1.1 # リクエストラインを入力
Host: example.com # Host フィールドを入力

HTTP/1.1 200 OK
Age: 392303
Cache-Control: max-age=604800
Content-Type: text/html; charset=UTF-8
Date: Sat, 15 May 2021 05:48:48 GMT
Etag: "3147526947+ident"
Expires: Sat, 22 May 2021 05:48:48 GMT
Last-Modified: Thu, 17 Oct 2019 07:18:26 GMT
Server: ECS (mic/9A9C)
Vary: Accept-Encoding
X-Cache: HIT
Content-Length: 1256

# レスポンスボディ省略
```

### 参考
- [Host -HTTP|MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Host)

# HTTP レスポンスの解析

レスポンスの各行は `\r\n` で分割されるので、
1. 最初の `\r\n` まではステータスライン、
2. `\r\n\r\n` まではヘッダ
3. `\r\n\r\n` よりも後はレスポンスボディ
として分割できる.

# 独自エラーのエラーハンドリング
独自エラーに `std::error::Error` トレイトを実装して、 `Result<T, Box<dyn Error>>` でまとめて返す.

# HTML トークナイザの実装

最初はなんもわからんかった.

## 言語処理系をつくるときの情報収集
### 1. 言語仕様を調べる
HTML だったら、[HTML Living Standard](https://html.spec.whatwg.org/).  
言語仕様にはトークナイズやパースの手法が定められていることもある.

### 2. 他の実装を調べる
仕様がわかっても、実際どう実装するのかわからない場合、既存の実装を参考にするのがよい. すでに言語仕様から処理手法を理解していれば、既存の実装も理解しやすくなる.

今回は、Rust の実装だけでなく Go の実装を参考にした.  
Go 言語は
    - コード自体が読みやすく (文法が単純)、
    - かつ標準ライブラリや準標準ライブラリが充実しており、
    - さらに libc への依存がない (すべて Go で実装している)
ために参考になる. HTML パーサの場合、[golang/x/net/html](https://pkg.go.dev/golang.org/x/net/html) が参考になった.

## 状態機械によるトークナイズ

BNF (Buckus-Naur form) による解析しか経験がなかったため、よい実装方法がわからなかった.

今回は `enum` で状態を列挙して `match` で状態遷移しているが、それが状態機械の正しい実装方法なのか自信がない. Rust によるステートマシン実装方法としてトレイトオブジェクトを用いたものもあるらしい ([State パターン](https://qiita.com/mopp/items/3794dc955f7dc9d8ca63#state-%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B3)). 

```rust
// src/renderer/html_parser.rs

// `enum StateType` と `match` による状態機械
fn execute(&mut self) -> Vec<Token> {
    while self.con.peek_char().is_some() {
        // 状態の列挙 -> 遷移
        match self.state {
            StateType::Data => self.data(),
            StateType::TagOpen => self.tag_open(),
            StateType::EndTagOpen => self.end_tag_open(),
            StateType::TagName => self.tag_name(),
            StateType::BeforeAttributeName => self.before_attr_name(),
            StateType::AttributeName => self.attr_name(),
            StateType::BeforeAttributeValue => self.before_attr_val(),
            StateType::AttributeValueQuoted => self.attr_val_quoted(),
            StateType::AttributeValueUnquoted => self.attr_val_unquoted(),
            StateType::AfterAttributeValue => self.after_attr_val(),
            StateType::SelfClosingStartTag => self.self_closing_start_tag(),
            StateType::MarkupDeclarationOpen => self.markup_declaration_open(),
            StateType::Comment => self.comment(),
            StateType::Doctype => self.doctype(),
        }
    }
}
```

### 参考: 他の Rust デザインパターン
- [Rust風にデザインパターン23種](https://keens.github.io/blog/2017/05/06/rustkazenidezainpata_n23tane/)
- [Rust Design Patterns](https://github.com/lpxxn/rust-design-pattern)











