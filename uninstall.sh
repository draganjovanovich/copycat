#!/bin/zsh


sudo rm ~/.copycat
sudo rm  /usr/local/bin/copycat
sudo rm  /usr/local/bin/popup
sudo rm  /usr/local/bin/popup.py
sudo rm ~/Library/LaunchAgents/com.copycat.gagleto.plist

launchctl remove com.copycat.gagleto
