# Dotenv codegen and `rustc 1.35.0 (3c235d560 2019-05-20)`

Building forever (and so does [RLS](https://github.com/rust-lang/rls) under VSCode editor):

```bash
dotenv-codegen-rustc-hang romanvg$ cargo build
   Compiling dotenv-codegen-rustc-hang v0.1.0 (/Users/romanvg/tmp/dotenv-codegen-rustc-hang)
^C  Building [======================================================>  ] 39/40: dotenv-codegen-rustc-hang(bin)
```

Tested on all those toolchain versions:

```bash
$ rustup show
Default host: x86_64-unknown-linux-gnu

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu (default)
1.30.0-x86_64-unknown-linux-gnu
1.31.0-x86_64-unknown-linux-gnu
1.33.0-x86_64-unknown-linux-gnu
1.34.0-x86_64-unknown-linux-gnu

active toolchain
----------------

nightly-x86_64-unknown-linux-gnu (default)
rustc 1.37.0-nightly (d3e2cec29 2019-06-26)
```

# rustc --verbose

```bash
$ cargo build --verbose
       Fresh unicode-xid v0.1.0
       Fresh cc v1.0.37
       Fresh lazy_static v1.3.0
       Fresh cfg-if v0.1.9
       Fresh rustc-demangle v0.1.15
       Fresh ucd-util v0.1.3
       Fresh utf8-ranges v1.0.3
       Fresh thread_local v0.3.6
       Fresh proc-macro2 v0.4.30
       Fresh regex-syntax v0.6.7
       Fresh libc v0.2.58
       Fresh memchr v2.2.0
       Fresh quote v0.6.12
       Fresh backtrace-sys v0.1.29
       Fresh aho-corasick v0.7.3
       Fresh syn v0.15.38
       Fresh backtrace v0.3.32
       Fresh regex v1.1.7
       Fresh synstructure v0.10.2
       Fresh proc-macro-hack v0.5.7
       Fresh failure_derive v0.1.5
       Fresh failure v0.1.5
       Fresh dotenv v0.14.1
       Fresh dotenv_codegen_implementation v0.14.1
       Fresh dotenv_codegen v0.14.1
   Compiling dotenv-codegen-rustc-hang v0.1.0 (/Users/romanvg/tmp/dotenv-codegen-rustc-hang)
     Running `rustc --edition=2018 --crate-name dotenv_codegen_rustc_hang src/main.rs --color always --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=1e9d1a17a5002a44 -C extra-filename=-1e9d1a17a5002a44 --out-dir /Users/romanvg/tmp/dotenv-codegen-rustc-hang/target/debug/deps -C incremental=/Users/romanvg/tmp/dotenv-codegen-rustc-hang/target/debug/incremental -L dependency=/Users/romanvg/tmp/dotenv-codegen-rustc-hang/target/debug/deps --extern dotenv_codegen=/Users/romanvg/tmp/dotenv-codegen-rustc-hang/target/debug/deps/libdotenv_codegen-69e80c3fd90e141b.rlib`
    Building [======================================================>  ] 39/40: dotenv-codegen-rustc-hang(bin)
```
