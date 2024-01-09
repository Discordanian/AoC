#!/bin/sh

for i in 20??/
do
	cd $i
	if [ -f ./clean.sh ]
	then
		./clean.sh
	fi
	cd ..
done

