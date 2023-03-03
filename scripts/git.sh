#!/bin/bash

# ? A simple bash script for committing files to github without a GUI
# Git auth is setup to make this easy.

GIT_MESSAGE=""  # the commit message for git
DEFAULT_MSG="Updating Code!"

echo "What is your commit message? (Default: ${DEFAULT_MSG})"
read GIT_MESSAGE # save the data given.

if [[ -z $GIT_MESSAGE ]] # checks if the commit message was given. if not is, default to DEFAULT_MSG
    then
        # adding all changed files
        git add .
            # pushing code to github
            git commit -m "${DEFAULT_MSG}"
                # if no string was given for the commit. We add a default.
                echo "No message was given, using default. Default: ${DEFAULT_MSG}"
                    # push all committed code!
                    git push
    else
        # adding all changed files
        git add .
            # pushing code to github
            git commit -m "${GIT_MESSAGE}"
                echo "The changes have been pushed to github!"
                echo "Commit message sent: ${GIT_MESSAGE}"
                    # push all committed code!
                    git push
fi

# build with: chmod +x git.sh
# run the script with: ./git.sh