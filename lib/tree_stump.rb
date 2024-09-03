# frozen_string_literal: true

require_relative "tree_stump/version"

module TreeStump
  class Error < StandardError;
    def initialize(msg)
      super(msg)
    end
  end

  class QueryError < Error; end
end

require_relative "tree_stump/tree_stump"
