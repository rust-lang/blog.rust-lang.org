#!/bin/bash
# requires apt packages: aspell, aspell-en
# Adapted from https://github.com/eleven-labs/eleven-labs.github.io/blob/master/bin/check-spelling.sh

# Usages:
#   Ensure you have aspell installed.
#   ./ci/spellcheck.sh [file,]

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;36m'
LIGHT_GREY='\033[0;37m'
GREY='\033[0;90m'
NC='\033[0m' # No Color

if [ -n "$1" ]; then
  MARKDOWN_FILES_CHANGED=`echo "${@:1}" |  tr " " "\n"`

  echo -e "$BLUE>> Following markdown files are being checked:$NC"
  echo -e "$MARKDOWN_FILES_CHANGED"
elif [ -n "$TRAVIS_COMMIT_RANGE" ]; then
  echo -e "$BLUE>> Checking all .md files $NC"
  MARKDOWN_FILES_CHANGED=`git ls-tree --full-tree --name-only  -r HEAD | grep .md`

  echo -e "$BLUE>> Following markdown files were changed in this pull request (commit range: $TRAVIS_COMMIT_RANGE):$NC"
  echo -e "$MARKDOWN_FILES_CHANGED"
else
  echo -e "$BLUE>> Checking all files modified between $MARKDOWN_FILES_CHANGED $NC"
  MARKDOWN_FILES_CHANGED=`(git diff --name-only $MARKDOWN_FILES_CHANGED || true) | grep .md`

  echo -e "$BLUE>> Following markdown files were changed in this repository:$NC"
  echo -e "$MARKDOWN_FILES_CHANGED"
fi


if [ -z "$MARKDOWN_FILES_CHANGED" ]
then
    echo -e "$GREEN>> No markdown file to check $NC"

    exit 0;
fi

echo -e "$BLUE>> Assuming language is 'en'. $NC"

STATUS=0

while read -r file; do
  echo -e "$BLUE>> Checking file: $file $NC"

  if [ ! -f $file ]; then
    echo -e "$RED>> File $file does not exist $NC"
    STATUS=1
    continue
  fi

  # cat all markdown files that changed
  TEXT_CONTENT_WITHOUT_METADATA=`sed -E ':a;N;$!ba;s/\n/ /g' $file`

  echo $TEXT_CONTENT_WITHOUT_METADATA >> before

  # remove metadata tags
  TEXT_CONTENT_WITHOUT_METADATA=`echo "$TEXT_CONTENT_WITHOUT_METADATA" | grep -v -E '^(layout:|permalink:|date:|date_gmt:|authors:|categories:|tags:|cover:)(.*)'`

  # remove { } attributes
  TEXT_CONTENT_WITHOUT_METADATA=`echo "$TEXT_CONTENT_WITHOUT_METADATA" | sed -E 's/\{:([^\}]+)\}//g'`

  # remove html
  TEXT_CONTENT_WITHOUT_METADATA=`echo "$TEXT_CONTENT_WITHOUT_METADATA" | sed -E 's/<([^<]+)>//g'`

  # remove code blocks
  TEXT_CONTENT_WITHOUT_METADATA=`echo "$TEXT_CONTENT_WITHOUT_METADATA" | sed  -n '/^\`\`\`/,/^\`\`\`/ !p'`

  # remove links
  TEXT_CONTENT_WITHOUT_METADATA=`echo "$TEXT_CONTENT_WITHOUT_METADATA" | sed -E 's/http(s)?:\/\/([^ ]+)//g'`

  # remove what is probably a domain
  TEXT_CONTENT_WITHOUT_METADATA=`echo "$TEXT_CONTENT_WITHOUT_METADATA" | perl -pe 's/\W\w*\.\w{2,}//g'`

  echo $TEXT_CONTENT_WITHOUT_METADATA >> after

  MISSPELLED=`echo "$TEXT_CONTENT_WITHOUT_METADATA" | aspell --lang=en --encoding=utf-8 --personal=./.aspell.en.pws list | sort -u`

  OUTPUT=""

  if [ -z "$MISSPELLED" ]; then
    NB_MISSPELLED=0
    COMMENT="No spelling errors were found"
    echo -e "$GREEN>> $COMMENT $NC"
  else
    echo -e "$RED>> Words that might be misspelled, please check:$NC"

    while read -r word; do
      line=`grep -n "$word" $file | awk -F ":" '{print $1}'`

      while read -r location; do
        context=`awk -v word="$word" -v location="$location" 'word && NR == location' "$file"`
        context=`echo "$context" | ack -o ".{0,15}$word.{0,15}"`
        MSG="$NC$file:$location$NC$RED\t$word$NC\t$GREY$context$NC\n"
        OUTPUT="$OUTPUT$MSG\n"
      done <<< "$line"
    done <<< "$MISSPELLED"

    echo -ne $OUTPUT | column -ts $'\t'

    STATUS=1
  fi
done <<< "$MARKDOWN_FILES_CHANGED"

exit $STATUS
