# Libsecret Rust bindings

These Rust bindings are generated using [gir](https://github.com/gtk-rs/gir). Current build instructions are listed below.

```
cd secret-sys && gir && cd ..
gir && cargo build
```

Note that these are not yet complete. The important schema_new and schema_newv are not working yet.