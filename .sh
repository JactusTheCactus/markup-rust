#!/usr/bin/env bash
set -uo pipefail
flag() {
	for f in "$@"
		do [[ -e ".flags/$f" ]] || return 1
	done
}
rm -rf logs index.html
mkdir -p logs
cargo=(
	fmt
	clippy
	clean
	check
	build
)
for i in "${cargo[@]}"; do
	cmd=($i)
	case $i in
		fmt)cmd=(+nightly ${cmd[@]});;
		build)cmd+=(--release);; # Comment out for dev work
	esac
	log="logs/$i.log"
	if ! cargo ${cmd[@]} &> "$log"; then
		code "$log"
		break
	fi
done
project=markup-rust
if [[ -f "target/debug/$project" ]]
	then dir=debug
	else dir=release
fi
bin=target/$dir/$project
"./$bin" > index.html
find . -empty -delete
