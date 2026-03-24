use incodoc_to_html::doc_to_html_string;
use incodoc_to_html::config::*;

use md_to_incodoc::parse_md_to_incodoc;

use incodoc::actions::toc::TableOfContentsItemType;
use incodoc::actions::toc::TableOfContentsFilterType;

use std::collections::HashSet;

const INPUT: &str =
"
+++
prop lang en
prop css style.css
prop table-of-contents include

nav l0
  link link text $ url
  nav l1
    link link text $ url
    nav l2a
      link link text $ url
      link link text $ url
    end
    nav l2b
      link link text $ url
    end
  end
end
+++

# H1

test par with some ***emphasis*** yay.
another line.

nother paragraph.

> quote

with another line.

## H2

par par
[link *text*](url)

- yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay yay
- this
- is
- a
  - [ ] list
  - [x] in
  - [x] a
- list
  1. one
  2. two
  3. three
- sup ^script^
- sub ~script~

![picture of hatsune miku](/home/cody/img/anime/miku-1.jpg)

C | D | E
--|--|--
2 | *3* | ~4~
**5** | ***6*** | `let x = 0;`

> yay yay
> > [!NOTE]
> > extra quote [^longernoteid]

```rust
let x = 0;
for i in 0..10 {
    println!(\"{}\", yay);
}
```

[^longernoteid]:
  line 0.
  line 1.
  `{ code }`
  line 2.
  line 3.
";

fn main() {
    let mut doc = parse_md_to_incodoc(INPUT);
    let conf = Config {
        include: Include::FullDocument,
        header_links: vec![],
        nav: NavConfig {
            include: true,
            close_top: true,
            closed_depth: 1000,
            position: Position::Bottom,
        },
        table_of_contents: TableOfContentsConfig {
            closed: false,
            include: TableOfContentsInclusion::IfSuggested,
            position: Position::BeforeFirstSubSection,
            filter: Some((
                HashSet::from([
                    TableOfContentsItemType::Document,
                    TableOfContentsItemType::Section,
                    TableOfContentsItemType::FootnoteDefinition,
                ]),
                TableOfContentsFilterType::IncludeWithChildren
            )),
        },
        links: LinksConfig {
            local_links_open_in_blank: false,
            footnote_ref_links_open_in_blank: false,
            footnote_ref_prefix: "^".to_string(),
        },
        paragraphs: ParagraphsConfig::JustSmallParagraphs,
    };
    println!("{}", doc_to_html_string(&mut doc, &conf));
}

