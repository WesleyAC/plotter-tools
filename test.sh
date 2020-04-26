#!/bin/sh
set -e

for rust_project in chunker hpgl hpgl2gcode optimize typewriter viz
do
	echo "testing $rust_project..."
	(cd $rust_project && cargo build && cargo test)
done

echo "all tests passed <3"
