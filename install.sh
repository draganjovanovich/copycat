#!/bin/zsh

pip install -r requirements.txt

launchctl remove com.copycat.meraxes

touch ~/.copycat

rm  /usr/local/bin/copycat
rm ~/Library/LaunchAgents/com.copycat.meraxes.plist
cp -r ./target/release/copycat /usr/local/bin/
cp -r ./popup.py /usr/local/bin/

echo "<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
 <key>Label</key>
 <string>com.copycat.meraxes</string>
 <key>ProgramArguments</key>
 <array>
  <string>/usr/local/bin/copycat</string>
 </array>
 <key>WorkingDirectory</key>
 <string>/usr/local/bin</string>
 <key>RunAtLoad</key>
 <true/>
 <key>KeepAlive</key>
 <true/>
</dict>
</plist>
" > ~/Library/LaunchAgents/com.copycat.meraxes.plist
chmod 640 ~/Library/LaunchAgents/com.copycat.meraxes.plist

if ! launchctl list | grep -q com.copycat.meraxes; then
  launchctl load ~/Library/LaunchAgents/com.copycat.meraxes.plist
fi

python_exec=$(which python)

if ! grep -q "copycat_popup" ~/.zshrc; then
    # write text to ~/.zshrc file
    echo "function copycat_popup() {
        BUFFER=\"${python_exec} /usr/local/bin/popup.py\"
        echo -e \"\033[2K\"
        zle accept-line
    }

    zle -N copycat_popup

    bindkey '^T' copycat_popup
    " >> ~/.zshrc
    source ~/.zshrc
fi

