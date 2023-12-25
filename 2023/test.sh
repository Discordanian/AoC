#!/bin/sh

for i in day-??
do
	cd $i
	figlet $i
	cargo test
	if [ $? -ne 0 ]
	then
		exit
	fi
	cd ..
done
