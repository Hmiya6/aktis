
# aktis text browser

HTML を Markdown として表示する CLI のテキストブラウザです.  
ブラウザとしての最小構成を目指しています.  

[EN]  
A CLI text browser, which renders HTML as Markdown file.  
Aktis contains only minimal features as browser.

## Features
* HTTP プロトコルのみをサポート
* GET メソッドのみをサポート
* ホスト OS の DNS リゾルバを使用 (自作 DNS でない)
* ホスト OS の TCP ソケットを使用 (自作 TCP/IP スタックでない)
* シンプルな HTML パーサを実装
* 最低限の外部クレートのみを使用 (現状では不使用)

[EN]  
* supports only HTTP protocol
* supports only GET method
* uses host OS's DNS resolver
* uses host OS's TCP socket
* has simple HTML parser
* uses minimum (or no) external crate


## TODO
* HTML パーサを実装する
* UI を実装する
* エラーハンドリングを行う. (現在は `unwrap()` でパニックさせている)
* 可読性の向上させる
* 他のメソッドを実装する 

[EN]  
* Add HTML parser
* Add user interface
* Add proper error handling
* Make the codes more readable
* Add other methods (such as POST)
