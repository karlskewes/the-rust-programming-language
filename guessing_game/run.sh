#!/usr/bin/env bash

set -o errexit -o nounset -o pipefail

# Simple task runner that's not Make so syntax highlighting and shellcheck, shfmt, etc work.
# https://gist.github.com/tvalladon/e316bee4b58ca082d2190be023565949

debug() { ## Debug REPL
	echo "Stopped in REPL. Press ^D to resume, or ^C to abort."
	local line
	while read -r -p "> " line; do
		eval "$line"
	done
	echo
}

versions() { ## Versions of tools
	cargo --version
	rustc --version
}

update() { ## Todo - update
	echo "not implemented"
}

audit() { ## Audit security vulnerabilities with cargo-audit
	cargo-audit
}

coverage() { ## Code coverage with tarpaulin
	cargo install cargo-tarpaulin

	cargo tarpaulin --ignore-tests
}

cw() { ## Cargo watch all the things
	cargo-watch -x check -x test -x run
}

doc() { ## Open cargo docs in browser
	cargo doc --open
}

fmt() { ## Format with rustfmt
	cargo fmt
}

lint() { ## Lint with clippy
	cargo clippy -- -D warnings
}

one-arg-that-is-very-long() { ## Example that requires 1 arg <arg>
	if [ $# -ne 1 ]; then
		echo 1>&2 "Usage: $0 ${FUNCNAME[0]} <arg>"
		exit 3
	fi
	echo "Doing onearg thing with $1"
}

help() { ## Display usage for this application
	echo "$0 <task> <args>"
	grep -E '^[a-zA-Z_-]+\(\) { ## .*$' "$0" |
		sed -E 's/\(\)//' |
		sort |
		awk 'BEGIN {FS = "{.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $1, $2}'
}

TIMEFORMAT="Task completed in %3lR"
time "${@:-help}"
