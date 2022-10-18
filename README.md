rinku
=====

<img
    align="right"
    width="150"
    height="150"
    src="https://i.postimg.cc/Gtv847M9/link-1.png"
    alt="rinku logo">

*rinku* is Hepburn romanization of リンク that is 外来語 of *Link*
the main protagonist of Nintendo's video game series _The Legend of Zelda_

This program ensures that required links exist and point to correct files/dirs
according provided configuration file

Example of configuration file
-----------------------------
```toml
[[link]]
source = 'vimrc'
target.windows = [
  'vimfiles/vimrc',
  'AppData\Local\nvim\init.vim'
]
target.unix = '.vim/vimrc'
```

