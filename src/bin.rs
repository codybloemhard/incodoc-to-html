use incodoc_to_html_css::doc_to_html_css_string;

use md_to_incodoc::parse_md_to_incodoc;

const INPUT: &str =
"
# H1

test par with some ***emphasis*** yay.
another line.

nother paragraph.
with another line.

## H2

par par
[link *text*](url)

- yay
- this
- is
- a
  - [ ] list
  - [x] in
  - [x] a
- list
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    println!("{}", doc_to_html_css_string(&doc));
}
