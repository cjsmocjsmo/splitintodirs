#!/bin/bash

cd /media/pipi/0123-4567/JPG;
ls -l | wc -l;
cd /media/pipi/taz/JPG;
ls -l | wc -l;
cd /media/pipi/USB01/JPG;
ls -l | wc -l;
cd /home/pipi/splitintodirs;
git pull;
cargo run --release -- /media/pipi/0123-4567/JPG /media/pipi/0123-4567/Master_Split;
cargo run --release -- /media/pipi/taz/JPG /media/pipi/taz/Master_Split;
cargo run --release -- /media/pipi/USB01/JPG /media/pipi/USB01/Master_Split;
