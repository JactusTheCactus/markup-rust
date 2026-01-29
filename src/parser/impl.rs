use crate::{
	Token,
	Token::*,
	parser::r#struct::Parser,
};
impl Parser
{
	pub fn new(tokens : Vec<Token>) -> Self
	{
		Self {
			tokens,
			current : 0,
		}
	}

	pub fn parse(&mut self)
	{
		while !self.is_at_end()
		{
			self.statement();
		}
	}

	fn is_at_end(&self) -> bool { matches!(self.peek(), Eof) }

	fn peek(&self) -> &Token { &self.tokens[self.current] }

	fn advance(&mut self) -> &Token
	{
		if !self.is_at_end()
		{
			self.current += 1;
		}
		self.previous()
	}

	fn previous(&self) -> &Token { &self.tokens[self.current - 1] }

	fn statement(&mut self) { self.advance(); }

	pub fn eval(&mut self) -> String
	{
		let mut html = String::from("");
		struct Header
		{
			on : bool,
			level : u8,
		}
		struct Style
		{
			italic : bool,
			bold : bool,
			underline : bool,
			header : Header,
		}
		let mut style = Style {
			italic : false,
			bold : false,
			underline : false,
			header : Header {
				on : false,
				level : 0,
			},
		};
		for i in &self.tokens
		{
			match i
			{
				Slash =>
				{
					html +=
						&format!("<{}i>", if style.italic { "" } else { "/" });
					style.italic = !style.italic;
				},
				Star =>
				{
					html +=
						&format!("<{}b>", if style.bold { "" } else { "*" });
					style.bold = !style.bold;
				},
				Underscore =>
				{
					html += &format!(
						"<{}u>",
						if style.underline { "" } else { "_" }
					);
					style.underline = !style.underline;
				},
				Exclamation =>
				{
					if !style.header.on
					{
						style.header.on = true;
					}
				},
				Newline | Eof =>
				{
					if style.header.on
					{
						html += &format!("</h{}>", style.header.level);
						style.header.on = false;
					}
				},
				_ =>
				{
					if let Some(c) = i.as_char()
					{
						html.push(c);
					}
					else if let Some(c) = i.as_num()
					{
						if style.header.on && (1..=6).contains(&c)
						{
							style.header.level = c;
							html += &format!("<h{c}>")
						}
						else
						{
							html.push_str(&c.to_string())
						}
					}
					else
					{
						html.push('?')
					}
				},
			}
		}
		html
	}
}
