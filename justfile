fmt:
	cargo +nightly fmt

alias f := fmt

check:
    cargo clippy --all-targets -- -D clippy::all -W clippy::pedantic

alias c := check

fix:
    cargo clippy --fix 

watch:
	cargo watch -cq --no-vcs-ignores -x "run --bin debug"

alias w := watch

install:
    @cargo install --path .

alias i := install