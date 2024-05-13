#!/bin/bash

# Specific tag to delete
TAG="$1"

# Delete the specific local tag
git tag -d $TAG

# Fetch the current list of tags
git fetch

# Delete the specific remote tag
git push --delete origin $TAG

# Prune local references to remote tags
git fetch --prune --prune-tags

# Delete the specific GitHub release using the GitHub CLI
gh release delete $TAG --yes
