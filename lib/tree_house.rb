# frozen_string_literal: true

require_relative "tree_house/version"

module TreeHouse
  class Error < StandardError;
    def initialize(msg)
      super(msg)
    end
  end

  class QueryError < Error; end
end

require_relative "tree_house/tree_house"
