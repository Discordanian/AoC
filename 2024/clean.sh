#!/bin/sh

for i in day-??
do
	cd $i
  if command -v lolcat >/dev/null 2>&1
  then
    figlet $i |lolcat
    cargo clean |lolcat
  else
    figlet $i 
    cargo clean 
  fi
	cd ..
done
