use incodoc::*;
use incodoc::actions::toc::*;

use std::collections::{ HashSet, HashMap };

pub mod config;

use config::*;

pub fn doc_to_html_string(doc: &mut Doc, conf: &Config) -> String {
    let mut res = String::new();
    doc.insert_table_of_contents_section_ids();
    doc_to_html(doc, conf, &mut res);
    res
}

pub type Blobs<'a> = (Option<&'a str>, Option<&'a str>);

pub fn doc_to_html(doc: &Doc, conf: &Config, output: &mut String) {
    if !matches!(conf.include, Include::BodyOnly) {
        *output += "<!DOCTYPE html>\n";
        *output += "<html";
        tags_to_html(&doc.tags, false, false, output);
        string_prop_to_html("lang", &doc.props, output);
        *output += ">\n";
        *output += "<head>\n";
        if let Some(PropVal::String(css)) = doc.props.get("css") {
            header_link_to_html(&HeaderLink::Css{ href: css.to_string() }, output);
        }
        for link in &conf.header_links {
            header_link_to_html(link, output);
        }
        if let Some(PropVal::String(title)) = doc.props.get("title") {
            *output += "<title>\n";
            *output += title;
            *output += "\n</title>\n";
        }
        *output += "</head>\n";
    }
    *output += "<body>\n";
    if let Include::Augmented(pre, _) = &conf.include {
        *output += pre;
        ensure_newline(output);
    }
    let mut nav = String::new();
    for n in &doc.navs {
        nav_to_html(n, &mut nav, 0, &conf.nav);
    }
    if conf.nav.position == Position::Top {
        *output += &nav;
    }
    let toc = doc.get_table_of_contents(&conf.table_of_contents.filter);
    let meta_says_include_toc = if let Some(PropVal::String(s)) = doc.props.get("table-of-contents")
        && s == "include" { true }
    else { false };
    let toc = top_toc_to_html(toc, &conf.table_of_contents, meta_says_include_toc);
    if conf.table_of_contents.position == Position::Top {
        *output += &toc;
    }
    let blobs = (
        if conf.table_of_contents.position == Position::BeforeFirstSubSection {
            Some(toc.as_str())
        } else {
            None
        },
        if conf.nav.position == Position::BeforeFirstSubSection { Some(nav.as_str()) }
        else { None }
    );
    for item in &doc.items {
        match item {
            DocItem::Paragraph(par) => paragraph_to_html(par, output),
            DocItem::Section(section) => section_to_html(section, output, blobs),
        }
    }
    if conf.table_of_contents.position == Position::Bottom {
        *output += &toc;
    }
    if conf.nav.position == Position::Bottom {
        *output += &nav;
    }
    if let Include::Augmented(_, post) = &conf.include {
        *output += post;
        ensure_newline(output);
    }
    *output += "</body>\n";
    if !matches!(conf.include, Include::BodyOnly) {
        *output += "</html>\n";
    }
}

pub fn header_link_to_html(hlink: &HeaderLink, output: &mut String) {
    *output += "<link rel=\"";
    match hlink {
        HeaderLink::Css { href } => {
            *output += "stylesheet\" type=\"text/css\" href=\"";
            *output += href;
            *output += "\">\n";
        },
        HeaderLink::General { rel, ltype, href } => {
            *output += rel;
            *output += "\" type=\"";
            *output += ltype;
            *output += "\" href=\"";
            *output += href;
            *output += "\">\n";
        },
    }
}

pub fn top_toc_to_html(
    toc: Option<TableOfContentsItem>,
    conf: &TableOfContentsConfig,
    meta_says_include: bool,
) -> String {
    let mut res = String::new();
    let include = conf.include != TableOfContentsInclusion::Exclude && meta_says_include;
    if let Some(toc) = toc && !toc.children.is_empty() && include {
        ensure_newline(&mut res);
        res += "<div class=\"table-of-contents\">\n";
        res += "<details open class=\"table-of-contents\">\n";
        res += "<summary>\n";
        res += "<h1>\n";
        res += "table of contents";
        res += "</h1>\n";
        res += "</summary>\n";
        toc_to_html(&toc, &mut res);
        res += "</details>\n";
        res += "</div>\n";
    }
    res
}

pub fn toc_to_html(toc: &TableOfContentsItem, output: &mut String) {
    if toc.item_type != TableOfContentsItemType::Document {
        if toc.link.is_empty() {
            *output += &toc.title;
        } else {
            *output += "<a href=\"";
            *output += &toc.link;
            *output += "\">";
            *output += &toc.title;
            *output += "</a>\n";
        }
    }
    if !toc.children.is_empty() {
        *output += "<ul>\n";
        for child in &toc.children {
            *output += "<li>\n";
            toc_to_html(child, output);
            *output += "</li>\n";
        }
        *output += "</ul>\n";
    }
}

pub fn nav_to_html(nav: &Nav, output: &mut String, depth: usize, conf: &NavConfig) {
    if !conf.include {
        return;
    }
    ensure_newline(output);
    *output += "<nav";
    tags_to_html(&nav.tags, true, false, output);
    *output += "\n";
    *output += "<details ";
    if depth < conf.closed_depth && !(depth == 0 && conf.close_top) {
        *output += "open ";
    }
    *output += "class=\"nav\">\n";
    *output += "<summary>\n";
    *output += "<h1>\n";
    if !nav.description.is_empty() {
        *output += &nav.description;
    } else {
        *output += "navigation";
    }
    *output += "\n</h1>\n";
    *output += "</summary>\n";
    *output += "<ol>\n";
    for link in &nav.links {
        ensure_newline(output);
        *output += "<li>\n";
        link_to_html(link, output);
        *output += "\n</li>\n";
    }
    *output += "</ol>\n";
    for sub in &nav.subs {
        nav_to_html(sub, output, depth + 1, conf);
    }
    ensure_newline(output);
    *output += "</nav>\n";
    *output += "</details>\n";
}

pub fn section_to_html(section: &Section, output: &mut String, blobs: Blobs) {
    ensure_newline(output);
    *output += "<section";
    tags_to_html(&section.tags, false, false, output);
    string_prop_to_html("id", &section.props, output);
    *output += ">\n";
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
    let mut blobs_written = false;
    for item in &section.items {
        match item {
            SectionItem::Paragraph(par) => paragraph_to_html(par, output),
            SectionItem::Section(section) => {
                if !section.tags.contains("blockquote")
                    && !section.tags.contains("blockquote-typed")
                    && !section.tags.contains("footnote-def")
                    && !blobs_written
                {
                    if let (_, Some(nav)) = blobs {
                        *output += nav;
                    }
                    if let (Some(toc), _) = blobs {
                        *output += toc;
                    }
                    blobs_written = true;
                }
                section_to_html(section, output, (None, None));
            },
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
    let tag = if tags.contains("super") {
        "sup"
    } else if tags.contains("sub") {
        "sub"
    } else {
        "span"
    };
    *output += "<";
    *output += tag;
    tags_to_html(tags, true, false, output);
    *output += text;
    *output += "</";
    *output += tag;
    *output += ">";
}

pub fn link_to_html(link: &Link, output: &mut String) {
    if link.tags.contains("image") {
        image_to_html(link, output);
        return;
    }
    *output += "<a ";
    *output += "href=\"";
    *output += &link.url;
    *output += "\" target=\"";
    if link.tags.contains("footnote-ref") {
        *output += "_self";
    } else {
        *output += "_blank";
    }
    *output += "\"";
    tags_to_html(&link.tags, true, false, output);
    for item in &link.items {
        match item {
            LinkItem::String(text) => *output += text,
            LinkItem::Em(em) => emphasis_to_html(em, output),
        }
    }
    *output += "</a>";
}

pub fn image_to_html(link: &Link, output: &mut String) {
    *output += "<img src=\"";
    *output += &link.url;
    *output += "\" alt=\"";
    for item in &link.items {
        match item {
            LinkItem::String(text) => *output += text,
            LinkItem::Em(em) => *output += &em.text,
        }
    }
    *output += "\">";
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
        (EmType::Deemphasis, EmStrength::Light) => ("<span class=\"light-de\">", "</span>"),
        (EmType::Deemphasis, EmStrength::Medium) => ("<span class=\"medium-de\">", "</span>"),
        (EmType::Deemphasis, EmStrength::Strong) => ("<span class=\"strong-de\">", "</span>"),
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
        *output += "\"";
    }
    if end_tag {
        if backslash {
            *output += "\\";
        }
        *output += ">";
    }
}

pub fn string_prop_to_html(prop: &str, props: &HashMap<String, PropVal>, output: &mut String) {
    if let Some(PropVal::String(val)) = props.get(prop) {
        *output += " ";
        *output += prop;
        *output += "=\"";
        *output += val;
        *output += "\"";
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

