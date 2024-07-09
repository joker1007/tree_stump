# frozen_string_literal: true

require "bundler/gem_tasks"
require "rb_sys/extensiontask"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

task build: :compile

GEMSPEC = Gem::Specification.load("tree_house.gemspec")

RbSys::ExtensionTask.new("tree_house", GEMSPEC) do |ext|
  ext.lib_dir = "lib/tree_house"
end

task spec: :compile
task default: :spec
