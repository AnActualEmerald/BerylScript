#!/usr/bin/fish

#set EMTIMESTART $(date +%ms)

time gem_cli run test.em

#EMTIMEEND=$(date +%ms)

#JSTIMESTART=$(date +%ms)

time node test.js

#JSTIMEEND=$(date +%ms)

#echo "EmeraldScript took $((EMTIMESTART - EMTIMEEND))"
#echo "JavaScript took $((JSTIMESTART - JSTIMEEND))"

