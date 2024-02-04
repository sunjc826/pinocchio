import re

def kebab_to_camel(s): # type: (str) -> str
    return re.sub(r'-(\w)', lambda m: m.group(1).upper(), s.capitalize())

def kebab_to_snake(s): # type: (str) -> str
    return "".join('_' if ch == '-' else ch for ch in s)
