
set export := true

IGNORE_PATTERN := "/.cargo/registry|main.rs|rustc/"

# see https://doc.rust-lang.org/rustc/instrument-coverage.html
generate-coverage:
  RUSTFLAGS="-C instrument-coverage" cargo test --tests


  llvm-profdata merge -sparse default_*.profraw -o default.profdata
  llvm-cov show --use-color --ignore-filename-regex='/.cargo/registry' -instr-profile=default.profdata target/debug/expenses -Xdemangler=rustfilt --ignore-filename-regex=${IGNORE_PATTERN}
  llvm-cov report -instr-profile=default.profdata target/debug/expenses --ignore-filename-regex=${IGNORE_PATTERN}
  rm default_*.profraw default.profdata

