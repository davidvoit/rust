// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_snake_case)]

// Error messages for EXXXX errors.
// Each message should start and end with a new line, and be wrapped to 80 characters.
// In vim you can `:set tw=80` and use `gq` to wrap paragraphs. Use `:set tw=0` to disable.
register_long_diagnostics! {

E0001: r##"
This error suggests that the expression arm corresponding to the noted pattern
will never be reached as for all possible values of the expression being
matched, one of the preceding patterns will match.

This means that perhaps some of the preceding patterns are too general, this one
is too specific or the ordering is incorrect.
"##,

E0002: r##"
This error indicates that an empty match expression is illegal because the type
it is matching on is non-empty (there exist values of this type). In safe code
it is impossible to create an instance of an empty type, so empty match
expressions are almost never desired.  This error is typically fixed by adding
one or more cases to the match expression.

An example of an empty type is `enum Empty { }`.
"##,

E0003: r##"
Not-a-Number (NaN) values cannot be compared for equality and hence can never
match the input to a match expression. To match against NaN values, you should
instead use the `is_nan` method in a guard, as in: x if x.is_nan() => ...
"##,

E0004: r##"
This error indicates that the compiler cannot guarantee a matching pattern for
one or more possible inputs to a match expression. Guaranteed matches are
required in order to assign values to match expressions, or alternatively,
determine the flow of execution.

If you encounter this error you must alter your patterns so that every possible
value of the input type is matched. For types with a small number of variants
(like enums) you should probably cover all cases explicitly. Alternatively, the
underscore `_` wildcard pattern can be added after all other patterns to match
"anything else".
"##,

// FIXME: Remove duplication here?
E0005: r##"
Patterns used to bind names must be irrefutable, that is, they must guarantee that a
name will be extracted in all cases. If you encounter this error you probably need
to use a `match` or `if let` to deal with the possibility of failure.
"##,

E0006: r##"
Patterns used to bind names must be irrefutable, that is, they must guarantee that a
name will be extracted in all cases. If you encounter this error you probably need
to use a `match` or `if let` to deal with the possibility of failure.
"##,

E0007: r##"
This error indicates that the bindings in a match arm would require a value to
be moved into more than one location, thus violating unique ownership. Code like
the following is invalid as it requires the entire Option<String> to be moved
into a variable called `op_string` while simultaneously requiring the inner
String to be moved into a variable called `s`.

let x = Some("s".to_string());
match x {
    op_string @ Some(s) => ...
    None => ...
}

See also Error 303.
"##,

E0008: r##"
Names bound in match arms retain their type in pattern guards. As such, if a
name is bound by move in a pattern, it should also be moved to wherever it is
referenced in the pattern guard code. Doing so however would prevent the name
from being available in the body of the match arm. Consider the following:

match Some("hi".to_string()) {
    Some(s) if s.len() == 0 => // use s.
    ...
}

The variable `s` has type String, and its use in the guard is as a variable of
type String. The guard code effectively executes in a separate scope to the body
of the arm, so the value would be moved into this anonymous scope and therefore
become unavailable in the body of the arm. Although this example seems
innocuous, the problem is most clear when considering functions that take their
argument by value.

match Some("hi".to_string()) {
    Some(s) if { drop(s); false } => (),
    Some(s) => // use s.
    ...
}

The value would be dropped in the guard then become unavailable not only in the
body of that arm but also in all subsequent arms! The solution is to bind by
reference when using guards or refactor the entire expression, perhaps by
putting the condition inside the body of the arm.
"##,

E0162: r##"
An if-let pattern attempts to match the pattern, and enters the body if the
match was succesful. If the match is irrefutable (when it cannot fail to match),
use a regular `let`-binding instead. For instance:

struct Irrefutable(i32);
let irr = Irrefutable(0);

// This fails to compile because the match is irrefutable.
if let Irrefutable(x) = irr {
    // This body will always be executed.
    foo(x);
}

// Try this instead:
let Irrefutable(x) = irr;
foo(x);
"##,

E0165: r##"
A while-let pattern attempts to match the pattern, and enters the body if the
match was succesful. If the match is irrefutable (when it cannot fail to match),
use a regular `let`-binding inside a `loop` instead. For instance:

struct Irrefutable(i32);
let irr = Irrefutable(0);

// This fails to compile because the match is irrefutable.
while let Irrefutable(x) = irr {
    ...
}

// Try this instead:
loop {
    let Irrefutable(x) = irr;
    ...
}
"##,

E0297: r##"
Patterns used to bind names must be irrefutable. That is, they must guarantee
that a name will be extracted in all cases. Instead of pattern matching the
loop variable, consider using a `match` or `if let` inside the loop body. For
instance:

// This fails because `None` is not covered.
for Some(x) in xs {
    ...
}

// Match inside the loop instead:
for item in xs {
    match item {
        Some(x) => ...
        None => ...
    }
}

// Or use `if let`:
for item in xs {
    if let Some(x) = item {
        ...
    }
}
"##,

E0301: r##"
Mutable borrows are not allowed in pattern guards, because matching cannot have
side effects. Side effects could alter the matched object or the environment
on which the match depends in such a way, that the match would not be
exhaustive. For instance, the following would not match any arm if mutable
borrows were allowed:

match Some(()) {
    None => { },
    option if option.take().is_none() => { /* impossible, option is `Some` */ },
    Some(_) => { } // When the previous match failed, the option became `None`.
}
"##,

E0302: r##"
Assignments are not allowed in pattern guards, because matching cannot have
side effects. Side effects could alter the matched object or the environment
on which the match depends in such a way, that the match would not be
exhaustive. For instance, the following would not match any arm if assignments
were allowed:

match Some(()) {
    None => { },
    option if { option = None; false } { },
    Some(_) => { } // When the previous match failed, the option became `None`.
}
"##,

E0303: r##"
In certain cases it is possible for sub-bindings to violate memory safety.
Updates to the borrow checker in a future version of Rust may remove this
restriction, but for now patterns must be rewritten without sub-bindings.

// Code like this...
match Some(5) {
    ref op_num @ Some(num) => ...
    None => ...
}

// ... should be updated to code like this.
match Some(5) {
    Some(num) => {
        let op_num = &Some(num);
        ...
    }
    None => ...
}

See also https://github.com/rust-lang/rust/issues/14587
"##

}

register_diagnostics! {
    E0009,
    E0010,
    E0011,
    E0012,
    E0013,
    E0014,
    E0015,
    E0016,
    E0017,
    E0018,
    E0019,
    E0020,
    E0022,
    E0079, // enum variant: expected signed integer constant
    E0080, // enum variant: constant evaluation error
    E0109,
    E0110,
    E0133,
    E0134,
    E0135,
    E0136,
    E0137,
    E0138,
    E0139,
    E0152,
    E0158,
    E0161,
    E0170,
    E0261, // use of undeclared lifetime name
    E0262, // illegal lifetime parameter name
    E0263, // lifetime name declared twice in same scope
    E0264, // unknown external lang item
    E0265, // recursive constant
    E0266, // expected item
    E0267, // thing inside of a closure
    E0268, // thing outside of a loop
    E0269, // not all control paths return a value
    E0270, // computation may converge in a function marked as diverging
    E0271, // type mismatch resolving
    E0272, // rustc_on_unimplemented attribute refers to non-existent type parameter
    E0273, // rustc_on_unimplemented must have named format arguments
    E0274, // rustc_on_unimplemented must have a value
    E0275, // overflow evaluating requirement
    E0276, // requirement appears on impl method but not on corresponding trait method
    E0277, // trait is not implemented for type
    E0278, // requirement is not satisfied
    E0279, // requirement is not satisfied
    E0280, // requirement is not satisfied
    E0281, // type implements trait but other trait is required
    E0282, // unable to infer enough type information about
    E0283, // cannot resolve type
    E0284, // cannot resolve type
    E0285, // overflow evaluation builtin bounds
    E0296, // malformed recursion limit attribute
    E0298, // mismatched types between arms
    E0299, // mismatched types between arms
    E0300, // unexpanded macro
    E0304, // expected signed integer constant
    E0305, // expected constant
    E0306, // expected positive integer for repeat count
    E0307, // expected constant integer for repeat count
    E0308,
    E0309, // thing may not live long enough
    E0310, // thing may not live long enough
    E0311, // thing may not live long enough
    E0312, // lifetime of reference outlives lifetime of borrowed content
    E0313, // lifetime of borrowed pointer outlives lifetime of captured variable
    E0314, // closure outlives stack frame
    E0315, // cannot invoke closure outside of its lifetime
    E0316, // nested quantification of lifetimes
    E0370  // discriminant overflow
}

__build_diagnostic_array! { DIAGNOSTICS }
