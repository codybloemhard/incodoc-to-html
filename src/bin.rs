use incodoc_to_html::doc_to_html_string;

use md_to_incodoc::parse_md_to_incodoc;

const INPUT: &str =
"
+++
prop lang en
prop css style.css
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

C | D | E
--|--|--
2 | *3* | ~~4~~
**5** | ***6*** | `let x = 0;`

```rust
let x = 0;
for i in 0..10 {
    println!(\"{}\", yay);
}
```
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    println!("{}", doc_to_html_string(&doc));
}
