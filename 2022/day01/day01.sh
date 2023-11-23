#!/bin/sh
export ELF=1
export MAX=0
export TOTAL=0

cat $1 |while read LINE
do
    if [ "$LINE" == "" ]
    then
        echo $TOTAL for ELF $ELF
        if [ $MAX -lt $TOTAL ]
        then
            export MAX=$TOTAL
            echo $TOTAL for ELF $ELF is Current MAX
        fi
        ELF=$((ELF+1))
        TOTAL=0
    fi
    TOTAL=$((TOTAL+LINE))
    # echo "Running Total $TOTAL for $LINE"
done
