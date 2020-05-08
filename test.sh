#!/usr/bin/env bash
set -e

declare -A projects
projects[chunker]="stable nightly"
projects[hpgl]="stable nightly"
projects[hpgl2gcode]="stable nightly"
projects[optimize]="nightly"
projects[typewriter]="stable nightly"
projects[viz]="stable nightly"
projects[osm2hpgl]="nightly"
projects[canonicalize]="stable nightly"

for project in "${!projects[@]}"
do
	for rust_version in ${projects[$project]}
	do
		echo "testing '$project' on $rust_version..."
		(cd "$project" && cargo +"$rust_version" build && cargo +"$rust_version" test)
	done
done

for model in $(find -name \*.scad)
do
	if [ "$model" != "./adapter/Write.scad" ]; then
		echo "testing '$model'..."
		OUTFILE=$(mktemp).stl
		openscad $model -o $OUTFILE 2> /dev/null
		if ! [ -s $OUTFILE ]; then
			echo "$model produced empty output!"
			exit 1
		fi
		rm $OUTFILE
	fi
done

echo "all tests passed <3"
