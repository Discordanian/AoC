#!/bin/sh

for i in day-??
do
	cd $i
  if command -v lolcat >/dev/null 2>&1
  then
    figlet $i |lolcat
    cargo test |lolcat
    if [ $? -ne 0 ]
    then
      exit
    fi
  else
    figlet $i
    cargo test
    if [ $? -ne 0 ]
    then
      exit
    fi
  fi
	cd ..
done
