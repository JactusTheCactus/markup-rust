use {
	crate::Token,
	Token::*,
};
impl Token
{
	pub fn as_char(&self) -> Option<char>
	{
		match self
		{
			Text(c) => Some(*c),
			_ => None,
		}
	}

	pub fn as_num(&self) -> Option<u8>
	{
		match self
		{
			Number(c) => Some(*c),
			_ => None,
		}
	}
}
