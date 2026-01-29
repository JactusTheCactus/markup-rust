use crate::Token;
pub struct Parser
{
	pub tokens : Vec<Token>,
	pub current : usize,
}
