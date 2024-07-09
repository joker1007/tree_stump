RSpec.describe TreeHouse do
  it "has a version number" do
    expect(TreeHouse::VERSION).not_to be nil
  end

  it "can register lang" do
    TreeHouse.register_lang("ruby", "/home/joker/.local/share/nvim/lazy/nvim-treesitter/parser/ruby.so")
    expect(TreeHouse.available_langs).to include("ruby")
  end

  it "can create parser" do
    TreeHouse.register_lang("ruby", "/home/joker/.local/share/nvim/lazy/nvim-treesitter/parser/ruby.so")
    parser = TreeHouse::Parser.new
    parser.set_language("ruby")
    tree = parser.parse(
      <<~RUBY
        def hello
          puts "hogehoge"
        end

        def foo
          puts "foo"
        end
      RUBY
    )
    cursor = tree.walk
    tree.root_node.children_with_cursor(cursor) do |node|
      p node
    end
  end
end
