use incodoc::*;

pub fn doc_to_html_css_string(doc: &Doc) -> String {
    let mut res = String::new();
    doc_to_html_css(doc, &mut res);
    res
}

pub fn doc_to_html_css(doc: &Doc, output: &mut String) {
    *output += "<html>\n";
    *output += "<body>\n";
    for item in &doc.items {
        match item {
            DocItem::Nav(nav) => nav_to_html_css(nav, output),
            DocItem::Paragraph(par) => paragraph_to_html_css(par, output),
            DocItem::Section(section) => section_to_html_css(section, output),
        }
    }
    *output += "</body>\n";
    *output += "</html>\n";
}

pub fn nav_to_html_css(nav: &Nav, output: &mut String) {

}

pub fn paragraph_to_html_css(par: &Paragraph, output: &mut String) {
    *output += "<p>\n";
    for item in &par.items {
        match item {
            ParagraphItem::Text(text) => *output += text,
            // ParagraphItem::MText(TextWithMeta { text, tags, .. }) => {
            //     if tags.contains("code") {
            //         inline_code_to_ansi(text, conf, c, output);
            //     } else {
            //         text_to_ansi(text, conf, c, output);
            //     }
            // },
            ParagraphItem::Em(emphasis) => emphasis_to_html_css(emphasis, output),
            // ParagraphItem::Link(link) => {
            //     link_to_ansi(link, &conf.link, conf, c, output);
            // },
            // ParagraphItem::Code(code) => {
            //     code_to_ansi(code, conf, c, output);
            // },
            // ParagraphItem::List(list) => {
            //     list_to_ansi(list, conf, c, output);
            // },
            // ParagraphItem::Table(table) => {
            //     table_to_ansi(table, conf, c, output);
            // },
            _ => {},
        }
    }
    *output += "\n</p>\n";
}

pub fn section_to_html_css(section: &Section, output: &mut String) {
    *output += "<section>\n";
    let level = match section.heading.level {
        0 => "1",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        _ => "6",
    };
    *output += "<h";
    *output += level;
    *output += ">\n";
    for item in &section.heading.items {
        match item {
            HeadingItem::String(string) => *output += string,
            HeadingItem::Em(emphasis) => emphasis_to_html_css(emphasis, output),
        }
    }
    *output += "\n</h";
    *output += level;
    *output += ">\n";
    for item in &section.items {
        match item {
            SectionItem::Paragraph(par) => paragraph_to_html_css(par, output),
            SectionItem::Section(section) => section_to_html_css(section, output),
        }
    }
    *output += "</section>\n";
}

pub fn emphasis_to_html_css(em: &Emphasis, output: &mut String) {
    let (start, end) = match (em.etype, em.strength) {
        (EmType::Emphasis, EmStrength::Light) => ("<em>", "</em>"),
        (EmType::Emphasis, EmStrength::Medium) => ("<strong>", "</strong>"),
        (EmType::Emphasis, EmStrength::Strong) => ("<mark>", "</mark>"),
        (EmType::Deemphasis, EmStrength::Light) => ("<span class=\"light-em\">", "</span>"),
        (EmType::Deemphasis, EmStrength::Medium) => ("<span class=\"medium-em\">", "</span>"),
        (EmType::Deemphasis, EmStrength::Strong) => ("<span class=\"strong-em\">", "</span>"),
    };
    *output += start;
    *output += &em.text;
    *output += end;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
