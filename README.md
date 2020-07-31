# tide_html_server_sample

## dependencies

[dependencies]  
tide = "0.12"  
tinytemplate = "1.1"  
serde = "1.0"

[dependencies.async-std]  
version = "1.6"  
features = ["attributes"]

## what is this?

(jp)  
tideを使ってhtmlファイルとしてResponseを作成するサンプル  
(en)  
Sample that make response as html file using tide.

## how to use

1. リポジトリのクローン / Clone this repository
```sh
git clone https://github.com/yusuke-ota/tide_html_server_sample.git
```

2. 実行 / cargo run
```sh
cd tide_html_server_sample
cargo run
```

3. サーバにアクセス / Access to this html server
```sh
# return string "hello"
curl http://localhost:8080/

# return static html file
curl http://localhost:8080/hello_html

# return html file generated by template engin
curl http://localhost:8080/html_template/[string]

# return static html file notice not found
curl http://localhost:8080/something
```
