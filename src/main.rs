#[derive(Debug, Clone, PartialEq)]
enum Token {
	Number(u8),
	Slash,
	Backslash,
	Newline,
	Star,
	Underscore,
	Exclamation,
	Hyphen,
	LBrace,
	RBrace,
	Pound,
	Dot,
	Pipe,
	Colon,
	Semicolon,
	LCaret,
	RCaret,
	Text(char),
	Eof,
}
impl Token {
	fn as_char(&self) -> Option<char> {
		match self {
			Token::Text(c) => Some(*c),
			_ => None,
		}
	}
	fn as_num(&self) -> Option<u8> {
		match self {
			Token::Number(c) => Some(*c),
			_ => None,
		}
	}
}
fn tokenize(input: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut chars = input.chars().peekable();
	while let Some(&c) = chars.peek() {
		match c {
			'0'..='9' => {
				let mut num = 0;
				while let Some(d @ '0'..='9') = chars.peek() {
					num = num * 10 + (d.to_digit(10).unwrap() as u8);
					chars.next();
				}
				tokens.push(Token::Number(num));
			}
			'\\' => {
				tokens.push(Token::Backslash);
				chars.next();
			}
			'\n' => {
				tokens.push(Token::Newline);
				chars.next();
			}
			'*' => {
				tokens.push(Token::Star);
				chars.next();
			}
			'/' => {
				tokens.push(Token::Slash);
				chars.next();
			}
			'_' => {
				tokens.push(Token::Underscore);
				chars.next();
			}
			'!' => {
				tokens.push(Token::Exclamation);
				chars.next();
			}
			'-' => {
				tokens.push(Token::Hyphen);
				chars.next();
			}
			'{' => {
				tokens.push(Token::LBrace);
				chars.next();
			}
			'}' => {
				tokens.push(Token::RBrace);
				chars.next();
			}
			'#' => {
				tokens.push(Token::Pound);
				chars.next();
			}
			'.' => {
				tokens.push(Token::Dot);
				chars.next();
			}
			':' => {
				tokens.push(Token::Colon);
				chars.next();
			}
			';' => {
				tokens.push(Token::Semicolon);
				chars.next();
			}
			'<' => {
				tokens.push(Token::LCaret);
				chars.next();
			}
			'>' => {
				tokens.push(Token::RCaret);
				chars.next();
			}
			'|' => {
				tokens.push(Token::Pipe);
				chars.next();
			}
			_ => {
				tokens.push(Token::Text(c));
				chars.next();
			}
		}
	}
	tokens.push(Token::Eof);
	tokens
}
struct Parser {
	tokens: Vec<Token>,
	current: usize,
}
impl Parser {
	fn new(tokens: Vec<Token>) -> Self {
		Self { tokens, current: 0 }
	}
	fn parse(&mut self) {
		while !self.is_at_end() {
			self.statement();
		}
	}
	fn is_at_end(&self) -> bool {
		matches!(self.peek(), Token::Eof)
	}
	fn peek(&self) -> &Token {
		&self.tokens[self.current]
	}
	fn advance(&mut self) -> &Token {
		if !self.is_at_end() {
			self.current += 1;
		}
		self.previous()
	}
	fn previous(&self) -> &Token {
		&self.tokens[self.current - 1]
	}
	fn statement(&mut self) {
		self.advance();
	}
	fn eval(&mut self) -> String {
		let mut html = "".to_string();
		let mut italic = false;
		let mut bold = false;
		let mut underline = false;
		let mut header = false;
		let mut header_level: u8 = 0;
		for i in &self.tokens {
			match i {
				Token::Slash => {
					html += &format!("<{}i>", if italic { "" } else { "/" });
					italic = !italic;
				}
				Token::Star => {
					html += &format!("<{}b>", if bold { "" } else { "*" });
					bold = !bold;
				}
				Token::Underscore => {
					html += &format!("<{}u>", if underline { "" } else { "_" });
					underline = !underline;
				}
				Token::Exclamation => {
					if !header {
						header = true;
					}
				}
				Token::Newline | Token::Eof => {
					if header {
						html += &format!("</h{header_level}>");
						header = false;
					}
				}
				_ => {
					if let Some(c) = i.as_char() {
						html.push(c);
					} else if let Some(c) = i.as_num() {
						if header && (1..=6).contains(&c) {
							header_level = c;
							html += &format!("<h{c}>")
						} else {
							html.push_str(&c.to_string())
						}
					} else {
						html.push('?')
					}
				}
			}
		}
		html
	}
}
fn main() {
	let mut parser = Parser::new(tokenize("!6Header\nHello, World!"));
	parser.parse();
	println!("{}", parser.eval());
}
