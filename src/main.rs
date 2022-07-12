mod css;
mod dom;
mod html;
mod layout;
mod painting;
mod style;

struct Parser {
   pos: usize,
   inp: String,
}

impl Parser {
   ///Read the current character without consuming it.
   pub fn next_char(&self,) -> char { self.inp[self.pos..].chars().next().unwrap() }

   ///Do the next characters start with the given string?
   pub fn starts_with(&self, s: &str,) -> bool { self.inp[self.pos..].starts_with(s,) }

   ///Return true if all input is consumed.
   pub fn eof(&self,) -> bool { self.pos >= self.inp.len() }

   ///Return the current character, and advance self.pos to the next character.
   pub fn cnsm_chr(&mut self,) -> char {
      let mut itr = self.inp[self.pos..].char_indices();
      let (_, cur_chr,) = itr.next().unwrap();
      let (next_pos, _,) = itr.next().unwrap_or((1, ' ',),);
      self.pos += next_pos;
      cur_chr
   }

   ///Consume characters until 'test' returns false.
   pub fn cnsm_while(&mut self, test: impl Fn(char,) -> bool,) -> String {
      let mut rslt = String::new();
      while !self.eof() && test(self.next_char(),) {
         rslt.push(self.cnsm_chr(),);
      }
      rslt
   }

   ///Consume and discard zero or more whitespace characters.
   pub fn cnsm_whitespace(&mut self,) { self.cnsm_while(char::is_whitespace,); }
}

fn main() {}
