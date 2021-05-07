
# aktis work log

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

# Log

## 独自エラーのエラーハンドリング
独自エラーに `std::error::Error` トレイトを実装して、 `Result<T, Box<dyn Error>>` でまとめて返す.

## 
