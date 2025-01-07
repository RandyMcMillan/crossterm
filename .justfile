alias d := doc
alias u := update
#alias r := run
alias t := test
alias b := build
alias c := check
alias ia := install-all
alias rr := run-release
alias w := watch

default:
    @just --choose || true

book:
    mdbook serve --open || cargo  install mdbook --vers "0.4.43"

build-all: generate-all
    #cargo b --bin tui-logger --features crossterm --manifest-path tui-logger/Cargo.toml
    cargo b --features crossterm

build-all-release: generate-all
    #cargo b -r --bin tui-logger --features crossterm --manifest-path tui-logger/Cargo.toml
    cargo b -r --features crossterm

install-all: build-all-release
    cargo install --force --path cli
    cargo install --force --path component
    cargo install --force --path gnostr-ui
    cargo install --force --path term
    cargo install --force --path simple
    cargo install --force --path simple-async
    cargo install --force --path tui-logger
    cargo install --force --path user-input
    cargo install --force --path lib

generate-all:
    #git stash #--include-untracked -a
    just generate-cli
    just gnostr-component
    just generate-nostui
    just generate-component
    just generate-wasm-pack
    just generate-simple
    just generate-simple-async
    just generate-term
    just generate-tui-logger
    just generate-lib
    just generate-user-input
    just generate-dumbpipe
    #git stash pop

gnostr-component:
    mkdir -p gnostr-component
    rm -rv gnostr-component
    cargo generate --path ./gnostr-component-template \
        --name gnostr-component \
        --define project-description="An example generated using the gnostr component template" \
        --define use-gitserver=false
    touch gnostr-component/.gitkeep

generate-nostui:
    mkdir -p gnostr-ui
    rm -rv gnostr-ui
    cargo generate --path ./nostui-template \
        --name gnostr-ui \
        --define project-description="An example generated using the gnostr component template" \
        --define use-gitserver=false
    touch gnostr-ui/.gitkeep

generate-component:
    mkdir -p component
    rm -rv component
    cargo generate --path ./component-template \
        --name component \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false
    touch component/.gitkeep

generate-wasm-pack:
    mkdir -p wasm-pack
    rm -rv wasm-pack
    cargo generate --git https://github.com/rustwasm/wasm-pack-template.git \
        --name wasm-pack \
        --define project-description="An example generated using the wasm-pack template" \
        --define use-gitserver=false
    touch wasm-pack/.gitkeep

generate-tui-logger:
    mkdir -p tui-logger
    rm -rv tui-logger
    cargo generate --path ./logger-template --name tui-logger
    touch tui-logger/.gitkeep

generate-term:
    mkdir -p term
    rm -rv term
    cargo generate --path ./term-template --name term
    touch term/.gitkeep

generate-simple:
    mkdir -p simple
    rm -rv simple
    cargo generate --path ./simple-template --name simple
    touch simple/.gitkeep

generate-simple-async:
    mkdir -p simple-async
    rm -rv simple-async
    cargo generate --path ./simple-async-template --name simple-async
    touch simple-async/.gitkeep

generate-cli:
    mkdir -p cli
    rm -rv cli
    cargo generate --path ./cli-template --name cli \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false
    touch cli/.gitkeep

generate-lib:
    mkdir -p lib
    rm -rv lib
    cargo generate --path ./lib-template --name lib \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false
    touch lib/.gitkeep

generate-user-input:
    mkdir -p user-input
    rm -rv user-input
    cargo generate --path ./user-input-template --name user-input
    touch user-input/.gitkeep

generate-dumbpipe:
    mkdir -p dumbpipe
    rm -rv dumbpipe
    cargo generate --path ./dumbpipe-template --name dumbpipe
    touch dumbpipe/.gitkeep

clippy:
    cargo  clippy --all-targets --all-features

deny:
    cargo  deny check

test:
    cargo  test -- --nocapture

crossterm_examples:
    cargo  run --bin  crossterm_examples --features="event-stream events" || true

event-match-modifiers:
    cargo  run --bin event-match-modifiers || true

event-poll-read:
    cargo  run --bin event-poll-read || true

event-read:
    cargo  run --bin event-read || true

event-read-line:
    cargo  run --bin event-read-line || true

event-read-char-line:
    cargo  run --bin event-read-chat-line || true

event-stream-async-std:
    cargo  run --bin event-stream-async-std --features="event-stream events" || true

event-stream-tokio:
    cargo  run --bin event-stream-tokio --features="event-stream events" || true

is_tty:
    cargo  run --bin is_tty || true

key-display:
    cargo  run --bin key-display || true

stderr:
    ls "$(target/debug/stderr)" || cargo  run --bin  stderr || true


build:
    cargo  build --bins --features="event-stream events" || true

build-examples:
    cargo b --example demo --manifest-path tui-logger/Cargo.toml --features crossterm || true

check:
    cargo  check || true

run-release:
    cargo  run --release --bin rust-cookbook || true

doc:
    cargo  doc --open --offline

update:
    cargo update

# Future incompatibility report, run regularly
future:
    cargo check --future-incompat-report

watch:
    cargo watch -x check -x test -x build

# vim: set list:
# vim: set expandtab:
# vim: set setfiletype make
