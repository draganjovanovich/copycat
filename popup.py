#!/usr/bin/python3

import sys
import os
import pyperclip

from curtsies import Input, CursorAwareWindow, fsarray

CHECKED = '\u25c9 '
UNCHECKED = '\u25cc '

def choose(choices, preselected=()):
    choice_list = ChoiceList(choices, preselected=preselected)
    with CursorAwareWindow(out_stream=sys.stderr, extra_bytes_callback=lambda x: x) as window:
        options = choice_list.run(window)

    return options

class Choice:
    def __init__(self, obj, display=None):
        self._obj = obj
        self._disp = display

    def __str__(self):
        if self._disp is None:
            self._disp = str(self._obj)
        return self._disp

    def render(self, width, selected):
        lines = str(self).split('\n')
        if selected:
            arr = fsarray('\033[1m\u001b[47m\u001b[34m' + line for line in lines)
        else:
            arr = fsarray(line for line in lines)
        return arr


class ChoiceList:
    def __init__(self, choices, prompt=None, preselected=(), selected=CHECKED, deselected=UNCHECKED):
        if prompt:
            self._prompt = fsarray([(line) for line in prompt.split('\n')])
        else:
            self._prompt = prompt
        if not choices:
            raise ValueError('No choices given')

        if isinstance(choices, dict):
            self._choices = [[k in preselected, Choice(v, k)] for k, v in choices.items()]
        else:
            self._choices = [[i in preselected, Choice(c)] for i, c in enumerate(choices)]
        self._sel = selected
        self._des = deselected
        self._idx = 0

    def run(self, window):
        opt_arr = self.render(window.width)
        window.render_to_terminal(opt_arr)
        try:
            with Input() as keyGen:
                for key in keyGen:
                    if key == '<UP>' or key == 'k':
                        self.prev()
                    elif key == '<DOWN>' or key == 'j':
                        self.next()
                    elif key == '<Ctrl-j>':
                        break
                    elif key == '<ESC>':
                        break
                    else:
                        continue
                    window.render_to_terminal(self.render(window.width))
        except KeyboardInterrupt:
            os._exit(0)

        return self.get_selection()

    def toggle(self):
        state = self._choices[self._idx]
        state[0] = not state[0]

    def select(self, index):
        self._idx = index

    def render(self, width):
        max_len = max(len(str(c)) for c in self)
        if max_len > width:
            width = max_len + 3

        arr = fsarray('', width=width)
        if self._prompt:
            arr.rows = self._prompt.rows + arr.rows
        l = len(arr)
        for checked, option in self._choices:
            current = self._choices[self._idx][1] == option
            opt_arr = option.render(width-3, current)
            arr[l:l+len(opt_arr), 2:width] = opt_arr
            state = '> ' if current else '  '
            arr[l:l+1, 0:2] = fsarray([(('\033[1m\u001b[44m\u001b[37m' + state))])
            l += len(opt_arr)
        return arr

    def get_selection(self):
        options = [item[1]._obj for item in self._choices if item[0]]
        if options:
            return options[0]
        return self._choices[self._idx][1]._obj

    def next(self):
        self._idx = min(len(self)-1, self._idx+1)

    def prev(self):
        self._idx = max(0, self._idx-1)

    def __len__(self):
        return len(self._choices)

    def __getitem__(self, key):
        item = self._choices[key]
        return item[1]._obj

    def __setitem__(self, key, value):
        self._choices[key] = [False, Choice(value)]

    def __delitem__(self, key):
        del self._choices[key]

    def __contains__(self, item):
        return item in [i[1]._obj for i in self._choices]


if __name__ == "__main__":
    try:
        home = os.path.expanduser("~")
        with open(home + "/.copycat", "r") as f:
            options = f.read()
            options = options.split("\u200b")

        options = options[-16:][::-1]
        options = [x.strip() for x in options if x.strip()]
        options = list(dict.fromkeys(options))

        if not options:
            print("\nNo CB history found!\n")
            exit()
        choice = choose(options)
        pyperclip.copy(choice)

    except KeyboardInterrupt:
        exit()


