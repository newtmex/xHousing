#!/usr/local/bin/bash
shopt -s globstar

cargo clean
# rm -rf target/debug/coverage

## https://github.com/mozilla/grcov
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests"
export RUSTDOCFLAGS="-Cpanic=abort"

cargo build

if [ -z $1 ]; then
    echo -e
    echo "Running all tests"
    echo -e
    cargo test
else
    if [ $1 == "unit" ]; then
        echo -e
        echo "Running unit tests"
        echo -e
        cargo test --lib
    else
        echo -e
        echo "Running integration tests"
        echo -e
        cargo test --test '*'
    fi
fi

echo "Generating coverage data"

report() {
    grcov ./target/debug/ -s . -t "$1" --llvm --branch -o "$2" \
        --ignore-not-existing \
        --ignore *meta* \
        --ignore *wasm* \
        --ignore *test*
}

report html ./target/debug/coverage/
report lcov ./target/debug/coverage/lcov.info
