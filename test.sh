#!/bin/bash

cd /media/pipi/0123-4567/JPG;
zoo= ls -l | wc -l;
cd /media/pipi/taz/JPG;
foo= ls -l | wc -l;
cd /media/pipi/USB01/JPG;
boo= ls -l | wc -l;

echo $zoo + $foo + $boo;