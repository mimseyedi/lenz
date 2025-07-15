<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/mimseyedi/lenz/master/docs/lenz-dark.png">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/mimseyedi/lenz/master/docs/lenz-light.png">
    <img alt="Lenz"
         src="https://raw.githubusercontent.com/mimseyedi/lenz/master/docs/lenz-light.png"
         width="50%">
  </picture>

[Introduction](#intro) | [Usage](#usage) | [Help](#help) | [Contributing](#cont)
</div>

`lenz` is a command-line utility for searching and Browse files. It allows users to quickly find specific text patterns within files, with options for case-insensitive matching, counting occurrences, and different display modes.

## Introduction <a class="anchor" id="intro"></a>
`lenz` is a personal and practical project for learning the Rust programming language better. Lenz is definitely not meant to compete with big tools like `grep` and its variants, it is just a practical and educational project.

## Usage <a class="anchor" id="usage"></a>
```
$ lenz "query" /path/to/your/file.txt /path/to/another/file.txt ...
```

## Help <a class="anchor" id="help"></a>
The best guide can be the (`-h`, `--help`) option.

```
Usage: lenz [QUERY] [FILEs]... [OPTION]

  Lenz is a simple CLI for browsing files to find words.

Options:
  -i, --ignore-case        Perform case insensitive matching.
  -c, --count              Counting matches in files.
  -p, --page-view          Page view.
  -h, --help               Show this message and exit.
  -v, --version            Display version and exit.
```

## Contributing <a class="anchor" id="cont"></a>
Students and Rust enthusiasts who have just started learning the language can participate in this project to learn more and understand the development of Rust projects and open source projects.

You can submit a [pull request](https://github.com/mimseyedi/lenz/pulls) to participate.