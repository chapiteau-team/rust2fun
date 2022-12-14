//! Combinators
//!
//! A combinator is a higher-order function that uses only function application and earlier defined
//! combinators to define a result from its arguments.

/// Compose functions.
///
/// (f ∘ g ∘ h)(x) = f(g(h(x)))
///
/// # Example
///
/// ```
/// use rust2fun::compose;
///
/// let f = |x| x + 1;
/// let g = |x| x / 2;
/// let h = |x| x * 3;
///
/// let composed = compose!(h, g, f, |x: i32| -x);
/// assert_eq!(6, composed(-3));
/// ```
#[macro_export]
macro_rules! compose {
    ($($lst:expr),+) => {
        |x| { compose!(arg x, $($lst),+) }
    };
    (arg $arg:ident, $head:expr, $($tail:expr),*) => {
        $head(compose!(arg $arg, $($tail),*))
    };
    (arg $arg:ident, $last:expr) => {
        $last($arg)
    };
}

/// Pipe functions.
///
/// x |> h |> g |> f = f(g(h(x)))
///
/// # Example
///
/// ```
/// use rust2fun::pipe;
///
/// let f = |x| x + 1;
/// let g = |x| x / 2;
/// let h = |x| x * 3;
///
/// let actual = pipe!(-3, |x: i32| -x, f, g, h);
/// assert_eq!(6, actual);
/// ```
#[macro_export]
macro_rules! pipe {
    ($first:expr, $second:expr, $($tail:expr),*) => {
        pipe!($second($first), $($tail),*)
    };
    ($first:expr, $second:expr) => {
        $second($first)
    };
    ($single:expr) => {
        $single
    };
}

/// Flip arguments of a function *flip(f)(x, y) = f(y, x)* also known as C (Cardinal) combinator.
///
/// # Example
///
/// ```
/// use rust2fun::flip;
///
/// let f = |x: i32, y: i32| 2 * x + y;
/// let flipped = flip!(f);
/// assert_eq!(5, flipped(1, 2));
/// ```
#[macro_export]
macro_rules! flip {
    ($f:expr) => {
        |y, x| $f(x, y)
    };
}

/// The constant with no arguments *constant(x) = () -> x*.
///
/// # Example
///
/// ```
/// use rust2fun::constant;
///
/// let f = constant!(42);
/// assert_eq!(42, f());
/// ```
#[macro_export]
macro_rules! constant {
    ($x:expr) => {
        || $x
    };
}

/// The constant function *constant1(x) = _ -> x* also known as K (Kestrel) combinator.
///
/// # Example
///
/// ```
/// use rust2fun::constant1;
///
/// let actual = Some(1).map(constant1!(2));
/// assert_eq!(Some(2), actual);
/// ```
#[macro_export]
macro_rules! constant1 {
    ($x:expr) => {
        |_| $x
    };
}

/// The constant function with two arguments *constant2(x) = (_, _) -> x*.
///
/// # Example
///
/// ```
/// use rust2fun::constant2;
///
/// let actual = (1..4).reduce(constant2!(2));
/// assert_eq!(Some(2), actual);
/// ```
#[macro_export]
macro_rules! constant2 {
    ($x:expr) => {
        |_, _| $x
    };
}

/// The identity function *id(x) = x* also known as I (Idiot) combinator.
///
/// # Example
///
/// ```
/// use rust2fun::prelude::*;
///
/// let actual = Some(1).map(id);
/// assert_eq!(Some(1), actual);
/// ```
#[inline]
pub const fn id<T>(x: T) -> T {
    x
}

/// The apply function *apply(f, x) = f(x)* also known as A (Apply) combinator.
/// It is the same as function application.
///
/// # Example
///
/// ```
/// use rust2fun::prelude::*;
///
/// let actual = apply(|x| x + 1, 2);
/// assert_eq!(3, actual);
/// ```
#[inline]
pub fn apply<T, R>(f: impl FnOnce(T) -> R, x: T) -> R {
    f(x)
}

/// The application function *apply_to(x, f) = f(x)* also known as T (Thrush) combinator.
///
/// # Example
///
/// ```
/// use rust2fun::prelude::*;
///
/// let actual = apply_to(2, |x| x + 1);
/// assert_eq!(3, actual);
/// ```
#[inline]
pub fn apply_to<T, R>(x: T, f: impl FnOnce(T) -> R) -> R {
    f(x)
}

/// This substitution function, also known as S (Starling) combinator, is used when you have a
/// binary function and you can supply the first argument and can use that value to create the
/// second argument.
///
/// # Example
///
/// ```
/// use rust2fun::prelude::*;
///
/// let generate = |s: &str, l: usize| format!("The string \"{}\" has a length of {}", s, l);
/// let actual = substitution(generate, str::len, "Hello, World!");
/// assert_eq!("The string \"Hello, World!\" has a length of 13", actual);
/// ```
#[inline]
pub fn substitution<A: Copy, B, C, F, G>(f: F, g: G, x: A) -> C
where
    F: FnOnce(A, B) -> C,
    G: FnOnce(A) -> B,
{
    f(x, g(x))
}

/// Provides a means of passing an accumulating function and two branching functions. A value can be
/// applied to the resulting function which will then be applied to each branching function, the
/// results of which will be applied to the accumulating function.
///
/// # Example
///
/// ```
/// use rust2fun::prelude::*;
///
/// let divide = |x: u32, y: usize| x / (y as u32);
/// let sum = |x: &Vec<u32>| x.iter().sum();
/// let actual = converge(divide, sum, Vec::len, &vec![1, 2, 3]);
/// assert_eq!(2, actual);
#[inline]
pub fn converge<A: Copy, B, C, D, F, G, H>(f: F, g: G, h: H, x: A) -> D
where
    F: FnOnce(B, C) -> D,
    G: FnOnce(A) -> B,
    H: FnOnce(A) -> C,
{
    f(g(x), h(x))
}

/// This function, also called the Psi combinator, allows you to call a function on transformations
/// of values. It can be considered the sister of [converge]. Where converge takes one argument and
/// maps it through two unary functions, merging the resulting values with a binary function, psi
/// takes two arguments and runs them each through the same unary function before merging them with
/// the given binary function.
///
/// # Example
///
/// ```
/// use rust2fun::prelude::*;
///
/// let equals = |x, y| x == y;
/// let actual = on(equals, str::to_lowercase, "Str", "STR");
/// assert_eq!(true, actual);
/// ```
#[inline]
pub fn on<A, B, C, F, G>(f: F, mut g: G, x: A, y: A) -> C
where
    F: FnOnce(B, B) -> C,
    G: FnMut(A) -> B,
{
    f(g(x), g(y))
}

/// This function allows for conditionals in composition chains. Unlike [converge], which branches
/// and merges, `if_else` chooses which function to run based on the predicate, and the other
/// function is ignored.
///
/// # Example
///
/// ```
/// use rust2fun::prelude::*;
///
/// let is_even = |x: &i32| x & 1 == 0;
/// let actual = if_else(is_even, |x| x / 2, |x| (x + 1) / 2, 7);
/// assert_eq!(4, actual);
/// ```
pub fn if_else<A, B, P, T, F>(predicate: P, if_true: T, if_false: F, x: A) -> B
where
    P: FnOnce(&A) -> bool,
    T: FnOnce(A) -> B,
    F: FnOnce(A) -> B,
{
    if predicate(&x) {
        if_true(x)
    } else {
        if_false(x)
    }
}

/// This function also known as Y combinator allows for recursive functions to be defined in a more
/// natural way. It takes a function that takes a function and a value, and returns a value. The
/// function is then applied to itself, and the value is returned.
///
/// # Example
///
/// ```
/// use rust2fun::prelude::*;
///
/// let factorial = |f: &dyn Fn(u32) -> u32, x| if x == 0 { 1 } else { x * f(x - 1) };
/// assert_eq!(120, fix(factorial, 5));
///
/// let fibonacci = |f: &dyn Fn(u32) -> u32, x| if x < 2 { x } else { f(x - 1) + f(x - 2) };
/// assert_eq!(8, fix(fibonacci, 6));
/// ```
pub fn fix<T, R, F>(f: F, x: T) -> R
where
    F: Fn(&dyn Fn(T) -> R, T) -> R,
{
    trait Rec<T, R> {
        fn apply(&self, x: T) -> R;
    }

    impl<T, R, F> Rec<T, R> for F
    where
        F: Fn(&dyn Rec<T, R>, T) -> R,
    {
        fn apply(&self, x: T) -> R {
            self(self, x)
        }
    }

    (|rec: &dyn Rec<T, R>, y| f(&|z| rec.apply(z), y)).apply(x)
}
