mod parser;
mod token;
use {
	parser::r#struct::Parser,
	token::{
		r#enum::Token,
		tokenise::tokenise,
	},
};
fn main()
{
	let mut parser = Parser::new(tokenise("!1Header\nHello, World!"));
	parser.parse();
	println!("{}", parser.eval());
}
