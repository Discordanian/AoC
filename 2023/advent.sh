#!/bin/sh

if [ -z "$1" ]
then
	echo "Pass in a two digit number 00 -> 25"
	exit 
fi

cargo new --lib day-$1
