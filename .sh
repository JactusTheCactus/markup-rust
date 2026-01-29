#!/usr/bin/env bash
set -uo pipefail
flag() {
	for f in "$@"
		do [[ -e ".flags/$f" ]] || return 1
	done
}
clean() {
	rm -rf logs index.html
	mkdir -p logs
}
build() {
	cargo_k=(
		fmt
		clippy
		check
		build
	)
	declare -A cargo_v=()
	cargo_v[fmt]='+nightly fmt'
	cargo_v[clippy]=clippy
	cargo_v[check]=check
	cargo_v[build]=build
	for i in "${cargo_k[@]}"; do
		log="logs/$i.log"
		# shellcheck disable=SC2086
		if ! cargo ${cargo_v[$i]} &> "$log"; then
			code "$log"
			break
		fi
	done
}
check() {
	shellcheck .sh
}
if [[ $(check) == '' ]]
	# then if cargo clean
		then if clean
			then if build
				then ./target/debug/markup-rust > index.html
			fi
		fi
	# fi
	else check
fi
find . -empty -delete
