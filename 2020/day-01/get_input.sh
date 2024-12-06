YEAR=2020
((DAY=$1+0))

echo curl -b session=`cat session_id` https://adventofcode.com/$YEAR/day/$DAY/input 
curl -b session=`cat session_id` https://adventofcode.com/$YEAR/day/$DAY/input > input.txt
