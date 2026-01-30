use ::{ logos::Logos, regex::Regex, std::{ env, u8 } };
fn html_esc(input: String) -> String {
	let mut html: String = "".to_string();
	for i in input.chars() {
		match i {
			'&' => html.push_str("&amp;"),
			'"' => html.push_str("&quot;"),
			'<' => html.push_str("&lt;"),
			'>' => html.push_str("&gt;"),
			_ => html.push(i),
		}
	}
	html
}
fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	let test_string = args.first().expect("Expected a String");
	#[derive(Debug, PartialEq, Clone)]
	struct Code {
		lang: String,
		text: String,
		esc: bool,
	}
	#[derive(Debug, PartialEq, Clone)]
	struct Header {
		lvl: u8,
		text: String,
	}
	#[derive(Logos, Debug, PartialEq, Clone)]
	enum Token {
		#[regex(r"<-!?[a-z]+>\{[\s\S]*?\}", |lex| {
			let Some(caps) = Regex::new(r"<-(?<esc>!?)(?<lang>[a-z]+)>\{(?<text>[\s\S]*?)\}")
				.unwrap()
				.captures(lex.slice()) else {
				panic!("no match!")
			};
			Code {
				lang: caps["lang"].to_string(),
				text: caps["text"].to_string(),
				esc: caps["esc"].to_string() == "!",
			}
		})] Code(Code),
		#[regex(r"![1-6]<.+?>", |lex| {
			let Some(caps) = Regex::new(r"!(?<lvl>[1-6])<(?<text>.*?)>")
				.unwrap()
				.captures(lex.slice()) else {
				panic!("no match!")
			};
			Header {
				lvl: caps["lvl"].parse::<u8>().expect("Expected an 8-biy Unsigned Integer"),
				text: caps["text"].to_string(),
			}
		})] Header(Header),
		#[regex(r"/(?:upper|lower|cap);", |lex| {
			let Some(caps) = Regex::new(r"/(?<cmd>upper|lower|cap);")
				.unwrap()
				.captures(lex.slice()) else {
				panic!("no match!")
			};
			caps["cmd"].to_string()
		})] Command(String),
		#[token("\t")] Tab,
		#[regex("[a-zA-Z\\d,'\"# ]+", |lex| lex.slice().to_string())] Text(String),
		#[token("\n")] Newline,
	}
	use Token::*;
	let tokens = Token::lexer(test_string);
	if true {
		print!("<pre>");
		for token in tokens.clone() {
			println!("{:?}", token.expect("Expected a Token"));
		}
		print!("</pre><hr>");
	} else {
		for token in tokens {
			let tok = token.expect("Expected a Token");
			match tok {
				// Command(_) => (),
				Code(c) => {
					print!(
						"<pre><code data-lang=\"{}\"{}>{}</code></pre>",
						c.lang,
						if c.esc {
							" escape"
						} else {
							""
						},
						if c.esc {
							html_esc(c.text)
						} else {
							c.text
						}
					);
				}
				Header(h) => print!("<h{}>{}</h{}>", h.lvl, h.text, h.lvl),
				Text(t) => print!("{}", html_esc(t)),
				Newline => println!(),
				_ =>
					print!(
						"<li class=err>Unknown Token: <code>{}</code></li>",
						html_esc(format!("{:?}", tok))
					),
			}
		}
	}
}
