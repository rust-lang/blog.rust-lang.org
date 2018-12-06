#!/bin/bash

set -o errexit -o nounset

if [ "$TRAVIS_BRANCH" != "master" ]
then
  echo "This commit was made against the $TRAVIS_BRANCH and not the master! No deploy!"
  exit 0
fi

rev=$(git rev-parse --short HEAD)

cd site

git init
git config user.name "Steve Klabnik"
git config user.email "steve@steveklabnik.com"

git remote add upstream "https://$GH_TOKEN@github.com/rust-lang/blog.rust-lang.org.git"
git fetch upstream
git reset upstream/gh-pages

touch .

cp ../CNAME .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
