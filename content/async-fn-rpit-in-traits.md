+++
layout = "post"
date = 2023-12-21
title = "Announcing `async fn` and return-position `impl Trait` in traits"
author = "Tyler Mandry"
team = "The Async Working Group <https://www.rust-lang.org/governance/wgs/wg-async>"
+++

The Rust Async Working Group is excited to announce major progress towards our goal of enabling the use of `async fn` in traits. Rust 1.75, which hits stable next week, will include support for both `-> impl Trait` notation and `async fn` in traits.

This is a big milestone, and we know many users will be itching to try these out in their own code. However, we are still missing some important features that many users need. Read on for recommendations on when and how to use the stabilized features.

## What's stabilizing

Ever since the stabilization of [RFC #1522] in Rust 1.26, Rust has allowed users to write `impl Trait` as the return type of functions (often called "RPIT"). This means that the function returns "some type that implements `Trait`". This is commonly used to return closures, iterators, and other types that are complex or impossible to write explicitly.

[RFC #1522]: https://rust-lang.github.io/rfcs/1522-conservative-impl-trait.html

```rust
/// Given a list of players, return an iterator
/// over their names.
fn player_names(
    players: &[Player]
) -> impl Iterator<Item = &String> {
    players
        .iter()
        .map(|p| &p.name)
}
```

Starting in Rust 1.75, you can use **return-position `impl Trait` in trait** (RPITIT) definitions and in trait impls. For example, you could use this to write a trait method that returns an iterator:

```rust
trait Container {
    fn items(&self) -> impl Iterator<Item = Widget>;
}

impl Container for MyContainer {
    fn items(&self) -> impl Iterator<Item = Widget> {
        self.items.iter().cloned()
    }
}
```

So what does all of this have to do with async functions? Well, async functions are "just sugar" for functions that return `-> impl Future`. Since these are now permitted in traits, **we also permit you to write traits that use `async fn`**.

```rust
trait HttpService {
    async fn fetch(&self, url: Url) -> HtmlBody;
//  ^^^^^^^^ desugars to:
//  fn fetch(&self, url: Url) -> impl Future<Output = HtmlBody>;
}
```

## Where the gaps lie

### `-> impl Trait` in public traits

The use of `-> impl Trait` is still discouraged for general use in **public** traits and APIs for the reason that users can't put additional bounds on the return type. For example, there is no way to write this function in a way that is generic over the `Container` trait:

```rust
fn print_in_reverse(container: impl Container) {
    for item in container.items().rev() {
        // ERROR:                 ^^^
        // the trait `DoubleEndedIterator`
        // is not implemented for
        // `impl Iterator<Item = Widget>`
        eprintln!("{item}");
    }
}
```

Even though some implementations might return an iterator that implements `DoubleEndedIterator`, there is no way for generic code to take advantage of this without defining another trait. In the future we plan to add a solution for this. For now, `-> impl Trait` is best used in internal traits or when you're confident your users won't need additional bounds. Otherwise you should consider using an associated type.[^nameable]

[^nameable]: Note that associated types can only be used in cases where the type is nameable. This restriction will be lifted once [`impl_trait_in_assoc_type`](https://github.com/rust-lang/rust/issues/63063) is stabilized.

### `async fn` in public traits

Since `async fn` desugars to `-> impl Future`, the same limitations apply. In fact, if you use bare `async fn` in a public trait today, you'll see a warning.

```
warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
 --> src/lib.rs:7:5
  |
7 |     async fn fetch(&self, url: Url) -> HtmlBody;
  |     ^^^^^
  |
help: you can desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change
  |
7 -     async fn fetch(&self, url: Url) -> HtmlBody;
7 +     fn fetch(&self, url: Url) -> impl std::future::Future<Output = HtmlBody> + Send;
  |
```

Of particular interest to users of async are `Send` bounds on the returned future. Since users cannot add bounds later, the error message is saying that you as a trait author need to make a choice: Do you want your trait to work with multithreaded, work-stealing executors?

Thankfully, we have a solution that allows using `async fn` in public traits today! We recommend using the `trait_variant::make` proc macro to let your users choose. This proc macro is part of the [`trait-variant`](https://crates.io/crates/trait-variant) crate, published by the rust-lang org. Add it to your project with `cargo add trait-variant`, then use it like so:

```rust
#[trait_variant::make(HttpService: Send)]
pub trait LocalHttpService {
    async fn fetch(&self, url: Url) -> HtmlBody;
}
```

This creates *two* versions of your trait: `LocalHttpService` for single-threaded executors and `HttpService` for multithreaded work-stealing executors. Since we expect the latter to be used more commonly, it has the shorter name in this example. It has additional Send bounds:

```rust
pub trait HttpService: Send {
    fn fetch(
        &self,
        url: Url,
    ) -> impl Future<Output = HtmlBody> + Send;
}
```

This macro works for async because `impl Future` rarely requires additional bounds other than Send, so we can set our users up for success. See the FAQ below for an example of where this is needed.

### Dynamic dispatch

Traits that use `-> impl Trait` and `async fn` are not object-safe, which means they lack support for dynamic dispatch. We plan to provide utilities that enable dynamic dispatch in an upcoming version of the `trait-variant` crate.

## How we hope to improve in the future

In the future we would like to allow users to add their own bounds to `impl Trait` return types, which would make them more generally useful. It would also enable more advanced uses of `async fn`. The syntax might look something like this:

```rust
trait HttpService = LocalHttpService<fetch(): Send> + Send;
```

Since these aliases won't require any support on the part of the trait author, it will technically make the Send variants of async traits unnecessary. However, those variants will still be a nice convenience for users, so we expect that most crates will continue to provide them.

Of course, the goals of the Async Working Group don't stop with `async fn` in traits. We want to continue building features on top of it that enable more reliable and sophisticated use of async Rust, and we intend to publish a more extensive roadmap in the new year.

## Frequently asked questions

### Is it okay to use `-> impl Trait` in traits?

For private traits you can use `-> impl Trait` freely. For public traits, it's best to avoid them for now unless you can anticipate all the bounds your users might want (in which case you can use `#[trait_variant::make]`, as we do for async). We expect to lift this restriction in the future.

### Should I still use the `#[async_trait]` macro?

There are a couple of reasons you might need to continue using async-trait:

* You want to support Rust versions older than 1.75.
* You want dynamic dispatch.

As stated above, we hope to enable dynamic dispatch in a future version of the `trait-variant` crate.

### Is it okay to use `async fn` in traits? What are the limitations?

Assuming you don't need to use `#[async_trait]` for one of the reasons stated above, it's totally fine to use regular `async fn` in traits. Just remember to use `#[trait_variant::make]` if you want to support multithreaded runtimes.

The biggest limitation is that a type must always decide if it implements the Send or non-Send version of a trait. It cannot implement the Send version *conditionally* on one of its generics. This can come up in the [middleware](https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md) pattern, for example, `RequestLimitingService<T>` that is HttpService if `T: HttpService`.

### Why do I need `#[trait_variant::make]` and `Send` bounds?

In simple cases you may find that your trait appears to work fine with a multithreaded executor. There are some patterns that just won't work, however. Consider the following:

```rust
fn spawn_task(service: impl HttpService + 'static) {
    tokio::spawn(async move {
        let url = Url::from("https://rust-lang.org");
        let _body = service.fetch(url).await;
    });
}
```

Without Send bounds on our trait, this would fail to compile with the error: "future cannot be sent between threads safely". By creating a variant of your trait with Send bounds, you avoid sending your users into this trap.

Note that you won't see a warning if your trait is not public, because if you run into this problem you can always add the Send bounds yourself later.

For a more thorough explanation of the problem, see [this blog post](https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html).[^cut-scope]

[^cut-scope]: Note that in that blog post we originally said we would solve the Send bound problem before shipping `async fn` in traits, but we decided to cut that from the scope and ship the `trait-variant` crate instead.

### Can I mix async fn and impl trait?

Yes, you can freely move between the `async fn` and `-> impl Future` spelling in your traits and impls. This is true even when one form has a Send bound.[^leakage] This makes the traits created by `trait_variant` nicer to use.

```rust
trait HttpService: Send {
    fn fetch(&self, url: Url)
    -> impl Future<Output = HtmlBody> + Send;
}

impl HttpService for MyService {
    async fn fetch(&self, url: Url) -> HtmlBody {
        // This works, as long as `do_fetch(): Send`!
        self.client.do_fetch(url).await.into_body()
    }
}
```

[^leakage]: This works because of *auto-trait leakage*, which allows knowledge of auto traits to "leak" from an item whose signature does not specify them.

### Why don't these signatures use `impl Future + '_`?

For `-> impl Trait` in traits we adopted the [2024 Capture Rules] early. This means that the `+ '_` you often see today is unnecessary in traits, because the return type is already assumed to capture input lifetimes. In the 2024 edition this rule will apply to all function signatures. See the linked RFC for more.

[2024 Capture Rules]: https://rust-lang.github.io/rfcs/3498-lifetime-capture-rules-2024.html

### Why am I getting a "refine" warning when I implement a trait with `-> impl Trait`?

If your impl signature includes more detailed information than the trait itself, you'll [get a warning](https://play.rust-lang.org/?version=beta&mode=debug&edition=2021&gist=6248cfe0419a693d1331ef47c729d1fe):

```rust
pub trait Foo {
    fn foo(self) -> impl Debug;
}

impl Foo for u32 {
    fn foo(self) -> String {
//                  ^^^^^^
//  warning: impl trait in impl method signature does not match trait method signature
        self.to_string()
    }
}
```

The reason is that you may be leaking more details of your implementation than you meant to. For instance, should the following code compile?

```rust
fn main() {
    // Did the implementer mean to allow
    // use of `Display`, or only `Debug` as
    // the trait says?
    println!("{}", 32.foo());
}
```

Thanks to [refined trait implementations][3245] it does compile, but the compiler asks you to confirm your intent to refine the trait interface with `#[allow(refining_impl_trait)]` on the impl.

[3245]: https://rust-lang.github.io/rfcs/3245-refined-impls.html

## Conclusion

The Async Working Group is excited to end 2023 by announcing the completion of our primary goal for the year! Thank you to everyone who helpfully participated in design, implementation, and stabilization discussions. Thanks also to the users of async Rust who have given great feedback over the years. We're looking forward to seeing what you build, and to delivering continued improvements in the years to come.
