# Playing with mdbook

This chapter focus on [mdbook](https://rust-lang.github.io/mdBook/index.html)

<!-- toc -->

## Code snippets

### As usual
Python
```python
def main() {
    print("Hello, world")
}
```

JS
```js
function main() {
    console.log("Hello, world")
}
```

[MathJax](https://www.mathjax.org/) support *(warning: [syntax changes for mdbook](https://rust-lang.github.io/mdBook/format/mathjax.html))*

\\( \int x dx = \frac{x^2}{2} + C \\) 

### Rust runnable and editable snippet
Snippet can be run directly into the browser!
```rust
fn main() {
    println!("Hello, world");
}
```

Snippet can be editable
```rust,editable
fn main() {
    // Feel free to complete this function
}
```

### Including files and lines

Directly include your file
```rust, editable
// mdbook/hello-world.rs file
{{#include mdbook/hello-world.rs}}
```

Specific lines
```rust
// mdbook/hello-world.rs file : line 2 to 3
{{#include mdbook/hello-world.rs:2:3}}
```

Snippet and lines
```rust,editable
fn main() {
    let name = "World from markdown!";
{{#include mdbook/hello-world.rs:3}} // Only this line is part of hello-world.rs
}
```

### Limitation : external dependencies inclusion
```rust
{{#include mdbook/random.rs}}
```

## Mermaid support
A graph
```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```