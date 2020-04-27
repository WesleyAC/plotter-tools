#!/bin/sh
set -e

declare -A projects
projects[chunker]="stable nightly"
projects[hpgl]="stable nightly"
projects[hpgl2gcode]="stable nightly"
projects[optimize]="nightly"
projects[typewriter]="stable nightly"
projects[viz]="stable nightly"

for project in ${!projects[@]}
do
	for rust_version in ${projects[$project]}
	do
		echo "testing $project on $rust_version..."
		(cd $project && cargo +$rust_version build && cargo +$rust_version test)
	done
done

echo "all tests passed <3"
