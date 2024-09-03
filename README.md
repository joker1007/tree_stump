# TreeStump
[![RSpec](https://github.com/joker1007/tree_stump/actions/workflows/rspec.yml/badge.svg)](https://github.com/joker1007/tree_stump/actions/workflows/rspec.yml)

[tree-sitter](https://github.com/tree-sitter/tree-sitter) binding for Ruby written by Rust.

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add tree_stump

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install tree_stump

## Usage

```ruby
require "tree_stump"
require "rouge"

TreeStump.register_lang("ruby", "./libtree-sitter-ruby.so")

parser = TreeStump::Parser.new
parser.set_language("ruby")

source = File.read("./sample.rb")

formatter = Rouge::Formatters::Terminal256.new
lexer = Rouge::Lexers::Ruby.new

puts "== Source =="
puts formatter.format(lexer.lex(source))

puts "\n"

puts "==  Tree  =="
tree = parser.parse(source)

puts tree.root_node.to_sexp
```

```
(program (class name: (constant) superclass: (superclass (constant)) body: (body_statement (call method: (identifier) arguments: (argument_list (simple_symbol) (pair key: (hash_key_symbol) value: (true)))) (singleton_method object: (self) name: (identifier) parameters: (method_parameters (identifier)) body: (body_statement (call receiver: (constant) method: (identifier) arguments: (argument_list (pair key: (hash_key_symbol) value: (identifier)))))))) (call receiver: (constant) method: (identifier) arguments: (argument_list (string (string_content)))))
```

## Development

### Requirements

- Rust Toolchain
- tree-sitter-ruby

1. Download source of tree-sitter-ruby from [GitHub Repository](https://github.com/tree-sitter/tree-sitter-ruby).
1. Extract tree-sitter-ruby source
1. mv tree-sitter-ruby-v{version_num} to tree_stump/tree-sitter-ruby
1. Execute `make` in tree-sitter-ruby directory

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/joker1007/tree_stump.
