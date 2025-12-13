use incodoc::*;

use std::collections::HashSet;

pub fn doc_to_html_string(doc: &Doc) -> String {
    let mut res = String::new();
    doc_to_html(doc, &mut res);
    res
}

pub fn doc_to_html(doc: &Doc, output: &mut String) {
    *output += "<html";
    tags_to_html(&doc.tags, true, false, output);
    *output += "\n";
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
    ensure_newline(output);
    *output += "<nav";
    tags_to_html(&nav.tags, true, false, output);
    *output += "\n";
    if !nav.description.is_empty() {
        *output += "<h1>\n";
        *output += &nav.description;
        *output += "\n</h1>\n";
    }
    for link in &nav.links {
        ensure_newline(output);
        link_to_html(link, output);
    }
    for sub in &nav.subs {
        nav_to_html(sub, output);
    }
    ensure_newline(output);
    *output += "</nav>\n";
}

pub fn section_to_html(section: &Section, output: &mut String) {
    ensure_newline(output);
    *output += "<section";
    tags_to_html(&section.tags, true, false, output);
    *output += "\n";
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
    tags_to_html(&section.heading.tags, true, false, output);
    *output += "\n";
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
    ensure_newline(output);
    *output += "</section>\n";
}

pub fn paragraph_to_html(par: &Paragraph, output: &mut String) {
    ensure_newline(output);
    *output += "<p";
    tags_to_html(&par.tags, true, false, output);
    *output += "\n";
    for item in &par.items {
        match item {
            ParagraphItem::Text(text) => *output += text,
            ParagraphItem::MText(mtext) => mtext_to_html(mtext, output),
            ParagraphItem::Em(emphasis) => emphasis_to_html(emphasis, output),
            ParagraphItem::Link(link) => link_to_html(link, output),
            ParagraphItem::Code(code) => code_to_html(code, output),
            ParagraphItem::List(list) => list_to_html(list, output),
            ParagraphItem::Table(table) => table_to_html(table, output),
        }
    }
    ensure_newline(output);
    *output += "</p>\n";
}

pub fn mtext_to_html(TextWithMeta { text, tags, .. }: &TextWithMeta, output: &mut String) {
    // inline code is not handled differently
    // it will show up as a class
    // and the css can handle it
    *output += "<span";
    tags_to_html(tags, true, true, output);
    *output += text;
    *output += "</span>";
}

pub fn link_to_html(link: & Link, output: &mut String)  {
    *output += "<a ";
    *output += "href=\"";
    *output += &link.url;
    *output += "\" target=\"";
    *output += "_blank";
    tags_to_html(&link.tags, true, true, output);
    for item in &link.items {
        match item {
            LinkItem::String(text) => *output += text,
            LinkItem::Em(em) => emphasis_to_html(em, output),
        }
    }
    *output += "</a>";
}

pub fn list_to_html(list: &List, output: &mut String) {
    ensure_newline(output);
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
    ensure_newline(output);
    *output += "<table";
    tags_to_html(&table.tags, true, false, output);
    *output += "\n";
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
    ensure_newline(output);
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

pub fn tags_to_html(tags: &HashSet<String>, end_tag: bool, backslash: bool, output: &mut String) {
    if !tags.is_empty() {
        *output += " class=\"";
        for tag in tags {
            *output += tag;
            *output += " ";
        }
    }
    if end_tag {
        if backslash {
            *output += "\"";
        }
        *output += ">";
    }
}

pub fn ensure_newline(output: &mut String) {
    if !output.ends_with('\n') {
        *output += "\n";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
