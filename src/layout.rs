#![allow(unused)]
use crate::{css, style};

///CSS box model. All sizes are in px.
#[derive(Default, Clone,)]
struct Dimensions {
   //Position of the content area  relative to the document origin:
   content: Rct,
   //Surrounding edges:
   padding: EdgeSizes,
   border:  EdgeSizes,
   margin:  EdgeSizes,
}

///Rectangular module
#[derive(Default, Clone,)]
struct Rct {
   x:      f64,
   y:      f64,
   width:  f64,
   height: f64,
}

///Positions of 4 corners
#[derive(Default, Clone,)]
struct EdgeSizes {
   left:   f64,
   right:  f64,
   top:    f64,
   bottom: f64,
}

///The layout tree is a collection of layoutboxes. It contains boxes as child
#[derive(Clone,)]
struct LayoutBox<'a,> {
   box_type:   BoxType<'a,>,
   dimensions: Dimensions,
   children:   Vec<LayoutBox<'a,>,>,
}

///A box can be a block node, an inline node, OR an anonymous block box
#[derive(Clone,)]
enum BoxType<'a,> {
   BlockNode(&'a style::StyledNode<'a,>,),
   InlineNode(&'a style::StyledNode<'a,>,),
   AnonymousBlock,
}

impl<'a,> LayoutBox<'a,> {
   ///Constructor
   fn new(box_type: BoxType,) -> LayoutBox { LayoutBox { box_type, dimensions: Default::default(), children: vec![], } }

   ///Where a new inline child should go.
   fn get_inline_container(self,) -> LayoutBox<'a,> {
      match self.box_type {
         BoxType::BlockNode(_,) => {
            //If we've just generated an anonymous block box, keep using it.
            //Otherwise, create a new one.
            match self.children.last() {
               Some(&LayoutBox { box_type: BoxType::AnonymousBlock, .. },) => self.children.last().unwrap().clone(),
               _ => {
                  let mut cl = self;
                  cl.children.push(LayoutBox::new(BoxType::AnonymousBlock,),);
                  cl.children.last().unwrap().clone()
               }
            }
         }
         _ => self,
      }
   }

   ///Layout a box and its descendants.
   fn layout(&mut self, cntin_blck: Dimensions,) {
      todo!(
         "------------------------------------------------------
       [ImplLater]
           InlineNode & AnonymousBlock
             ------------------------------------------------------"
      );
      match self.box_type {
         BoxType::BlockNode(_,) => self.layout_block(cntin_blck,),
         BoxType::InlineNode(_,) => {}
         BoxType::AnonymousBlock => {}
      }
   }

   ///Block's width depends on its parent, height depends on its children
   fn layout_block(&mut self, cntin_blck: Dimensions,) {
      //Calculate parent's width at first
      self.calc_width(cntin_blck,);
      self.calc_position(cntin_blck,);
      self.layout_children();
      //Calculate parent's height at last
      self.calc_height();
   }

   ///Calculate width of block
   fn calc_width(&mut self, cntin_blck: Dimensions,) {
      let style = self.get_style_node();
      //'width' has initial value 'auto'
      let auto = css::Value::Keyword("auto".to_string(),);
      let mut width = style.val("width",).unwrap_or(auto.clone(),);

      //margin, border, padding have init value 0.
      let zero = css::Value::Length(0.0, Px,);

      let mut margin_left = style.lookup("margin-left", "margin", &zero,);
      let mut margin_right = style.lookup("margin-right", "margin", &zero,);
      let border_left = style.lookup("border-left-width", "border-width", &zero,);
      let border_right = style.lookup("border-right-width", "border-width", &zero,);
      let padding_left = style.lookup("padding-left", "padding", &zero,);
      let padding_right = style.lookup("padding-right", "padding", &zero,);

      let total = [&margin_left, &margin_right, &border_left, &border_right, &padding_left, &padding_right, &width,]
         .iter()
         .map(|v| v.to_px(),)
         .sum();
      //if width!=auto & total is wider than container, treat auto margins as 0.
      if width != auto && total > cntin_blck.content.width {
         if margin_left == auto {
            margin_left = css::Value::Length(0.0, Px,);
         }
         if margin_right == auto {
            margin_right = css::Value::Length(0.0, Px,);
         }
      }
      //if 'flow' is +, it's underflow. 'flow' is -, it's overflow.
      let flow = cntin_blck.content.width - total;
   }
}

///Build the tree of LayoutBoxes, but don't perform any layout calculations
/// yet.
fn build_layout_tree<'a,>(style_node: &'a style::StyledNode<'a,>,) -> LayoutBox<'a,> {
   use {style::Display::*, BoxType::*};
   //Create the root box.
   let mut root = LayoutBox::new(match style_node.display() {
      Block => BlockNode(style_node,),
      Inline => InlineNode(style_node,),
      Non => panic!("Root node has display: none."),
   },);
   //Create the descendant boxes.
   for child in &style_node.children {
      root = match child.display() {
         Block | Inline => {
            let mut cl = root.clone();
            let mut ret = cl.get_inline_container();
            ret.children.push(build_layout_tree(&child,),);
            ret
         }
         _ => root,
      };
   }
   root
}
