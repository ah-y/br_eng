use std::collections::HashMap;

pub type AttrMap = HashMap<String, String,>;

//#[derive(Debug,)]
pub struct Node {
   //data common to all nodes
   children:  Vec<Node,>,
   //data specific to each node type
   node_type: NodeType,
}

//#[derive(Debug,)]
enum NodeType {
   Text(String,),
   Element(ElementData,),
}

//#[derive(Debug,)]
struct ElementData {
   tag_name:   String,
   attributes: AttrMap,
}

pub fn text(data: String,) -> Node {
   Node { children: Vec::new(), node_type: NodeType::Text(data,), }
}
pub fn elem(name: String, attrs: AttrMap, children: Vec<Node,>,) -> Node {
   Node {
      children,
      node_type: NodeType::Element(ElementData { tag_name: name, attributes: attrs, },),
   }
}

//pub fn prettyprint(nod: Node,) { println!("{:#?}", nod) }
