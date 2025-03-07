Example 1:
```rs
fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    s = String::from("goodbye");
    println!("{}", ref3.to_uppercase());
}
```

The prorgam will **not** compile. `ref1` borrows the ownership of `s`, so `s` is dropped.
When s is called in `s = String::from("goodbye");`, it's trying to assign a value to an
already borrowed variable. Moreover, immutable borrows to `s` already exist (`ref1`, 
`ref2`, `ref3`), so we can't mutate the value of `s`.


Example 2:
```rs
fn drip_drop() -> &String {
    let s = String::from("hello world!");
    return &s;
}
```

The program will **not** compile. The function attempts to return a reference to a string,
but `s` is a local value that is dropped when the function exits. Thus, trying to `return &s`
will result in an error trying to return a dropped value (`s` is deallocated).


Example 3:
```rs
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: String = v[0];
    println!("{}", s2);
}
```

The program will **not** compile. We are trying to move out of a vector. That is, ownership
of `s1` is moved into `v`, so we can't move it out of the `vec` in `let s2: String = v[0];`
because you're trying to take its ownership. Doing so would create an invalid vector (missing 
element).
