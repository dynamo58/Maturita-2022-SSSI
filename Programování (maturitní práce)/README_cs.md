[🇬🇧 english](https://github.com/dynamo58/gol/blob/master/README.md)

---

# O projektu
Tento repozitář je místem pro vše soubory týkající se mé maturitní práce.

Skládá se ze dvou hlavních částí:
1) dokumenty:
    * `docs/text.tex` - popisuje [programovací jazyk Rust](https://www.rust-lang.org) společně s věcmi sdruženými, a také popis vytvořené aplikace (2↓)
    * `docs/presentation.tex` - prezentace pro obhajobu maturitní práce
2) vytvořená aplikace - implementace [Hry života](https://cs.wikipedia.org/wiki/Hra_%C5%BEivota) (`code/`)

# Připravení projektu pro vývoj
Spusťte `setup.py` skript (jen pro GNU/Linux) \[ WIP \]

# Stavění
Spusťte `build.py` skript;

cli argumenty:
* `-os` - cílové OS (linux, mac, win)
* `-arch` - cílová architektura (wasm, ...])

(build tools pro danou architekturu musí samozřejmě být nainstalovány)

př.:
```./build.py -arch wasm```
```./build.py -os win -arch i686-pc-windows-msvc```

[...] nebo samozřejmě můžete postavit sami, dle sebe ¯\\\_(ツ)\_/¯