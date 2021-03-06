# how2-rs

Simple CLI tool to retrieve answers from StackExchange.

Inspired by [how2](https://github.com/santinic/how2) written in JS on node.js.

Main reason to reimplement in Rust - absence of desire to have node.js installed
on my machine.

Another one - I want this tool to be more editor-friendly (without fancy UI and spinners).

# Installation

1. Just download pre-built binary from `build` folder
(don't forget to check for viruses). 

2. Assuming you have Rustc and Cargo installed.

``` console
  git clone https://github.com/0nkery/how2-rs/
  cd how2-rs
  cargo build --release
  # OR
  cargo install
```

# Usage

``` console
  how2-rs [options] any googling query
```

Available options:

  - -m, --max-answers 

    Maximum answers to retrieve (defaults to 5).
    
  - -j, --json
  
    Return answers json-encoded (defaults to false).
  
  - -h, --help

    Show this message and exit.
    
# Contribution

If you have some insights how make my Rust code
more idiomatic, **you're welcome**!

If you want to add some fancy functionality, **you're welcome**!

Just open Issue or make PR.

  
