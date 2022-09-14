#!/bin/zsh


rm ~/.copycat
rm  /usr/local/bin/copycat
rm  /usr/local/bin/popup.py
rm ~/Library/LaunchAgents/com.copycat.meraxes.plist

launchctl remove com.copycat.meraxes
