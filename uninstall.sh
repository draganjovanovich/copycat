#!/bin/zsh

launchctl bootout gui/$(id -u)/com.copycat.gagleto 2>/dev/null || launchctl remove com.copycat.gagleto 2>/dev/null || true

pkill -9 -f "/usr/local/bin/copycat" 2>/dev/null || true
pkill -9 -f "/usr/local/bin/popup" 2>/dev/null || true

rm -f ~/.copycat
sudo rm -f /usr/local/bin/copycat
sudo rm -f /usr/local/bin/popup
sudo rm -f /usr/local/bin/popup.py
rm -f ~/Library/LaunchAgents/com.copycat.gagleto.plist

echo ""
echo "NOTE: You may want to remove 'copycat' from:"
echo "  System Settings > Privacy & Security > Accessibility"
echo ""
