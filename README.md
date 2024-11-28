# Spire

此项目是对[ebou](https://github.com/terhechte/ebou)的学习性重写。

`Spire`是一个跨平台的[Mastodon](https://github.com/mastodon/mastodon)的GUI界面程序。

## 开发指南

此项目使用[dioxus](https://github.com/DioxusLabs/dioxus)+[tailwindcss](https://github.com/tailwindlabs/tailwindcss)的开发框架。

- `cargo make tailwind` 用于开启`tailwindcss`服务。
- `cargo make dev` 用于开启`dioxus`服务。

两者开启后可以在 http://localhost:3000 查看应用程序。

## 架构

- [megalodon-rs](https://docs.rs/megalodon/latest/megalodon/) 作为主要的`Mastodon`API。
