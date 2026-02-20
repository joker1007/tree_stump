# frozen_string_literal: true

source "https://rubygems.org"

# Specify your gem's dependencies in tree_stump.gemspec
gemspec

gem "rake", "~> 13.0"

gem "rake-compiler"

# Ruby 4.0 compat
# ruby 4.1 HEAD compat requires unreleased changes
if RUBY_VERSION >= "4.1.0"
  # Use rb_sys from GitHub for Ruby 4.1+ (ruby-head), pinned to a specific commit for reproducibility
  gem "rb_sys",
    github: "oxidize-rb/rb-sys",
    branch: "main",
    ref: "5e2978121cd809d33fd1aeca54c685ebe6dc01de"
else
  # Use released rb_sys gem for stable Rubies
  # Rely on gemspec
end
