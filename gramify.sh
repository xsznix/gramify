#!/bin/sh
# Use gramify with reasonable default significance thresholds and patterns.
CHAR="[A-Za-z0-9\\\\/\-?'\",<.>_;: ]"
./target/release/gramify \
  --letter-threshold=100 \
  --bigram-threshold=100 \
  --skipgram-threshold=100 \
  --trigram-threshold=10 \
  --letter-pattern="$CHAR" \
  --bigram-pattern="$CHAR{2}" \
  --skipgram-pattern="$CHAR{2}" \
  --trigram-pattern="$CHAR{3}" \
  "$@"