set -euxo pipefail

main() {
    cargo run

    linkchecker public
}

main
