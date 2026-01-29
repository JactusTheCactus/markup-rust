use {
	crate::Token,
	Token::*,
};
pub fn tokenise(input : &str) -> Vec<Token>
{
	let mut tokens = Vec::new();
	let mut chars = input.chars().peekable();
	while let Some(&c) = chars.peek()
	{
		match c
		{
			'0'..='9' =>
			{
				let mut num = 0;
				while let Some(d @ '0'..='9') = chars.peek()
				{
					num = num * 10 + (d.to_digit(10).unwrap() as u8);
					chars.next();
				}
				tokens.push(Number(num));
			},
			'\\' =>
			{
				tokens.push(Backslash);
				chars.next();
			},
			'\n' =>
			{
				tokens.push(Newline);
				chars.next();
			},
			'*' =>
			{
				tokens.push(Star);
				chars.next();
			},
			'/' =>
			{
				tokens.push(Slash);
				chars.next();
			},
			'_' =>
			{
				tokens.push(Underscore);
				chars.next();
			},
			'!' =>
			{
				tokens.push(Exclamation);
				chars.next();
			},
			'-' =>
			{
				tokens.push(Hyphen);
				chars.next();
			},
			'{' =>
			{
				tokens.push(LBrace);
				chars.next();
			},
			'}' =>
			{
				tokens.push(RBrace);
				chars.next();
			},
			'#' =>
			{
				tokens.push(Hash);
				chars.next();
			},
			'.' =>
			{
				tokens.push(Dot);
				chars.next();
			},
			':' =>
			{
				tokens.push(Colon);
				chars.next();
			},
			';' =>
			{
				tokens.push(Semicolon);
				chars.next();
			},
			'<' =>
			{
				tokens.push(LCaret);
				chars.next();
			},
			'>' =>
			{
				tokens.push(RCaret);
				chars.next();
			},
			'|' =>
			{
				tokens.push(Pipe);
				chars.next();
			},
			_ =>
			{
				tokens.push(Text(c));
				chars.next();
			},
		}
	}
	tokens.push(Eof);
	tokens
}
