#!/usr/bin/env bash
set -uo pipefail
flag() {
	for f in "$@"
		do [[ -e ".flags/$f" ]] || return 1
	done
}
rm -rf logs
mkdir -p logs
cargo=(
	fmt
	clippy
	clean
	check
	build
)
for i in "${cargo[@]}"; do
	log="logs/$i.log"
	cargo $i &> "$log"
	if [[ $? != 0 ]]; then
		code "$log"
		break
	fi
done
bin=target/debug/markup-rust
chmod +x "$bin"
if [[ $? == 0 ]]
	then "./$bin" > index.html
fi
find . -empty -delete
