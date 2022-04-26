depth=$1
fen=$2
moves=$3

./target/release/engine perft-div "$depth" "$fen" "$moves"
