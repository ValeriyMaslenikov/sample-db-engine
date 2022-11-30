#include <std/mem.pat>
#include <std/ptr.pat>
#include <std/io.pat>

#pragma endian little

u32 PAGE_SIZE = 4096;
u32 NODE_HEADER_SIZE = 16;
u32 DB_HEADER_SIZE = 100;

fn format_varint(auto raw_varint) {
  if (raw_varint.first_byte < 251) {
    return raw_varint.first_byte;
  } else {
    return raw_varint.value;
  }
};

struct VariableInt {
  u8 first_byte [[hidden]];
  if (first_byte == 251) {
    u16 value [[hidden]];
  } else if (first_byte == 252) {
    u32 value [[hidden]];
  } else if (first_byte == 253) {
    u64 value [[hidden]];
  } else if (first_byte == 254) {
    u128 value [[hidden]];
  } else {
    u8 value[0] [[hidden]];
  }
} [[transform("format_varint"), format("format_varint")]];


enum NodeType: u32 {
  TableInternal,
  TableLeaf,
};

struct NodeHeader {
  NodeType node_type;
  u32 free_space_start_offset;
  u32 free_space_end_offset;
  u32 elements_count;
  u8 skip[NODE_HEADER_SIZE - sizeof(node_type) - sizeof(free_space_start_offset) - sizeof(free_space_end_offset) - sizeof(elements_count)];
};

fn format_leaf(auto leaf) {
  return std::format(
    "key: {}, length: {}, hash {}",
    leaf.key,
    leaf.length,
    leaf.reference
  );
};

fn relative_to_node(u128 offset) {
  u128 page_id = $ / PAGE_SIZE;
  
  if (page_id == 0) {
    return DB_HEADER_SIZE;
  } else {
    return page_id * PAGE_SIZE;
  }
};

struct LeafKeyRef {
  u32 key;
  u32 length;
  u8 *reference[length]: u32 [[pointer_base("relative_to_node")]];
} [[format("format_leaf")]];

fn format_internal(auto internal) {
  return std::format("divider: {}, page_id: {}", internal.divider, internal.page_id);
};


struct InternalKeyRef {
  u32 divider; 
  u32 page_id;
} [[format("format_internal")]];


fn format_node(auto node) {
  return std::format("{}, len: {}, page: {}", node.node_header.node_type, node.node_header.elements_count, $/4096);
};


struct Node {
  NodeHeader node_header;
  if (node_header.node_type == NodeType::TableLeaf) {
    LeafKeyRef key_ref[node_header.elements_count];
  } else {
    InternalKeyRef key_ref[node_header.elements_count];
  }
}[[format("format_node")]];

  

struct DBHeader {
  // The value should be always equal to MAGIC_HEADER_STRING constant
  char magic_header_string[18];
  u32 page_size_bytes;
  u32 pages_count;
  // Page where the root of the tree starts, it's 0 after the init, but the id is changed 
  // after the first split
  u32 root_page_id;
};

struct Page {
  Node node;
  u8 heap[PAGE_SIZE-$%PAGE_SIZE] 
  [[comment("This is the data area, in leafs the data is stored here. In internal it's free space for new pointers")]];
};



DBHeader db_header @0;
Page pages[db_header.pages_count ] @ DB_HEADER_SIZE;