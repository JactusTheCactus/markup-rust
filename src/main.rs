mod parser;
mod token;
use {
	logos::Logos,
	parser::r#struct::Parser,
	regex::Regex,
	token::{
		r#enum::Token,
		tokenise::tokenise,
	},
};
fn main()
{
	let test_string = "!1 Header\nHello, World!";
	let testing = true;
	if !testing
	{
		let mut parser = Parser::new(tokenise(test_string));
		parser.parse();
		println!("{}", parser.eval());
	}
	else
	{
		#[derive(Logos, Debug, PartialEq, Clone)]
		enum Token0
		{
			#[regex(r"\d")]
			Number,
			#[token("/")]
			Slash,
			#[token("\\")]
			BackSlash,
			#[token("\n")]
			Newline,
			#[token("*")]
			Star,
			#[token("_")]
			Underscore,
			#[regex(r"![1-6]")]
			Header,
			#[token("-")]
			Hyphen,
			#[token("{")]
			LeftBrace,
			#[token("}")]
			RightBrace,
			#[token("#")]
			Hash,
			#[token(".")]
			Dot,
			#[token("|")]
			Pipe,
			#[token(":")]
			Colon,
			#[token(";")]
			Semicolon,
			#[token("<")]
			LeftCaret,
			#[token(">")]
			RightCaret,
			#[regex(r"/(?:upper|lower|cap)")]
			Command,
			#[regex(r"[a-zA-Z]+?")]
			Word,
			#[token("!")]
			Exclamation,
			#[token(",")]
			Comma,
			#[token("\t")]
			Tab,
			#[token(" ")]
			Space,
			#[token("'")]
			SingleQuote,
			#[token("\"")]
			DoubleQuote
		}
		let mut lexer = Token0::lexer(test_string);
		let re_token = Regex::new(r"Ok\((.*?)\)").unwrap();
		let re_slice = Regex::new(r"(.*?)").unwrap();
		let re_whitespace = Regex::new(r"Space|Tab|Newline").unwrap();
		while let Some(token) = lexer.next()
		{
			let token_string = format!("{:?}", token);
			let slice_string = lexer.slice().to_string();
			let t = re_token.replace(&token_string, "$1");
			let s = re_slice.replace(&slice_string, "$1");
			let t_ = token.clone().ok().expect("Expected a Token0 :(");
			use Token0::*;
			match t_
			{
				Newline =>
				{
					print!("\n");
				},
				Space =>
				{
					print!(" ");
				},
				Word | Symbol | Number =>
				{
					if !re_whitespace.is_match(&t)
					{
						print!("{s}")
					}
				},
				Header =>
				{},
				_ =>
				{
					print!("<{:?}>", t_);
				},
			}
		}
	}
}
