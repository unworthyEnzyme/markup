# markup
A markup language with xml's extensibility with better looking syntax

## Why?
I wanted to add highlight to specific lines on a code block. Markdown doesnt' support this and it isn't easy to extend the grammar. So i designed this language.

## Syntax
```
code-block(highlights: [1, 3..5], lang: "ts") {
    "
        function add(a: number, b: number) {
            return a + b;
        }
        const result = add(1, 2);
        console.log(result);
    "
    p { "a js snippet" }
}
```
`code-block` defines a node. You pass your arguments between parentheses. All the arguments must be named. If you don't have any arguments you can omit the parentheses.
`[1, 3..5]` defines a list that contains number `3` and range `3..5`. The second number in range sytax is optional so you can define an open-ended range like `0..`. 
String literals uses `"` and are also a node, so you can pass them as children to other nodes.

## Note
There are still things i need to do like a trait for transforming the AST to something other mediums can understand.
Another thing is escape sequences to string literals.