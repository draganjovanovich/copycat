#!/bin/zsh

if ! command -v rustc &> /dev/null
then
    echo "rustc is required, please install it: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit
fi

if ! command -v python3 &> /dev/null
then
    echo "python3 is required, please install it: brew install python"
    exit
fi

if ! command -v pip3 &> /dev/null
then
    echo "pip3 is required, please install it: brew install python"
    exit
fi

cargo build --release --bins
cd popup/popup
cargo build --release --bins
cd ../../

pip3 install --user -r requirements.txt 2>/dev/null || pip3 install --break-system-packages -r requirements.txt 2>/dev/null || echo "Python deps may already be installed"

launchctl bootout gui/$(id -u)/com.copycat.gagleto 2>/dev/null || launchctl remove com.copycat.gagleto 2>/dev/null || true

pkill -9 -f "/usr/local/bin/copycat" 2>/dev/null || true
pkill -9 -f "/usr/local/bin/popup" 2>/dev/null || true

sudo touch ~/.copycat
sudo chown $(whoami) ~/.copycat

sudo cp -r ./target/release/copycat /usr/local/bin/
sudo chown $(whoami) /usr/local/bin/copycat
sudo cp -r ./popup/popup/target/release/popup /usr/local/bin/
sudo chown $(whoami) /usr/local/bin/popup
sudo cp -r ./popup/popup.py /usr/local/bin/
sudo chown $(whoami) /usr/local/bin/popup.py

cat > ~/Library/LaunchAgents/com.copycat.gagleto.plist << 'EOIPFW'
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
 <false/>
</dict>
</plist>
EOIPFW
chmod 640 ~/Library/LaunchAgents/com.copycat.gagleto.plist

launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/com.copycat.gagleto.plist 2>/dev/null || launchctl load ~/Library/LaunchAgents/com.copycat.gagleto.plist 2>/dev/null || true

echo ""
echo "==================================================================="
echo "IMPORTANT: Grant Accessibility permissions to enable Alt+Alt hotkey"
echo "==================================================================="
echo ""
echo "Opening System Settings > Privacy & Security > Accessibility..."
echo "Please ADD or RE-ENABLE '/usr/local/bin/copycat'"
echo ""

open "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility"

python_exec=$(which python3)

if ! grep -q "copycat_popup" ~/.zshrc; then
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

echo "Please restart your terminal if ctrl+t does not work, or run: source ~/.zshrc"
