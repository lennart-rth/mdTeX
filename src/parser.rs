use crate::USEPACKAGES;
use crate::lexer::Settings;
use crate::lexer::Token;

pub fn parse(lexed_markdown: (Vec<Token>,Settings)) -> Vec<String>{
    let mut latex:Vec<String> = Vec::new();
    let tokens = lexed_markdown.0;
    let settings = lexed_markdown.1;

    let mut in_heading_section = false;
    let mut in_list_section = false;
    let mut current_list_level = 0;
    let mut in_numberedlist_section = false;
    let mut current_numberedlist_level = 0;
    let mut image_lex_index = 0;
    let mut in_newslide_section = false;
    let mut slide_begun = false;
    let mut in_collumn_section = false;

    latex.push("\\documentclass{beamer}\n\n".to_string());
    latex.push(format!("\\usetheme{{{}}}\n", settings.theme.expect("REASON").to_string()));
    latex.push(format!("\\title{{{}}}\n", settings.title.expect("REASON").to_string()));
    latex.push(format!("\\author{{{}}}\n", settings.author.expect("REASON").to_string()));
    latex.push(format!("\\date{{{}}}\n\n", settings.date.expect("REASON").to_string()));
    latex.push("\\begin{document}\n".to_string());
    latex.push("\\begin{frame}\n".to_string());
    latex.push("\\titlepage\n".to_string());
    latex.push("\\end{frame}\n".to_string());

    for (i,token) in tokens.iter().enumerate() {
        // println!("{}, {:?}",i,token);
        match token {
            Token::Newline => {
                if in_heading_section {
                    latex.push("\n".to_string());
                    latex.push("\\normalsize".to_string());
                    latex.push("\n".to_string());
                    in_heading_section = false;
                } else if in_newslide_section {
                    latex.push("}\n".to_string());
                    in_newslide_section = false;
                } else if in_list_section {
                    if i+1 <= tokens.len() {
                        match tokens[i+1]  {
                            Token::List(level) => {
                                if level < (current_list_level-1) {
                                    // latex.push("\n".to_string());
                                    for _ in level..current_list_level-1 {
                                        latex.push("\\end{itemize}".to_string());
                                        latex.push("\n".to_string());
                                    }
                                    latex.pop();        //remove the newline thats been placed falsely
                                    current_list_level = level+1;
                                } else {
                                    // latex.push("\n".to_string());
                                }
                            }
                            _ => {
                                latex.push("\n".to_string());
                                for _ in 0..current_list_level {
                                    latex.push("\\end{itemize}\n".to_string());
                                }
                                current_list_level = 0;
                                in_list_section = false;
                            }
                        }
                    } else {
                        latex.push("\n".to_string());
                        latex.push("\\end{itemize}\n".to_string());
                        in_list_section = false;
                    }
                } else if in_numberedlist_section {
                    if i+1 <= tokens.len() {
                        match tokens[i+1]  {
                            Token::NumberedList(level) => {
                                if level < (current_numberedlist_level-1) {
                                    latex.push("\n".to_string());
                                    for _ in level..current_numberedlist_level-1 {
                                        latex.push("\\end{enumerate}\n".to_string());
                                    }
                                    current_numberedlist_level = level+1;
                                } else {
                                    latex.push("\n".to_string());
                                }
                            }
                            _ => {
                                latex.push("\n".to_string());
                                for _ in 0..current_numberedlist_level {
                                    latex.push("\\end{enumerate}\n".to_string());
                                }
                                current_numberedlist_level = 0;
                                in_numberedlist_section = false;
                            }
                        }
                    } else {
                        latex.push("\n".to_string());
                        latex.push("\\end{itemize}\n".to_string());
                        in_list_section = false;
                    }
                } else {
                    latex.push("\n".to_string());
                }
            }
            Token::NewSlide => {
                if slide_begun {
                    latex.push("\\end{frame}\n".to_string());
                }
                latex.push("\\begin{frame}{".to_string());
                in_newslide_section = true;
                slide_begun = true;
            }
            Token::Linebreak => {
                latex.push("\\\\\n".to_string());
            }
            Token::Text(text) => {
                latex.push(text.to_string());
            }
            Token::Bold(text) => {
                latex.push(format!("\\textbf{{{}}}", text));
            }
            Token::Italic(text) => {
                latex.push(format!("\\textit{{{}}}", text));
            }
            Token::Underlined(text) => {
                latex.push(format!("\\underline{{{}}}", text));
            }
            Token::Crossed(text) => {
                latex.push(format!("\\cancel{{{}}}", text));
                unsafe{
                    USEPACKAGES.cancle = true
                };
            }
            Token::Comment(text) => {
                latex.push(format!("% {{{}}}\n", text));
            }
            Token::HeadingLarge => {
                latex.push("\n\\Huge\n".to_string());
                in_heading_section = true;
            }
            Token::HeadingMedium => {
                latex.push("\n\\Large\n".to_string());
                in_heading_section = true;
            }
            Token::HeadingSmall => {
                latex.push("\n\\footnotesize\n".to_string());
                in_heading_section = true;
            }
            Token::List(level) => {
                if level >= &(current_list_level) {
                    latex.push("\\begin{itemize}\n".to_string());
                    current_list_level = current_list_level + 1;

                } else if level < &(current_list_level-1) {
                    latex.push("\\end{itemize}\n".to_string());
                    current_list_level = current_list_level - 1;
                }
                latex.push("\\item ".to_string());

                in_list_section = true;
            }
            Token::NumberedList(level) => {
                if level >= &(current_numberedlist_level) {
                    latex.push("\\begin{enumerate}\n".to_string());
                    current_numberedlist_level = current_numberedlist_level + 1;

                } else if level < &(current_numberedlist_level-1) {
                    latex.push("\\end{enumerate}\n".to_string());
                    current_numberedlist_level = current_numberedlist_level - 1;
                }
                latex.push("\\item ".to_string());

                in_numberedlist_section = true;
            }
            Token::BlockStart => {
                latex.push("\\begin{block}{".to_string());
            }
            Token::BlockEndTitle => {
                latex.push("}\n".to_string());
            }
            Token::BlockEnd => {
                latex.push("\n".to_string());
                latex.push("\\end{block}\n".to_string());
            }
            Token::Image(x,y,w,path) => {
                latex.push("\\begin{figure}\n".to_string());
                latex.push(format!("\\hspace*{{{x}\\textwidth}}\n"));
                latex.push(format!("\\vspace*{{{y}\\textwidth}}\n"));
                latex.push(format!("\\includegraphics[width={w}\\linewidth]{{{path}}}\n"));
                latex.push("\\caption{".to_string());
                image_lex_index = i;
            }
            Token::ImageCaptionEnd => {
                if image_lex_index+1 == i { //if the image caption follows directly after the image we do not have a caption
                    latex.pop();       // remove falsely added image caption start.
                    latex.push("\\end{figure}\n".to_string());
                } else {
                    latex.push("}\n".to_string());
                    latex.push("\\end{figure}\n".to_string());
                }
                
            }
            Token::Column => {
                if in_collumn_section {
                    latex.push("\\end{columns}".to_string());
                } else {
                    latex.push("\\begin{columns}".to_string());
                }
                in_collumn_section = !in_collumn_section;
            }
            Token::ColumnEle(p) => {
                latex.push(format!("\\column{{{}\\textwidth}}",*p as f32/100.0).to_string());
            }
        }
    }

    latex.push("\\end{frame}\n".to_string());

    println!("Successfully parsed to Latex Beamer.");

    latex
}