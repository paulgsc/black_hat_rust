# Motivation

```sh
//...
// from what I understand this will be garbage collected
// borrow checker will not let you borrow it out of scope
let pointer = RC::new(1);
```

but we can make it long lived by use a smart pointer?!
