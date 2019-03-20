//! Run with `cargo run --example readme`

use ::iter_python::{*,
    iter,           // "generator expressions"
    vec_it as vec,  // "list comprehensions"
};

fn main ()
{
    let v = vec![
        2 * x
        for &x_opt in &[None, Some(21), None]
        if let Some(x) = x_opt
    ];
    assert_eq!(
        dbg!(v),
        vec![42],  // classic vec! macro is still there
    );

    let is_square = |n: u32| n == ((n as f64).sqrt().trunc() as u32).pow(2);
    let odds = || iter!(2 * n + 1 for n in 0 ..);
    let sums_of_odds = iter!(odds().take(n).sum() for n in 1 ..);
    assert!(sums_of_odds.take(100).all(is_square));

    const N: usize = 6;
    let line = format!("+-{}-+", "-+-".join(iter!("---" for _ in 1 .. N)));
    // And the following allocates only one `String`:
    let matrix = format!(
        "{line}\n{body}\n{line}\n",
        line=line,
        body="\n".join(iter!(
            f!(
                "| {row} |",
                row=" | ".join(iter!(
                    f!("a{i}{j}", i=i, j=j)
                    for j in 1 .. N
                )),
            ) for i in 1 .. N
        )),
    );
    println!("{}", matrix);
    assert_eq!(matrix, concat!(
        "+-----+-----+-----+-----+-----+\n",
        "| a11 | a12 | a13 | a14 | a15 |\n",
        "| a21 | a22 | a23 | a24 | a25 |\n",
        "| a31 | a32 | a33 | a34 | a35 |\n",
        "| a41 | a42 | a43 | a44 | a45 |\n",
        "| a51 | a52 | a53 | a54 | a55 |\n",
        "+-----+-----+-----+-----+-----+\n",
    ));
}
