YEAR=2016

((DAY=$1+0))

curl -b session=`cat session_id` https://adventofcode.com/$YEAR/day/$DAY/input > input.txt
