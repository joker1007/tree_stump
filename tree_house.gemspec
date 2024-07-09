# frozen_string_literal: true

require_relative "lib/tree_house/version"

Gem::Specification.new do |spec|
  spec.name = "tree_house"
  spec.version = TreeHouse::VERSION
  spec.authors = ["joker1007"]
  spec.email = ["kakyoin.hierophant@gmail.com"]

  spec.summary = "Ruby bindings for Tree-sitter written in Rust"
  spec.description = "Ruby bindings for Tree-sitter written in Rust"
  spec.homepage = "https://github.com/joker1007/tree_house"
  spec.required_ruby_version = ">= 3.1.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(%w[git ls-files -z], chdir: __dir__, err: IO::NULL) do |ls|
    ls.readlines("\x0", chomp: true).reject do |f|
      (f == gemspec) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git appveyor Gemfile])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/tree_house/Cargo.toml"]

  # Uncomment to register a new dependency of your gem
  spec.add_development_dependency "rspec"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
