#!/usr/bin/env bash
set -uo pipefail
flag() {
	for f in "$@"
		do [[ -e ".flags/$f" ]] || return 1
	done
}
build() {
	cargo_k=(clippy check build)
	declare -A cargo_v=(
		[clippy]=clippy
		[check]=check
		[build]=build
	)
	for i in "${cargo_k[@]}"; do
		log="logs/$i.log"
		# shellcheck disable=SC2086
		if ! cargo ${cargo_v[$i]} &> "$log"; then
			code "$log"
			break
		fi
		if [[ -f "$log" && $(wc -l < "$log") -le 2 ]]; then
			rm "$log"
		fi
	done
}
rm -rf logs index.html
mkdir -p logs
CHECK=$(mktemp)
shellcheck --color=always .sh &> "$CHECK"
if [[ $(<"$CHECK") == '' ]]; then
	if build &>> logs/build.log; then
		./target/debug/markup-rust \
			"$(<src/input.txt)" \
			> index.html \
			2> logs/main.log
	fi
	else cat "$CHECK"
fi
find . -empty -delete
