#!/bin/sh

if [ -f src/bin/part-1.rs ]
then
	cargo run --bin part-1
fi

if [ -f src/bin/part-2.rs ]
then
	cargo run --bin part-2
fi
