default:
    @just --list

dev:
    cargo run

prod:
    cargo run --release
