#!/bin/sh

for i in day-??
do
	cd $i
	figlet $i
	cargo clean
	cd ..
done
