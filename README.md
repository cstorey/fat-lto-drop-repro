# Reproduction example


```

: cez@ceri-no-mini; (set -x; cargo --version; rustc --version; git clean -xdf; cargo run; cargo run --release; sed -i '' -e 's/fat/thin/' Cargo.toml; git diff; cargo run; cargo run --release)
+-zsh:43> cargo --version
cargo 1.37.0 (9edd08916 2019-08-02)
+-zsh:43> rustc --version
rustc 1.37.0 (eae3437df 2019-08-13)
+-zsh:43> git clean -xdf
Removing target/
+-zsh:43> cargo run
   Compiling lto-drop v0.1.0 (/Users/cez/2019/09/21/fat-lto-drop-repro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/lto-drop`
About to panic
thread 'main' panicked at '???', src/libcore/option.rs:1034:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
Dropping a Droppable
+-zsh:43> cargo run --release
   Compiling lto-drop v0.1.0 (/Users/cez/2019/09/21/fat-lto-drop-repro)
    Finished release [optimized] target(s) in 2.09s
     Running `target/release/lto-drop`
About to panic
thread 'main' panicked at '???', src/libcore/option.rs:1034:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
Dropping a Droppable
+-zsh:43> sed -i '' -e s/fat/thin/ Cargo.toml
+-zsh:43> git diff
diff --git a/Cargo.toml b/Cargo.toml
index b2151aa..c483339 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -10,4 +10,4 @@ license = "MIT"
 [dependencies]

 [profile.release]
-lto = "fat"
+lto = "thin"
+-zsh:43> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/lto-drop`
About to panic
thread 'main' panicked at '???', src/libcore/option.rs:1034:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
Dropping a Droppable
+-zsh:43> cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/lto-drop`
About to panic
thread 'main' panicked at '???', src/libcore/option.rs:1034:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
Dropping a Droppable
: cez@ceri-no-mini;

```
