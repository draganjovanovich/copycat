# COPYCAT  
<img src="/logo.jpeg"  width="30%" height="30%"><br/>
<sup>Produced with stable-diffusion<sup>

## Clipboard (copy/paste) history buffer for terminal emulators, MAC OS gui and VIM* environment usage.

### Rrequirements  
- MACOS  
- ZSH  
- Python3  

**NOTE:** `It's possible to use bash instead of zsh, but it will require manual tweaking of .bashrc file.`  

### Instalation  

Run this command from terminal: `./install.sh`  
### Usage modes
There are 3 possible modes of usage:
- **Terminal emulator**:  
You can use this mode when you are inside of the terminal and you want to access CB history directly inside of terminal window, without leaving terminal.
- **GUI**:  
You can use this mode from wherever you are in the macos gui currently, for example you are in Chrome, just Double press Left Alt key ...  
- **VIM**:  
You can use this mode from inside of VIM, with some help of additional vim plugin mentioned in the description below  


### TERMINAL emulator usage
Open select menu from terminal with `<CTRL> + t`  
Toggle preview of large CB items with key `p`  
Navigate window with VIM like motions: `j, k, gg, G, C-u, C-d`  
Confirm and copy selection to cliboard with `Enter key`  
**NOTE:** `You can customize keybinding for triggering copycat in terminal by editing ~/.zshrc file`  

### GUI usage
Open select menu with `Double press Left Alt key` it's not remapable for now  
Toggle preview of large CB items with key `Mouse right click` or `key p`   
Navigate window with VIM like motions: `j, k, gg, G, C-u, C-d`  
Confirm and copy selection to cliboard with `Enter key` or `Mouse double left click`   

### *VIM usage
You can use it in VIM with the help of this plugin:  
[https://github.com/voldikss/vim-floaterm](vim-floatterm)  

And add something like this into your vim config file:  
`noremap <C-t> :FloatermNew --title=copycat --width=1.0 --disposable --wintype=float --position=bottom python /usr/local/bin/popup.py<CR>`
