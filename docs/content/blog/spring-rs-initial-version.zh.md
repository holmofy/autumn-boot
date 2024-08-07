+++
title = "spring-rs初始版本发布"
description = "Introducing Doks, a Hugo theme helping you build modern documentation websites that are secure, fast, and SEO-ready — by default."
date = 2024-08-04T09:19:42+00:00
updated = 2024-08-04T09:19:42+00:00
draft = false
template = "blog/page.html"

[extra]
lead = "经过一个月的沉淀，我用rust编写了一个类似于spring-boot的微服务框架。下面是一个最简单的web应用的例子"
+++

```rust
use spring::{route, get, App};
use spring_web::{
    extractor::Path, handler::TypeRouter, response::IntoResponse, 
    Router, WebConfigurator, WebPlugin,
};

#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(WebPlugin)
        .add_router(router())
        .run()
        .await
}

fn router() -> Router {
    Router::new()
        .typed_route(hello_word)
        .typed_route(hello)
}

#[get("/")]
async fn hello_word() -> impl IntoResponse {
    "hello word"
}

#[route("/hello/:name", method = "GET", method = "POST")]
async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("hello {name}")
}
```

`spring-rs`使用插件的方式整合了rust生态中流行的几个框架，并提供了过程宏来简化开发。

对`spring-rs`感兴趣的可以[点击这里](/docs/)快速上手。
