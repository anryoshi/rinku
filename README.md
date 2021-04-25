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

This program links your dotfiles according provided configuration file
(that could be stored along dotfiles themselves)

## Example of configuration file

```toml
[[link]]
source = 'vimrc'
target.windows = [
  'vimfiles/vimrc',
  'AppData\Local\nvim\init.vim'
]
target.unix = '.vim/vimrc'
```
