require "tempfile"

RSpec.describe TreeHouse do
  before(:all) do
    TreeHouse.register_lang("ruby", tree_sitter_ruby_path)
  end

  it "can register lang" do
    expect(TreeHouse.available_langs).to include("ruby")
  end

  let(:parser) do
    TreeHouse::Parser.new.tap do |p|
      p.set_language("ruby")
    end
  end
  let(:source) do
    <<~RUBY
    class Hoge
      def hello
        puts "hogehoge"
      end

      def foo
        puts "foo"
        "hogehoge".upcase
      end
    end

    Hoge.new.hello
    RUBY
  end

  it "can parse code" do
    tree = parser.parse(source)
    expect(tree).to be_a(TreeHouse::Tree)

    root_node = tree.root_node
    expect(root_node).to be_a(TreeHouse::Node)
  end

  it "can print_dot_graph" do
    tree = parser.parse(source)

    Tempfile.create("print_dot_graph") do |f|
      tree.print_dot_graph(f)
      f.flush
      expect(File.read(f)).not_to be_empty
    end
  end

  describe "TreeHouse::Node" do
    let(:node) do
      parser.parse(source).root_node
    end

    describe "#kind" do
      it "returns the node's kind" do
        expect(node.kind).to eq("program")
      end
    end

    describe "#kind_id" do
      it "returns the node's kind id" do
        expect(node.kind_id).to eq(159)
      end
    end

    describe "#grammar_id" do
      it "returns the node's grammar id" do
        expect(node.grammar_id).to eq(159)
      end
    end

    describe "#grammar_name" do
      it "returns the node's grammar name" do
        expect(node.grammar_name).to eq("program")
      end
    end

    describe "#is_named?" do
      it "returns true if the node is named" do
        expect(node.is_named?).to be_truthy
      end
    end

    describe "#is_extra?" do
      it "returns true if the node is extra" do
        expect(node.is_extra?).to be_falsey
      end
    end

    describe "#has_error?" do
      it "returns true if the node has error" do
        expect(node.has_error?).to be_falsey
      end

      context "when the node has error" do
        let(:source) { "class Hoge" }

        it "returns true" do
          expect(node.has_error?).to be_truthy
        end
      end
    end

    describe "#is_error?" do
      it "returns true if the node is error" do
        expect(node.is_error?).to be_falsey
      end
    end

    describe "#start_byte" do
      it "returns the node's start byte" do
        expect(node.start_byte).to eq(0)
      end
    end

    describe "#end_byte" do
      it "returns the node's end byte" do
        expect(node.end_byte).to eq(source.bytesize)
      end
    end

    describe "#byte_range" do
      it "returns the node's byte range" do
        expect(node.byte_range).to eq(0...source.bytesize)
      end
    end

    describe "#range" do
      it "returns the node's range" do
        expect(node.range).to eq(TreeHouse::Range.new(0, 123, TreeHouse::Point.new(0, 0),  TreeHouse::Point.new(12, 0)))
      end
    end

    describe "#start_position" do
      it "returns the node's start position" do
        expect(node.start_position).to eq(TreeHouse::Point.new(0, 0))
      end
    end

    describe "#end_position" do
      it "returns the node's end position" do
        expect(node.end_position).to eq(TreeHouse::Point.new(12, 0))
      end
    end

    describe "#child" do
      it "returns the node's child" do
        expect(node.child(0)).to be_a(TreeHouse::Node)
        expect(node.child(0).kind).to eq("class")
        expect(node.child(0).child(0).kind).to eq("class") # "class" keyword

        expect(node.child(1)).to be_a(TreeHouse::Node)
        expect(node.child(1).kind).to eq("call")
      end
    end

    describe "#child_count" do
      it "returns the node's child count" do
        expect(node.child_count).to eq(2)
        expect(node.child(0).child_count).to eq(4)
      end
    end

    describe "#named_child" do
      it "returns the node's named child" do
        expect(node.named_child(0)).to be_a(TreeHouse::Node)
        expect(node.named_child(0).kind).to eq("class")
        expect(node.named_child(0).named_child(0).kind).to eq("constant")
      end
    end

    describe "#named_child_count" do
      it "returns the node's named child count" do
        expect(node.named_child_count).to eq(2)
        expect(node.child(0).named_child_count).to eq(2)
      end
    end

    describe "#child_by_field_name" do
      it "returns the node's child by field name" do
        expect(node.child(0).child_by_field_name("name").kind).to eq("constant")
        expect(node.child(0).child_by_field_name("body").kind).to eq("body_statement")
      end
    end

    describe "#child_by_field_id" do
      it "returns the node's child by field id" do
        language = node.language
        field_id = language.field_id_for_name("name")
        expect(node.child(0).child_by_field_id(field_id).kind).to eq("constant")
      end
    end

    describe "#children" do
      it "yields the node's children with cursor" do
        result = []
        node.children do |child|
          expect(child).to be_a(TreeHouse::Node)
          result << child
        end
        expect(result.size).to eq(2)
        expect(result[0].kind).to eq("class")
        expect(result[1].kind).to eq("call")
      end
    end

    describe "#children_with_cursor" do
      it "yields the node's children with cursor" do
        cursor = node.walk
        result = []
        node.children_with_cursor(cursor) do |child|
          expect(child).to be_a(TreeHouse::Node)
          result << child
        end
        expect(result.size).to eq(2)
        expect(result[0].kind).to eq("class")
        expect(result[1].kind).to eq("call")
      end
    end

    describe "#parent" do
      it "returns the node's parent" do
        expect(node.child(0).parent).to eq(node)
      end
    end

    describe "#child_containing_descendant" do
      it "returns the node's child containing descendant" do
        expect(node.child_containing_descendant(node.child(0).child(0))).to eq(node.child(0))
      end
    end

    describe "#next_sibling" do
      it "returns the node's next sibling" do
        expect(node.child(0).next_sibling).to eq(node.child(1))
      end
    end

    describe "#prev_sibling" do
      it "returns the node's prev sibling" do
        expect(node.child(1).prev_sibling).to eq(node.child(0))
      end
    end

    describe "#next_named_sibling" do
      it "returns the node's next named sibling" do
        expect(node.child(0).next_named_sibling).to eq(node.child(1))
      end
    end

    describe "#prev_named_sibling" do
      it "returns the node's prev named sibling" do
        expect(node.child(1).prev_named_sibling).to eq(node.child(0))
      end
    end

    describe "#descendant_count" do
      it "returns the node's descendant count" do
        expect(node.descendant_count).to eq(44)
      end
    end

    describe "#descendant_for_byte_range" do
      it "returns the node's descendant for byte range" do
        method_node = node.child(0).child(2).child(0)
        range = method_node.byte_range
        expect(node.descendant_for_byte_range(range.begin, range.end)).to eq(method_node)
      end
    end

    describe "#named_descendant_for_byte_range" do
      it "returns the node's descendant for byte range" do
        method_node = node.child(0).child(2).child(0)
        range = method_node.byte_range
        expect(node.named_descendant_for_byte_range(range.begin, range.end)).to eq(method_node)
      end
    end

    describe "#descendant_for_point_range" do
      it "returns the node's descendant for byte range" do
        method2_node = node.child(0).child(2).child(1)
        expect(node.descendant_for_point_range([5, 2], [8, 5])).to eq(method2_node)
      end
    end

    describe "#named_descendant_for_point_range" do
      it "returns the node's descendant for byte range" do
        method2_node = node.child(0).child(2).child(1)
        expect(node.named_descendant_for_point_range([5, 2], [8, 5])).to eq(method2_node)
      end
    end

    describe "#to_sexp" do
      it "returns the node's sexp" do
        expect(node.to_sexp).to match(/\(program .*\)/)
      end
    end

    describe "#utf8_text" do
      it "returns the node's utf8 text" do
        expect(node.child(0).child(1).utf8_text(source)).to eq("Hoge")
      end
    end

    describe "#inspect" do
      it "returns the node's inspect" do
        expect(node.inspect).to eq("{Node program (0, 0) - (12, 0)}")
      end
    end

    describe "#to_s" do
      it "returns the node's to_s" do
        expect(node.to_s).to eq(node.to_sexp)
      end
    end
  end

  describe "TreeHouse::Query" do
    let(:query_str) { "(class (constant) @class_name (body_statement) @body)" }
    it "can build query" do
      query = parser.build_query(query_str)
      expect(query).to be_a(TreeHouse::Query)
    end

    it "detect wrong query" do
      expect { parser.build_query("(invalid_node)") }.to raise_error(TreeHouse::Error)
    end

    it "#start_byte_for_pattern" do
      query = parser.build_query(query_str)
      expect(query.start_byte_for_pattern(0)).to eq(0)
    end

    it "#pattern_count" do
      query = parser.build_query(query_str)
      expect(query.pattern_count).to eq(1)
    end

    it "#capture_names" do
      query = parser.build_query(query_str)
      expect(query.capture_names).to eq(["class_name", "body"]);
    end

    it "#capture_quantifiers" do
      query = parser.build_query(query_str)
      expect(query.capture_quantifiers(0)).to eq([:One, :One]);
    end

    it "#capture_quantifiers with `*`" do
      query = parser.build_query("(class (constant) @class_name (body_statement)* @body)")
      expect(query.capture_quantifiers(0)).to eq([:One, :ZeroOrMore]);
    end

    it "#capture_quantifiers with `+`" do
      query = parser.build_query("(class (constant) @class_name (body_statement)+ @body)")
      expect(query.capture_quantifiers(0)).to eq([:One, :OneOrMore]);
    end

    it "#capture_index_for_name" do
      query = parser.build_query(query_str)
      expect(query.capture_index_for_name("body")).to eq(1)
    end

    it "#disable_capture" do
      query = parser.build_query(query_str)
      expect { query.disable_capture("constant") }.not_to raise_error
    end

    it "disable_pattern" do
      query = parser.build_query(query_str)
      expect { query.disable_pattern(0) }.not_to raise_error
    end

    it "#is_pattern_rooted" do
      query = parser.build_query(query_str)
      expect(query.is_pattern_rooted(0)).to be_truthy
    end

    it "#is_pattern_guaranteed_at_step" do
      query = parser.build_query(query_str)
      expect(query.is_pattern_guaranteed_at_step(0)).to be_falsey
    end
  end

  describe "TreeHouse::QueryCursor" do
    let(:query_str) { "(class (constant) @class_name (body_statement) @body)" }

    let(:source) do
      <<~RUBY
      class Hoge
        def hello
          puts "hogehoge"
        end

        def foo
          puts "foo"
          "hogehoge".upcase
        end
      end

      class Bar
        def bar
          puts "bar"
        end
      end

      Hoge.new.hello
      RUBY
    end

    it "#set_match_limit" do
      query_cursor = TreeHouse::QueryCursor.new
      expect { query_cursor.set_match_limit(10) }.not_to raise_error
    end

    it "#match_limit" do
      query_cursor = TreeHouse::QueryCursor.new
      expect(query_cursor.match_limit).to eq(2 ** 32 - 1)
      query_cursor.set_match_limit(10)
      expect(query_cursor.match_limit).to eq(10)
    end

    it "can match query" do
      query = parser.build_query(query_str)
      query_cursor = TreeHouse::QueryCursor.new
      root_node = parser.parse(source).root_node
      result = []
      query_cursor.matches(query, root_node, source) do |m|
        expect(m).to be_a(TreeHouse::QueryMatch)
        result << m
      end
      expect(result.size).to eq(2)
      expect(result[0].captures[0].node).to be_a(TreeHouse::Node)
      expect(result[0].captures[0].node.utf8_text(source)).to eq("Hoge")
    end
  end
end
