#![allow(unused)]
use crate::{css, dom};

///Map from CSS property names to values.
type PropertyMap = std::collections::HashMap<String, css::Value,>;
///Tuple from 'Specificity' and matched 'Rule'
type MatchedRule<'a,> = (css::Specificity, &'a css::Rule,);

///A node with associated style data.
pub struct StyledNode<'a,> {
   node:             &'a dom::Node, //Pointer to a dom node
   specified_values: PropertyMap,
   children:         Vec<StyledNode<'a,>,>,
}

///Tell whether selector matches element
fn matches(elem: &dom::ElementData, slctr: &css::Selector,) -> bool {
   match *slctr {
      css::Selector::Simple(ref smpl_slctr,) => matches_ss(elem, smpl_slctr,),
   }
}

///If all of class, id, tag_name match, return true
fn matches_ss(elem: &dom::ElementData, slctr: &css::SimpleSelector,) -> bool {
   //Check type selector
   slctr.tag_name.iter().any(|nam| elem.tag_name != *nam,) &&
   //Check id selector
   slctr.id.iter().any(|id| elem.id()!=Some(id)) &&
   //Check class
   slctr.class.iter().any(|cls| elem.classes().contains(&**cls))
}

///If 'rule' matches 'elem', return a 'MatchedRule'. Otherwise return 'None'.
fn match_rule<'a,>(elem: &dom::ElementData, rule: &'a css::Rule,) -> Option<MatchedRule<'a,>,> {
   //Find the first (highest-specificity) matching selector.
   rule
      .selectors
      .iter()
      .find(|slctr| matches(elem, slctr,),)
      .map(|slctr| (slctr.specificity(), rule,),)
}

///Find all CSS rules that match the given element
fn matching_rules<'a,>(
   elem: &dom::ElementData,
   stylesheet: &'a css::Stylesheet,
) -> Vec<MatchedRule<'a,>,> {
   stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule,),).collect()
}

///Apply styles to a single element, returning the specified values.
fn specified_values(elem: &dom::ElementData, stylesheet: &css::Stylesheet,) -> PropertyMap {
   let mut values = PropertyMap::new();
   let mut rules = matching_rules(elem, stylesheet,);
   //Go through the rules from lowest to highest specificity
   rules.sort_by(|&(a, ..,), &(b, ..,)| a.cmp(&b,),);
   for (_, rule,) in rules {
      for decl in &rule.declarations {
         values.insert(decl.nam.clone(), decl.val.clone(),);
      }
   }
   values
}

///Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
pub fn style_tree<'a,>(root: &'a dom::Node, stylesheet: &'a css::Stylesheet,) -> StyledNode<'a,> {
   use dom::NodeType;

   let specified_values = match root.node_type {
      NodeType::Element(ref elem,) => specified_values(elem, stylesheet,),
      NodeType::Text(_,) => PropertyMap::new(),
   };
   StyledNode {
      node: root,
      specified_values,
      children: root.children.iter().map(|child| style_tree(child, stylesheet,),).collect(),
   }
}
