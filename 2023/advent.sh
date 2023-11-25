#!/bin/sh

if [ -z "$1" ]
then
	echo "Pass in a two digit number 00 -> 25"
	exit 
fi

export SRCLIB=day-00
export LIBNAME=day-$1

cargo new --lib $LIBNAME
cp $SRCLIB/solve.sh $LIBNAME/solve.sh
mkdir $LIBNAME/src/bin/
sed "s/00/$1/g" $SRCLIB/src/bin/part-1.rs > $LIBNAME/src/bin/part-1.rs
sed "s/00/$1/g" $SRCLIB/src/bin/part-2.rs $LIBNAME/src/bin/part-2.rs
cp -v $SRCLIB/src/lib.rs $LIBNAME/src/lib.rs

cd $LIBNAME
cargo test
