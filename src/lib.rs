use incodoc::*;

pub fn doc_to_html_string(doc: &Doc) -> String {
    let mut res = String::new();
    doc_to_html(doc, &mut res);
    res
}

pub fn doc_to_html(doc: &Doc, output: &mut String) {
    *output += "<html>\n";
    *output += "<body>\n";
    for item in &doc.items {
        match item {
            DocItem::Nav(nav) => nav_to_html(nav, output),
            DocItem::Paragraph(par) => paragraph_to_html(par, output),
            DocItem::Section(section) => section_to_html(section, output),
        }
    }
    *output += "</body>\n";
    *output += "</html>\n";
}

pub fn nav_to_html(nav: &Nav, output: &mut String) {

}

pub fn paragraph_to_html(par: &Paragraph, output: &mut String) {
    *output += "<p>\n";
    for item in &par.items {
        match item {
            ParagraphItem::Text(text) => *output += text,
            ParagraphItem::MText(TextWithMeta { text, tags, .. }) => {
                // inline code is not handled differently
                // it will show up as a class
                // and the css can handle it
                *output += "<span class=\"";
                for tag in tags {
                    *output += tag;
                    *output += " ";
                }
                *output += "\">";
                *output += text;
                *output += "</span>";
            },
            ParagraphItem::Em(emphasis) => emphasis_to_html(emphasis, output),
            ParagraphItem::Link(link) => {
                *output += "<a ";
                *output += "href=\"";
                *output += &link.url;
                *output += "\" target=\"";
                *output += "_blank";
                if !link.tags.is_empty() {
                    *output += "\" class=\"";
                    for tag in &link.tags {
                        *output += tag;
                        *output += " ";
                    }
                }
                *output += "\">";
                for item in &link.items {
                    match item {
                        LinkItem::String(text) => *output += text,
                        LinkItem::Em(em) => emphasis_to_html(em, output),
                    }
                }
                *output += "</a>";
            },
            ParagraphItem::Code(code) => {
                code_to_html(code, output);
            },
            ParagraphItem::List(list) => {
                list_to_html(list, output);
            },
            ParagraphItem::Table(table) => {
                table_to_html(table, output);
            },
        }
    }
    *output += "\n</p>\n";
}

pub fn section_to_html(section: &Section, output: &mut String) {
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
            HeadingItem::Em(emphasis) => emphasis_to_html(emphasis, output),
        }
    }
    *output += "\n</h";
    *output += level;
    *output += ">\n";
    for item in &section.items {
        match item {
            SectionItem::Paragraph(par) => paragraph_to_html(par, output),
            SectionItem::Section(section) => section_to_html(section, output),
        }
    }
    *output += "</section>\n";
}

pub fn list_to_html(list: &List, output: &mut String) {
    let list_tag = match list.ltype {
        ListType::Distinct => "ol",
        ListType::Identical => "ul",
        ListType::Checked => "ul",
    };
    *output += "<";
    *output += list_tag;
    if list.ltype == ListType::Checked {
        *output += " class=\"checked-list\"";
    }
    *output += ">\n";
    for par in &list.items {
        *output += "<li";
        if par.tags.contains("checked") {
            *output += " class=\"checked-list-item\"";
        }
        *output += ">\n";
        paragraph_to_html(par, output);
        *output += "</li>\n";
    }
    *output += "</";
    *output += list_tag;
    *output += ">\n";
}

pub fn table_to_html(table: &Table, output: &mut String) {
    *output += "<table>\n";
    for row in &table.rows {
        *output += "<tr>\n";
        let item_tag = if row.is_header {
            "th"
        } else {
            "td"
        };
        for par in &row.items {
            *output += "<";
            *output += item_tag;
            *output += ">\n";
            paragraph_to_html(par, output);
            *output += "</";
            *output += item_tag;
            *output += ">\n";
        }
        *output += "</tr>\n";
    }
    *output += "</table>\n";
}

pub fn code_to_html(code: &Result<CodeBlock, CodeIdentError>, output: &mut String) {
    match code {
        Ok(code) => {
            *output += "<pre><code lang=\"";
            *output += &code.language;
            *output += "\">\n";
            *output += &code.code;
            *output += "\n</code></pre>\n";
        },
        Err(_) => {
            *output +=
                "<span class=\"code-indentation-error\">incodoc code indentation error</span>";
        },
    }
}

pub fn emphasis_to_html(em: &Emphasis, output: &mut String) {
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
