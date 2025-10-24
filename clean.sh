#!/bin/sh

for i in 20??/
do
	cd $i
	if [ -f ./clean.sh ]
	then
    if command -v lolcat >/dev/null 2>&1
      then
      ./clean.sh |lolcat
    else
      ./clean.sh
    fi
	fi
	cd ..
done

