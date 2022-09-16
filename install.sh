#!/bin/zsh

# check if we have rust installed
if ! command -v rustc &> /dev/null
then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

cargo build --release --bins
pip install -r requirements.txt

launchctl remove com.copycat.meraxes

sudo touch ~/.copycat
# chown .copycat so current user is owner
sudo chown $(whoami) ~/.copycat

sudo cp -r ./target/release/copycat /usr/local/bin/
sudo cp -r ./popup.py /usr/local/bin/

sudo echo "<?xml version="1.0" encoding="UTF-8"?>
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
        echo -ne \"\033[1A\"
        echo -ne \"\033[0K\"
    }

    zle -N copycat_popup

    bindkey '^T' copycat_popup
    " >> ~/.zshrc
    source ~/.zshrc
fi

