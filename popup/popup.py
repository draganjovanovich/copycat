#!/usr/bin/python3

import sys
import os
import pyperclip
from curtsies import Input, CursorAwareWindow, fsarray

CHECKED = '\u25c9 '
UNCHECKED = '\u25cc '
MAX_ITEMS = 500


def choose(choices, preselected=()):
    choice_list = ChoiceList(choices, preselected=preselected)
    with CursorAwareWindow(out_stream=sys.stderr,
                           extra_bytes_callback=lambda x: x) as window:
        options = choice_list.run(window)
    return options


class Choice:
    __slots__ = ['_obj', '_disp', '_disp_lower', '_preview', '_scroll']

    def __init__(self, obj, display=None):
        self._obj = obj
        self._disp = display
        self._disp_lower = None
        self._preview = False
        self._scroll = 0

    def __str__(self):
        if self._disp is None:
            self._disp = str(self._obj)
        return self._disp

    def matches(self, query):
        if not query:
            return True
        if self._disp_lower is None:
            self._disp_lower = str(self).lower()
        return query.lower() in self._disp_lower

    def render(
            self,
            width,
            selected,
            preview,
            scroll,
            scroll_to_bottom,
            scroll_to_top,
            index):
        if preview != self._preview:
            self._preview = preview
            self._scroll = 0

        lines = str(self).split('\n')
        lines = [line.rstrip() for line in lines]
        lines = [line.replace('\t', ' ') for line in lines]
        lines = [' ' + line for line in lines]
        max_len = width - 2

        REP_COLOR = selected and '\u001b[30m' or '\u001b[37m'
        preview_height = int(os.get_terminal_size().lines / 2)
        rest_lines = len(lines) - preview_height

        if selected:
            lines = [
                f'\u001b[0m\033[1m\u001b[47m\u001b[34m{line}' for line in lines]
        elif not selected:
            lines = [
                f'\u001b[0m\033[1m\u001b[0m\u001b[0m{line}' for line in lines]

        if selected and preview:
            lines = [
                f'\u001b[44m\u001b[36m{i+1:3}\u001b[0m\033[1m\u001b[47m\u001b[34m{line}' for i,
                line in enumerate(lines)]

        if rest_lines > 0:
            if self._scroll + scroll > -1 and self._scroll + scroll < rest_lines:
                self._scroll += scroll

            if scroll_to_bottom:
                self._scroll = rest_lines - 1
            if scroll_to_top:
                self._scroll = 0

            if preview and selected:
                scroll_tip_height = int(
                    (preview_height / rest_lines) * preview_height)
                if scroll_tip_height == 0:
                    scroll_tip_height = 1
                scroll_percent = int(
                    (self._scroll / rest_lines) * (preview_height))
                scroll_tip_pos = int(
                    (scroll_percent / preview_height) * (preview_height - scroll_tip_height)) + 1

                if scroll_tip_pos == 0:
                    scroll_tip_pos = 1

                if scroll_tip_pos + self._scroll > 0:
                    for i in range(scroll_tip_height):
                        idx = scroll_tip_pos + self._scroll + i
                        if idx > (len(lines) - 1) or idx < 0:
                            idx = 0

                        lines[idx] = lines[idx] + ' ' * (max_len - len(lines[idx]))
                        lines[idx] = lines[idx][:max_len - 1] + U'\u2589'

                if self._scroll > 0:
                    lines[0 + self._scroll] = lines[0 + self._scroll] + \
                        ' ' * (max_len - len(lines[0 + self._scroll]))
                    lines[0 + self._scroll] = lines[0 +
                                                    self._scroll][:max_len - 1] + '\u25b2'
                else:
                    lines[0 + self._scroll] = lines[0 + self._scroll] + \
                        ' ' * (max_len - len(lines[0 + self._scroll]))
                    lines[0 + self._scroll] = lines[0 +
                                                    self._scroll][:max_len - 1] + '\u25b3'

                if self._scroll + 1 < rest_lines:
                    lines[preview_height + self._scroll] = lines[preview_height + self._scroll] + \
                        ' ' * (max_len - len(lines[preview_height + self._scroll]))
                    lines[preview_height + self._scroll] = lines[preview_height +
                                                                 self._scroll][:max_len - 1] + '\u25bc'
                else:
                    lines[preview_height + self._scroll] = lines[preview_height + self._scroll] + \
                        ' ' * (max_len - len(lines[preview_height + self._scroll]))
                    lines[preview_height + self._scroll] = lines[preview_height +
                                                                 self._scroll][:max_len - 1] + '\u25bd'

        if len(lines) > 1:
            if preview is True and selected is True:
                if self._scroll > 0:
                    lines = lines[self._scroll:]
                elif self._scroll < 0:
                    lines = lines[:self._scroll]
                lines = lines[:preview_height + 1]
            else:
                rep = '{} ({} more lines in buffer...)'.format(
                    REP_COLOR, len(lines))
                if len(lines[0]) > max_len - len(rep):
                    lines = [lines[0][:max_len - len(rep)] + rep]
                else:
                    lines = [lines[0] + rep]
        else:
            preview = False

        lines = [line + ' ' * (max_len - len(line)) for line in lines]
        lines = [line[:width - 2] for line in lines]
        arr = fsarray([('\u001b[44m\u001b[01m{:2} \u001b[0m').format(
            i == 0 and index or '') + line for i, line in enumerate(lines)])

        return arr


class ChoiceList:
    def __init__(
            self,
            choices,
            prompt=None,
            preselected=(),
            selected=CHECKED,
            deselected=UNCHECKED):
        if prompt:
            self._prompt = fsarray([(line) for line in prompt.split('\n')])
        else:
            self._prompt = prompt
        if not choices:
            raise ValueError('No choices given')

        if isinstance(choices, dict):
            self._choices = [[k in preselected, Choice(
                v, k)] for k, v in choices.items()]
        else:
            self._choices = [[i in preselected, Choice(
                c)] for i, c in enumerate(choices)]
        self._sel = selected
        self._des = deselected
        self._idx = 0

        self._preview = False
        self._scroll = 0
        self._scroll_to_bottom = False
        self._scroll_to_top = False
        self._gg = 0

        self._search_query = ""
        self._search_mode = False
        self._filtered_indices = list(range(len(self._choices)))
        self._viewport_start = 0
        self._visible_count = 16

    def _update_filter(self):
        if not self._search_query:
            self._filtered_indices = list(range(len(self._choices)))
        else:
            self._filtered_indices = [
                i for i, (_, choice) in enumerate(self._choices)
                if choice.matches(self._search_query)
            ]
        if self._filtered_indices:
            if self._idx >= len(self._filtered_indices):
                self._idx = 0
        else:
            self._idx = 0
        self._viewport_start = 0

    def _adjust_viewport(self):
        if self._idx < self._viewport_start:
            self._viewport_start = self._idx
        elif self._idx >= self._viewport_start + self._visible_count:
            self._viewport_start = self._idx - self._visible_count + 1
        max_start = max(0, len(self._filtered_indices) - self._visible_count)
        self._viewport_start = min(self._viewport_start, max_start)
        self._viewport_start = max(0, self._viewport_start)

    def _get_filtered_choice(self, filtered_idx):
        if 0 <= filtered_idx < len(self._filtered_indices):
            actual_idx = self._filtered_indices[filtered_idx]
            return self._choices[actual_idx]
        return None

    def run(self, window):
        opt_arr = self.render(
            window.width,
            self._preview,
            self._scroll,
            self._scroll_to_bottom,
            self._scroll_to_top)
        window.render_to_terminal(opt_arr)
        try:
            with Input() as keyGen:
                for key in keyGen:
                    if self._search_mode:
                        if key == '<ESC>':
                            self._search_mode = False
                        elif key == '<Ctrl-j>' or key == '<RETURN>':
                            self._search_mode = False
                        elif key == '<BACKSPACE>' or key == '<DELETE>':
                            if self._search_query:
                                self._search_query = self._search_query[:-1]
                                self._update_filter()
                        elif key == '<SPACE>':
                            self._search_query += ' '
                            self._update_filter()
                        elif len(key) == 1 and key.isprintable():
                            self._search_query += key
                            self._update_filter()
                        window.render_to_terminal(
                            self.render(
                                window.width,
                                self._preview,
                                self._scroll,
                                self._scroll_to_bottom,
                                self._scroll_to_top))
                        continue

                    if key == '<UP>' or key == 'k':
                        self._scroll = -1
                        if self._preview is False:
                            self.prev()
                    elif key == '<DOWN>' or key == 'j':
                        self._scroll = 1
                        if self._preview is False:
                            self.next()
                    elif key == '<Ctrl-j>' or key == '<RETURN>':
                        self._preview = False
                        break
                    elif key == 'p' and len(self.get_selection().split('\n')) > 1:
                        self._preview = not self._preview
                        self._scroll_to_bottom = False
                        self._scroll_to_top = False
                    elif key == '<ESC>':
                        if self._search_query:
                            self._search_query = ""
                            self._update_filter()
                        elif not self._preview:
                            break
                        self._preview = False
                    elif key == 'q' or key == 'Q':
                        if not self._preview:
                            break
                        self._preview = False
                    elif key == '/':
                        self._search_mode = True
                        self._search_query = ""
                    elif key == 'n':
                        if self._search_query and len(self._filtered_indices) > 1:
                            self._idx = (self._idx + 1) % len(self._filtered_indices)
                            self._adjust_viewport()
                    elif key == 'N':
                        if self._search_query and len(self._filtered_indices) > 1:
                            self._idx = (self._idx - 1) % len(self._filtered_indices)
                            self._adjust_viewport()
                    elif key == 'G':
                        self._scroll_to_bottom = True
                        if not self._preview and self._filtered_indices:
                            self._idx = len(self._filtered_indices) - 1
                            self._adjust_viewport()
                    elif key == 'g':
                        self._gg += 1
                        if self._gg == 2:
                            self._scroll_to_top = True
                            self._gg = 0
                            if not self._preview:
                                self._idx = 0
                                self._viewport_start = 0
                    elif key == '<Ctrl-u>':
                        page_size = max(1, self._visible_count // 2)
                        self._idx = max(0, self._idx - page_size)
                        self._adjust_viewport()
                    elif key == '<Ctrl-d>':
                        page_size = max(1, self._visible_count // 2)
                        self._idx = min(len(self._filtered_indices) - 1, self._idx + page_size)
                        self._adjust_viewport()
                    else:
                        continue
                    window.render_to_terminal(
                        self.render(
                            window.width,
                            self._preview,
                            self._scroll,
                            self._scroll_to_bottom,
                            self._scroll_to_top))
                    self._scroll = 0
                    self._scroll_to_bottom = False
                    self._scroll_to_top = False
        except KeyboardInterrupt:
            os._exit(0)

        return self.get_selection()

    def toggle(self):
        choice = self._get_filtered_choice(self._idx)
        if choice:
            choice[0] = not choice[0]

    def select(self, index):
        self._idx = index

    def render(self, width, preview, scroll, scroll_to_bottom, scroll_to_top):
        self._adjust_viewport()
        arr = fsarray('', width=width)

        search_bar = ""
        if self._search_mode:
            search_bar = f"\u001b[44m\u001b[37m/\u001b[0m\u001b[47m\u001b[30m {self._search_query}\u001b[5m_\u001b[0m"
        elif self._search_query:
            match_count = len(self._filtered_indices)
            total = len(self._choices)
            search_bar = f"\u001b[44m\u001b[37m Search: {self._search_query} ({match_count}/{total} matches) \u001b[0m"

        if search_bar:
            search_arr = fsarray([search_bar + ' ' * (width - len(self._search_query) - 30)])
            arr.rows = search_arr.rows + arr.rows

        if self._prompt:
            arr.rows = self._prompt.rows + arr.rows
        l = len(arr)

        total_items = len(self._filtered_indices)
        visible_end = min(self._viewport_start + self._visible_count, total_items)
        show_scrollbar = total_items > self._visible_count

        for view_idx, filtered_idx in enumerate(range(self._viewport_start, visible_end)):
            actual_idx = self._filtered_indices[filtered_idx]
            checked, option = self._choices[actual_idx]
            current = filtered_idx == self._idx
            opt_arr = option.render(
                width - 4 if show_scrollbar else width - 3,
                current,
                preview,
                scroll,
                scroll_to_bottom,
                scroll_to_top,
                filtered_idx + 1)
            arr[l:l + len(opt_arr), 2:width - (1 if show_scrollbar else 0)] = opt_arr
            state = '> ' if current else '  '
            arr[l:l + 1,
                0:2] = fsarray([(('\033[1m\u001b[44m\u001b[37m' + state))])

            if show_scrollbar:
                scrollbar_height = max(1, int(self._visible_count * self._visible_count / total_items))
                scrollbar_pos = int(self._viewport_start * (self._visible_count - scrollbar_height) / max(1, total_items - self._visible_count))
                if scrollbar_pos <= view_idx < scrollbar_pos + scrollbar_height:
                    arr[l:l + 1, width - 1:width] = fsarray(['\u001b[44m\u2588\u001b[0m'])
                else:
                    arr[l:l + 1, width - 1:width] = fsarray(['\u001b[44m\u2502\u001b[0m'])

            l += len(opt_arr)

        if not self._filtered_indices and self._search_query:
            no_results = f"\u001b[33m  No matches for '{self._search_query}'\u001b[0m"
            arr[l:l + 1, 0:width] = fsarray([no_results])

        if total_items > 0:
            status = f"\u001b[44m\u001b[37m [{self._idx + 1}/{total_items}] \u001b[0m"
            arr[l:l + 1, 0:width] = fsarray([status])

        return arr

    def get_selection(self):
        choice = self._get_filtered_choice(self._idx)
        if choice:
            return choice[1]._obj
        options = [item[1]._obj for item in self._choices if item[0]]
        if options:
            return options[0]
        if self._choices:
            return self._choices[0][1]._obj
        return None

    def next(self):
        if self._filtered_indices:
            self._idx = min(len(self._filtered_indices) - 1, self._idx + 1)
            self._adjust_viewport()

    def prev(self):
        self._idx = max(0, self._idx - 1)
        self._adjust_viewport()

    def __len__(self):
        return len(self._filtered_indices)

    def __getitem__(self, key):
        choice = self._get_filtered_choice(key)
        if choice:
            return choice[1]._obj
        return None

    def __setitem__(self, key, value):
        if 0 <= key < len(self._filtered_indices):
            actual_idx = self._filtered_indices[key]
            self._choices[actual_idx] = [False, Choice(value)]

    def __delitem__(self, key):
        if 0 <= key < len(self._filtered_indices):
            actual_idx = self._filtered_indices[key]
            del self._choices[actual_idx]
            self._update_filter()

    def __contains__(self, item):
        return item in [i[1]._obj for i in self._choices]


if __name__ == "__main__":
    try:
        home = os.path.expanduser("~")
        with open(home + "/.copycat", "r") as f:
            options = f.read()
            options = options.split("\u200b")

        options = options[-MAX_ITEMS:][::-1]
        options = [x.strip() for x in options if x.strip()]
        seen = {}
        unique_options = []
        for opt in options:
            if opt not in seen:
                seen[opt] = True
                unique_options.append(opt)
        options = unique_options

        if not options:
            print("\nNo CB history found!\n")
            exit()
        choice = choose(options)
        pyperclip.copy(choice)

    except KeyboardInterrupt:
        exit()
