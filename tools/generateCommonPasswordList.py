#!/usr/bin/env python3

from os import path
from typing import List

import urllib.request
import sys


WORDLISTS = [
  "https://raw.githubusercontent.com/danielmiessler/SecLists/master/Passwords/xato-net-10-million-passwords-1000.txt",
  "https://raw.githubusercontent.com/danielmiessler/SecLists/master/Passwords/darkweb2017-top10000.txt"
]

DEFAULT_DST = path.abspath(path.join(__file__, "../../proton-pass-common", "passwords.txt"))

def download_list(url: str) -> List[str]:
     response = urllib.request.urlopen(url)
     data = response.read()
     text = data.decode('utf-8')
     lines = []
     for line in text.split('\n'):
        stripped = line.strip().replace("'", "")
        if len(stripped) > 3:
            lines.append(stripped.lower())
     return lines

def main(dst: str):
    # Download all the wordlists
    wordlists = [download_list(l) for l in WORDLISTS]

    # Combine all the words
    words = [word for wordlist in wordlists for word in wordlist]

    # Dedup
    deduped_words = sorted(list(set(words)))

    # Write
    with open(dst, 'w') as f:
        f.write("\n".join(deduped_words))

    print(f"Wrote the passwords file to {dst}")

if __name__ == '__main__':
    dst = DEFAULT_DST
    if len(sys.argv) == 2:
        if sys.argv[1] in ["-h", "--help"]:
            print(f"{sys.argv[0]} DST_FILE")
            print(f"(defaults to {DEFAULT_DST})")
            sys.exit(0)
        else:
            dst = sys.argv[1]
    if len(sys.argv) > 2:
        print(f"Bad usage:\n\t{sys.argv[0]} DST_FILE")
        sys.exit(1)

    main(dst)