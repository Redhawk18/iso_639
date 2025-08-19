#!/bin/sh

curl -o iso-639-1.tsv "https://id.loc.gov/vocabulary/iso639-1.tsv"
curl -o iso-639-2.tsv "https://id.loc.gov/vocabulary/iso639-2.tsv"
# TODO find a good source for 3.
# curl -o iso-639-3.tsv "https://id.loc.gov/vocabulary/iso639-3.tsv"
curl -o iso-639-5.tsv "https://id.loc.gov/vocabulary/iso639-5.tsv"
