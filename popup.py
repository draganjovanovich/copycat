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
        self._preview = False
        self._scroll = 0

    def __str__(self):
        if self._disp is None:
            self._disp = str(self._obj)
        return self._disp

    def render(self, width, selected, preview, scroll, scroll_to_bottom, scroll_to_top, index):
        if preview != self._preview:
            self._preview = preview
            self._scroll = 0

        lines = str(self).split('\n')
        lines = [line.rstrip() for line in lines]
        # replace TABS with spaces to avoid weird rendering
        lines = [line.replace('\t', ' ') for line in lines]
        # prepend one space for each line
        lines = [' ' + line for line in lines]
        max_len = width
        if max_len + 10 > os.get_terminal_size().columns:
            max_len = os.get_terminal_size().columns - 5

        REP_COLOR = selected == True and '\u001b[30m' or '\u001b[37m'
        preview_height = int(os.get_terminal_size().lines / 2)
        rest_lines = len(lines) - preview_height

        if selected and not preview:
            lines = [f'\u001b[0m\033[1m\u001b[47m\u001b[34m{line}' for line in lines]

        if selected and preview:
            # render line numbers for each line, but without bolding and with balck font
            lines = [f'\u001b[44m\u001b[36m{i+1:3}\u001b[0m\033[1m\u001b[47m\u001b[34m{line}' for i, line in enumerate(lines)]

        # scroll if needed, handle scroll and add scrollbar arrow indicators
        if rest_lines > 0:
            if self._scroll + scroll > -1 and self._scroll + scroll < rest_lines:
                self._scroll += scroll

            if scroll_to_bottom:
                self._scroll = rest_lines - 1
            if scroll_to_top:
                self._scroll = 0

            if preview and selected:
                scroll_percent = int((self._scroll / rest_lines) * (preview_height))
                scroll_tip_pos = int((scroll_percent / preview_height) * (preview_height))
                if scroll_tip_pos == 0:
                    scroll_tip_pos = 1
                if scroll_tip_pos + self._scroll > 0:
                    lines[scroll_tip_pos + self._scroll] = lines[scroll_tip_pos + self._scroll] + ' ' * (max_len - len(lines[scroll_tip_pos + self._scroll]))
                    lines[scroll_tip_pos + self._scroll] = lines[scroll_tip_pos + self._scroll][:max_len-1] + '\u2588'
                if self._scroll > 0:
                    lines[0 + self._scroll] = lines[0 + self._scroll] + ' ' * (max_len - len(lines[0 + self._scroll]))
                    lines[0 + self._scroll] = lines[0 + self._scroll][:max_len - 1] + '\u25b2'
                else:
                    lines[0 + self._scroll] = lines[0 + self._scroll] + ' ' * (max_len - len(lines[0 + self._scroll]))
                    lines[0 + self._scroll] = lines[0 + self._scroll][:max_len-1] + '\u25b3'

                if self._scroll + 1 < rest_lines:
                    lines[preview_height + self._scroll] = lines[preview_height + self._scroll] + ' ' * (max_len - len(lines[preview_height + self._scroll]))
                    lines[preview_height + self._scroll] = lines[preview_height + self._scroll][:max_len-1] + '\u25bc'
                else:
                    lines[preview_height + self._scroll] = lines[preview_height + self._scroll] + ' ' * (max_len - len(lines[preview_height + self._scroll]))
                    lines[preview_height + self._scroll] = lines[preview_height + self._scroll][:max_len-1] + '\u25bd'

        if len(lines) > 1:
            if preview is True and selected is True:
                if self._scroll > 0:
                    lines = lines[self._scroll:]
                elif self._scroll < 0:
                    lines = lines[:self._scroll]
                lines = lines[:preview_height + 1]
            else:
                rep = '{} ({} more lines in buffer...)'.format(REP_COLOR, len(lines))
                if len(lines[0]) > max_len - len(rep):
                    lines = [lines[0][:max_len - len(rep)] + rep]
                else:
                    lines = [lines[0] + rep]
        else:
            preview = False

        # fill up with spaces
        lines = [line + ' ' * (max_len - len(line)) for line in lines]
        # trim lines length to os.get_terminal_size().columns - 2
        lines = [line[:width-2] for line in lines]
        # add index with 3 spaces padding
        arr = fsarray([('\u001b[44m\u001b[01m{:2} \u001b[0m').format(i == 0 and index or '') + line for i, line in enumerate(lines)])

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

        self._preview = False
        self._scroll = 0
        self._scroll_to_bottom = False
        self._scroll_to_top = False
        self._gg = 0
    def run(self, window):
        opt_arr = self.render(window.width, self._preview, self._scroll, self._scroll_to_bottom, self._scroll_to_top, self._idx)
        window.render_to_terminal(opt_arr)
        try:
            with Input() as keyGen:
                for key in keyGen:
                    if key == '<UP>' or key == 'k':
                        self._scroll = -1
                        if self._preview is False:
                            self.prev()
                    elif key == '<DOWN>' or key == 'j':
                        self._scroll = 1
                        if self._preview is False:
                            self.next()
                    elif key == '<Ctrl-j>':
                        self._preview = False
                        break
                    elif key == 'p' and len(self.get_selection().split('\n')) > 1:
                        self._preview = not self._preview
                        self._scroll_to_bottom = False
                        self._scroll_to_top = False
                    elif key == '<ESC>':
                        if not self._preview:
                            break
                        self._preview = False
                    elif key == 'q' or key == 'Q':
                        if not self._preview:
                            break
                        self._preview = False
                    elif key == 'G':
                        self._scroll_to_bottom = True
                    elif key == 'g':
                        self._gg += 1
                        if self._gg == 2:
                            self._scroll_to_top = True
                            self._gg = 0
                    else:
                        continue
                    window.render_to_terminal(self.render(window.width, self._preview, self._scroll, self._scroll_to_bottom, self._scroll_to_top, self._idx))
                    self._scroll = 0
                    self._scroll_to_bottom = False
                    self._scroll_to_top = False
        except KeyboardInterrupt:
            os._exit(0)

        return self.get_selection()

    def toggle(self):
        state = self._choices[self._idx]
        state[0] = not state[0]

    def select(self, index):
        self._idx = index

    def render(self, width, preview, scroll, scroll_to_bottom, scroll_to_top, index):
        arr = fsarray('', width=width)
        if self._prompt:
            arr.rows = self._prompt.rows + arr.rows
        l = len(arr)
        i = 1
        for checked, option in self._choices:
            current = self._choices[self._idx][1] == option
            opt_arr = option.render(width-3, current, preview, scroll, scroll_to_bottom, scroll_to_top, i)
            arr[l:l+len(opt_arr), 2:width] = opt_arr
            state = '> ' if current else '  '
            arr[l:l+1, 0:2] = fsarray([(('\033[1m\u001b[44m\u001b[37m' + state))])
            l += len(opt_arr)
            i += 1
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


