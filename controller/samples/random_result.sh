#!/bin/bash

number=0
number=$(( $RANDOM % 2 ))
if [ "$number" == "1" ]; then
	exit 0
else
	exit 1
fi