# Gramify

Pre-calculates letter and n-gram frequency data for text corpora, intended to be used for keyboard layout analysis.

```
Usage: ./target/release/gramify FILE [options]

Options:
    -h, --help          Show usage instructions, then exit
    -o, --output-format json|msgpack
                        Output format
    -i, --input-format json|msgpack|raw
                        Input format
        --letter-threshold NUM
                        Threshold of significance for letters. Letters that
                        appear fewer than NUM times per million will not
                        appear in the output.
        --letter-pattern REGEX
                        Regex pattern for letters. Letters that don't match
                        REGEX will be excluded from output.
        --bigram-threshold NUM
                        Threshold of significance for bigrams. Bigrams that
                        appear fewer than NUM times per million will not
                        appear in the output.
        --bigram-pattern REGEX
                        Regex pattern for bigrams. Bigrams that don't match
                        REGEX will be excluded from output.
        --skipgram-threshold NUM
                        Threshold of significance for skipgrams. Skipgrams
                        that appear fewer than NUM times per million will not
                        appear in the output.
        --skipgram-pattern REGEX
                        Regex pattern for skipgrams. Skipgrams that don't
                        match REGEX will be excluded from output.
        --trigram-threshold NUM
                        Threshold of significance for trigrams. Trigrams that
                        appear fewer than NUM times per million will not
                        appear in the output.
        --trigram-pattern REGEX
                        Regex pattern for trigrams. Trigrams that don't match
                        REGEX will be excluded from output.
```

## Terminology

* Bigram: two consecutive characters
* Skipgram: two characters separated by one character (this is useful for keyboard layout analysis)
* Trigram: three consecutive characters

## Format

Gramify supports JSON and [MessagePack](https://msgpack.org/index.html) input and output formats. The value associated with each n-gram key is the ratio of instances of that n-gram relative to the total number of n-grams in the corpus.

## Ready-to-use Corpora

You can find ready-made frequency data in the `corpora` folder, filtered on "significant" letters and n-grams for size since for keyboard layout analysis, the long tail of infrequent n-grams doesn't really affect the overall score of a layout but takes a long time to evaluate.

* `iweb`: from Shai Coleman's sanitized [corpus used for Colemak](https://colemak.com/Design#Corpus_Resources), 527MB of source text (unfiltered n-gram data also available)
* `xsznix-fb-messages`: Personal Facebook messages sent by the author, 15MB of source text
* `carpalx-books`: from Martin Krzywinski's [corpus used for Carpalx](http://mkweb.bcgsc.ca/carpalx/?download_carpalx), 12MB of source text
* `akl-messages`: Messages from the Alt Keyboard Layout Discord server, 9MB of source text
* `typeracer`: Quotes from [Typeracer](https://play.typeracer.com/), 2MB of source text

## License

The frequency analyses in the `corpora` folder are for research and education purposes only.

I don't care what you do with the code in this repo.