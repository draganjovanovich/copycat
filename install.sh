#!/bin/zsh

# check if we have rust installed
if ! command -v rustc &> /dev/null
then
    echo "rustc is required, please install it: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit
fi

# check if user have python3
if ! command -v python3 &> /dev/null
then
    echo "python3 is required, please install it: brew install python"
    exit
fi

# check if user have pip3 installed
if ! command -v pip3 &> /dev/null
then
    echo "pip3 is required, please install it: brew install python"
    exit
fi

cargo build --release --bins
cd popup/popup
cargo build --release --bins
cd ../../
pip install -r requirements.txt

launchctl remove com.copycat.gagleto

sudo touch ~/.copycat
sudo chown $(whoami) ~/.copycat

sudo cp -r ./target/release/copycat /usr/local/bin/
sudo chown $(whoami) /usr/local/bin/copycat
sudo cp -r ./popup/popup/target/release/popup /usr/local/bin/
sudo chown $(whoami) /usr/local/bin/popup
sudo cp -r ./popup/popup.py /usr/local/bin/
sudo chown $(whoami) /usr/local/bin/popup.py

sudo bash -c 'cat <<EOIPFW >> ~/Library/LaunchAgents/com.copycat.gagleto.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
 <key>Label</key>
 <string>com.copycat.gagleto</string>
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
EOIPFW' 
sudo chmod 640 ~/Library/LaunchAgents/com.copycat.gagleto.plist
sudo chown $(whoami) ~/Library/LaunchAgents/com.copycat.gagleto.plist

if ! launchctl list | grep -q com.copycat.gagleto; then
  launchctl load ~/Library/LaunchAgents/com.copycat.gagleto.plist
fi

python_exec=$(which python3)

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
fi

source ~/.zshrc

# please restart your terminal if needed
echo "Please restart your terminal if ctrl+t does not work, or run: source ~/.zshrc"

