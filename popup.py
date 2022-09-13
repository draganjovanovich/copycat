#!/usr/bin/python3

import os
import enquiries
import pyperclip

try:
    home = os.path.expanduser("~")
    with open(home + "/.copycat", "r") as f:
        options = f.readlines()

    options = options[-10:][::-1]
    options = [x.strip() for x in options if x.strip()]

    choice = enquiries.choose('Last 10 items: ', options)
    pyperclip.copy(choice)

except KeyboardInterrupt:
    exit()


