use regex::Regex;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    HeadingLarge,
    HeadingMedium,
    HeadingSmall,
    Comment(String),
    BlockStart,
    BlockEndTitle,
    BlockEnd,
    List(u32),
    NumberedList(u32),
    NewSlide,
    Column,
    ColumnEle(u32),
    Image(f64, f64, f64, String),
    ImageCaptionEnd,
    Newline,
    Linebreak,
    
    Text(String),
    Underlined(String),
    Bold(String),
    Italic(String),
    Crossed(String),
}

#[derive(Debug)]
pub struct Settings {
    pub theme: Option<String>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub date: Option<String>,
    pub indentation: u32,
}

fn settings_init_failed(settings: &Settings) -> bool {
    settings.theme.is_none() || settings.author.is_none() || settings.title.is_none() || settings.date.is_none()
}

fn parse_settings(settings_content: Vec<String>) -> Settings {
    let mut theme = None;
    let mut author = None;
    let mut title = None;
    let mut date = None;
    let mut indentation = 0;


    for line in &settings_content {
        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        if let Some(key) = parts.get(0) {
            if let Some(value) = parts.get(1) {
                match *key {
                    "theme" => theme = Some(value.to_string()),
                    "author" => author = Some(value.to_string()),
                    "title" => title = Some(value.to_string()),
                    "date" => date = Some(value.to_string()),
                    "indentation" => indentation = value.parse().unwrap_or(0),
                    _ => {}
                }
            }
        }
    }

    let settings = Settings {
        theme: theme,
        author: author,
        title: title,
        date: date,
        indentation: indentation,
    };
    settings
}


fn lex_formatting(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    
    let mut bold = false;
    let mut italic = false;
    let mut underlined = false;
    let mut crossed = false;

    let mut buffer:Vec<String> = Vec::new();

    let input_len = input.len();

    let mut string_iter = input.split("");
    let mut i = 0;
    let mut c: String;

    while i <= input_len {
        c = string_iter.next().unwrap_or_default().to_string();
        if c == "\\" {
            c = string_iter.next().unwrap_or_default().to_string();
            buffer.push(c.clone());
        } else if c == "*" {
            c = string_iter.next().unwrap_or_default().to_string();
            if c == "*" {
                if !italic && !underlined && !crossed {
                    if bold {
                        tokens.push(Token::Bold(buffer.join("").to_string()));
                    } else{
                        tokens.push(Token::Text(buffer.join("").to_string()));
                    }
                    buffer = Vec::new();
                    bold = !bold;
                    c = string_iter.next().unwrap_or_default().to_string();
                }
            } else {
                if !bold && !underlined && !crossed {
                    if italic {
                        tokens.push(Token::Italic(buffer.join("").to_string()));
                    } else{
                        tokens.push(Token::Text(buffer.join("").to_string()));
                    }
                    buffer = Vec::new();
                    italic = !italic;
                }
            }
        } else if c == "_" {
            if !italic && !bold && !crossed {
                if underlined {
                    tokens.push(Token::Underlined(buffer.join("").to_string()));
                } else{
                    tokens.push(Token::Text(buffer.join("").to_string()));
                }
                buffer = Vec::new();
                underlined = !underlined;
                c = string_iter.next().unwrap_or_default().to_string();
            }

        } else if c == "-" {
            if !italic && !bold && !underlined {
                if crossed {
                    tokens.push(Token::Crossed(buffer.join("").to_string()));
                } else{
                    tokens.push(Token::Text(buffer.join("").to_string()));
                }
                buffer = Vec::new();
                crossed = !crossed;
                c = string_iter.next().unwrap_or_default().to_string();
            }
        }

        // if c != "*" && c != "-" && c != "_" {
        buffer.push(c);
        
        i = i+1;
    }

    tokens.push(Token::Text(buffer.join("").to_string()));

    tokens

}

pub fn lex(input: &str) -> (Vec<Token>, Settings) {
    let mut tokens = Vec::new();
    let mut delimiter_cnt = 0;

    let heading_large_regex = Regex::new(r"^### (.*)").unwrap();
    let heading_medium_regex = Regex::new(r"^## (.*)").unwrap();
    let heading_small_regex = Regex::new(r"^# (.*)").unwrap();

    let comment_regex = Regex::new(r"^//(.*)").unwrap();
    let newslide_regex = Regex::new(r"^---(?: (.*))?").unwrap();

    let block_regex = Regex::new(r"^(?:\[(.*)\])?\[\[(.*)\]\]").unwrap();
    let list_regex = Regex::new(r"^( *)?\* (.*)").unwrap();
    let numbered_list_regex = Regex::new(r"^( *)?\d\. (.*)").unwrap();

    let column_regex = Regex::new(r"^::").unwrap();
    let column_element_regex = Regex::new(r"^:(\d{1,3})%").unwrap();

    let image_regex = Regex::new(r"^!\[x:(-?\d+(?:\.?\d+)?),y:(-?\d+(?:\.?\d+)?),w:(-?\d+(?:\.?\d+)?)\](?:\[(.*)\])?\((.*)\)").unwrap();
    let text_regex = Regex::new(r"^(.*)").unwrap();
    let linebreak_regex = Regex::new(r".*\\").unwrap();



    let mut line_iter = input.split("\n");
    let mut settings_section: Vec<String> = Vec::new();

    while delimiter_cnt < 2 {
        let line = line_iter.next().unwrap_or_default();
        if line.trim() == "%%" {
            delimiter_cnt = delimiter_cnt + 1;
        } else {
            settings_section.push(line.trim().to_string());
        }
    }

    // parse settings
    let settings = parse_settings(settings_section);
    // println!("{:?}", settings);

    if settings_init_failed(&settings) {
        eprintln!("Error while parsing Settings!");
    }


    for line in line_iter {
                
        if let Some(captures) = heading_large_regex.captures(line.trim()) {
            tokens.push(Token::HeadingLarge);
            let format_tokens = lex_formatting(captures[1].trim().to_string());
            for t in format_tokens {
                tokens.push(t);
            }
            tokens.push(Token::Newline);
        } else if let Some(captures) = heading_medium_regex.captures(line.trim()) {
            tokens.push(Token::HeadingMedium);
            let format_tokens = lex_formatting(captures[1].trim().to_string());
            for t in format_tokens {
                tokens.push(t);
            }
            tokens.push(Token::Newline);
        } else if let Some(captures) = heading_small_regex.captures(line.trim()) {
            tokens.push(Token::HeadingSmall);
            let format_tokens = lex_formatting(captures[1].trim().to_string());
            for t in format_tokens {
                tokens.push(t);
            }
            tokens.push(Token::Newline);
        } else if let Some(captures) = comment_regex.captures(line.trim()) {
            tokens.push(Token::Comment(captures[1].trim().to_string()));
        } else if let Some(captures) = block_regex.captures(line.trim()) {
            tokens.push(Token::BlockStart);
            if let Some(title_group) = captures.get(1) {
                let format_tokens = lex_formatting(title_group.as_str().trim().to_string());
                for t in format_tokens {
                    tokens.push(t);
                }
            }
            tokens.push(Token::BlockEndTitle);
            let format_tokens = lex_formatting(captures[2].trim().to_string());
            for t in format_tokens {
                tokens.push(t);
            }
            tokens.push(Token::BlockEnd);
        } else if let Some(captures) = list_regex.captures(line) {
            tokens.push(Token::List((u32::try_from(captures[1].to_string().len()).unwrap()/settings.indentation).try_into().unwrap()));
            let format_tokens = lex_formatting(captures[2].trim().to_string());
            for t in format_tokens {
                tokens.push(t);
            }
        } else if let Some(captures) = numbered_list_regex.captures(line) {
            tokens.push(Token::NumberedList((u32::try_from(captures[1].to_string().len()).unwrap()/settings.indentation).try_into().unwrap()));
            let format_tokens = lex_formatting(captures[2].trim().to_string());
            for t in format_tokens {
                tokens.push(t);
            }
        } else if let Some(captures) = newslide_regex.captures(line.trim()) {
            tokens.push(Token::NewSlide);
            if let Some(slide_title) = captures.get(1) {
                let format_tokens = lex_formatting(slide_title.as_str().trim().to_string());
                for t in format_tokens {
                    tokens.push(t);
                }
            }
        } else if column_regex.captures(line.trim()).is_some() {
            tokens.push(Token::Column);
        } else if let Some(captures) = column_element_regex.captures(line.trim()) {
            tokens.push(Token::ColumnEle(captures[1].parse::<u32>().unwrap()));
        } else if let Some(captures) = image_regex.captures(line.trim()) {
            let x = captures[1].parse().unwrap();
            let y = captures[2].parse().unwrap();
            let w = captures[3].parse().unwrap();
            let image_path = captures[5].trim().to_string();
            tokens.push(Token::Image(x, y, w, image_path));
            if let Some(caption) = captures.get(4) {
                let format_tokens = lex_formatting(caption.as_str().trim().to_string());
                for t in format_tokens {
                    tokens.push(t);
                }
            }
            tokens.push(Token::ImageCaptionEnd);
                    
        } else if let Some(captures) = text_regex.captures(line.trim()) {
                if let Some(captures) = linebreak_regex.captures(line.trim()) {
                    let format_tokens = lex_formatting(captures[1].trim().to_string());
                    for t in format_tokens {
                        tokens.push(t);
                    }
                    tokens.push(Token::Linebreak);
                } else {
                    let format_tokens = lex_formatting(captures[1].trim().to_string());
                    for t in format_tokens {
                        tokens.push(t);
                    }
                }
        }

        tokens.push(Token::Newline);
    }

    (tokens, settings)
}