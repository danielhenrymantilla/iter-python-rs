//! Run with `cargo run --example readme`

use ::iter_python::{*, vec_it as vec};

fn main ()
{
    use ::itertools::Itertools; // .join() method

    let s = iter!(
        format!("{}!", s),
        for s in " Hello World ".split_whitespace(),
        if !s.is_empty()
    ).join("\n");
    dbg!(&s);
    assert_eq!(
        s,
        "Hello!\nWorld!",
    );

    let v = vec![
        x,
        for &x_opt in &[Some(42), None],
        if let Some(x) = x_opt
    ];
    dbg!(&v);
    assert_eq!(
        v,
        vec![42],  // classic vec! macro is still there
    );
}
