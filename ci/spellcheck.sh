#!/bin/bash
# requires apt packages: aspell, aspell-en
# Adapted from https://github.com/eleven-labs/eleven-labs.github.io/blob/master/bin/check-spelling.sh

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;36m'
NC='\033[0m' # No Color

MARKDOWN_FILES_CHANGED=`(git diff --name-only $TRAVIS_COMMIT_RANGE || true) | grep .md`

if [ -z "$MARKDOWN_FILES_CHANGED" ]
then
    echo -e "$GREEN>> No markdown file to check $NC"

    exit 0;
fi

echo -e "$BLUE>> Following markdown files were changed in this pull request (commit range: $TRAVIS_COMMIT_RANGE):$NC"
echo "$MARKDOWN_FILES_CHANGED"

echo -e "$BLUE>> Assuming language is 'en'."

# cat all markdown files that changed
TEXT_CONTENT_WITHOUT_METADATA=`cat $(echo "$MARKDOWN_FILES_CHANGED" | sed -E ':a;N;$!ba;s/\n/ /g')`

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

echo -e "$BLUE>> Text content that will be checked (without metadata, html, and links):$NC"
echo "$TEXT_CONTENT_WITHOUT_METADATA"

MISSPELLED=`echo "$TEXT_CONTENT_WITHOUT_METADATA" | aspell --lang=en --encoding=utf-8 --personal=./.aspell.en.pws list | sort -u`

NB_MISSPELLED=`echo "$MISSPELLED" | wc -l`

if [ "$NB_MISSPELLED" -gt 0 ]
then
    echo -e "$RED>> Words that might be misspelled, please check:$NC"
    MISSPELLED=`echo "$MISSPELLED" | sed -E ':a;N;$!ba;s/\n/, /g'`
    echo "$MISSPELLED"
    COMMENT="$NB_MISSPELLED words might be misspelled, please check them: $MISSPELLED"
    
   exit 1
else
    COMMENT="No spelling errors were found"
    echo -e "$GREEN>> $COMMENT $NC"
    exit 0
fi
