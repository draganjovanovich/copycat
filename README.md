# COPYCAT  

## Clipboard buffer for terminal and gui environments

### Rrequirements  
- MACOS  
- ZSH  
**NOTE:** `It's possible to use bash instead of zsh, but with manual tweaking of .bashrc file.`  

### Instalation  

Run this command from terminal: `./install.sh`  
### Usage modes
There are 3 possible modes of usage:
- **Terminal emulator**:  
You can use this mode when you are inside of the terminal and you want to access CB history directly inside of terminal window, without leaving terminal.
- **GUI**:  
You can use this mode from wherever you are in the of the macos, for example you are in Chrome...  
- **VIM**:  
You can use this mode from inside of VIM, with some help of additional vim plugin mentioned in the description below  


### Terminal emulator usage
Open select menu from terminal with `<CTRL> + t`  
Toggle preview of large CB items with key `p`  
Navigate preview window with VIM like motions: `j,k, gg, G`
Confirm and copy selection to cliboard with `Enter key`  
**NOTE:** `You can customize keybinding for triggering copycat in terminal by editing ~/.zshrc file`  

### GUI usage
Open select menu with `F1 key` it's not remapable for now  
Toggle preview of large CB items with key `Mouse right click`  
Confirm and copy selection to cliboard with `Enter key` or `Mouse double left click`   

### VIM usage
You can use it in VIM with the help of this plugin:  
[https://github.com/voldikss/vim-floaterm](vim-floatterm)

And add something like this into your vim config file:  
`noremap <C-t> :FloatermNew --title=copycat --width=1.0 --disposable --wintype=float --position=bottom python /usr/local/bin/popup.py<CR>`
