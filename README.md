# Axum & SQLite template

Hello there!

This is a humble project template for [Boilermaker](https://github.com/yeajustmars/boilermaker).

This template creates a basic Rust project for a small web service with the following flavor:

- Tokio for async,
- Axum web server, and
- SQLx + SQLite for persistence.

You should always find the latest version at: https://github.com/oz/boil-axum-sqlite

# Usage

Install the template with:

```
boil install https://github.com/oz/boil-axum-sqlite
```

Now, you can create a project with:

```
$ boil new -r veryweb --var package_name=veryweb boil-axum-sqlite
(...)
$ cd veryweb
$ cargo run
```
