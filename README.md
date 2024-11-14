# book_ideas

## Epub Notes:
### Epub command stuff:
[epubcheck](https://github.com/w3c/epubcheck) is the actual command, but I have it aliased as `epub` on one of my machines.

Note: You will have to manually install the command to get an up to date version of it. This will require instAalling Java, Mavan and creating an alias that looks like this: `alias epub='java -jar ~/Github/epubcheck/target/epubcheck.jar'`


### Check epub format:
```
epub practical_sorting_in_rust/ --mode exp -v 3.0
```

### Create new version of Epub:
```
epub practical_sorting_in_rust/ --mode exp -v 3.0 --save
```

## Terminal hacks
### View xhtml file in the terminal
Install [pandoc](https://pandoc.org/index.html) and use it to convert your file to plain text.
```
cat practical_sorting_in_rust/EPUB/ch10.xhtml | pandoc -f html  -t plain
```

With this trick, you can pipe the plain text version to wc and start gathering metrics.

```
cat practical_sorting_in_rust/EPUB/ch10.xhtml | pandoc -f html  -t plain | wc
```
