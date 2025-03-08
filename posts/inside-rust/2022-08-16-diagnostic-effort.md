+++
layout = "post"
title = "Contribute to the diagnostic translation effort!"
author = "David Wood"
team = "the compiler team <https://www.rust-lang.org/governance/teams/compiler>"
+++

The Rust Diagnostics working group is leading an effort to add support for
internationalization of error messages in the compiler, allowing the compiler
to produce output in languages other than English.

For example, consider the following diagnostic where a user has used a colon to
specify a function's return type instead of an arrow:

```text
error: return types are denoted using `->`
 --> src/main.rs:1:21
  |
1 | fn meaning_of_life(): u32 { 42 }
  |                     ^ help: use `->` instead
```

We could output that diagnostic in Chinese:

<pre lang="zh-CN">
<code class="language-text">错误: 返回类型使用`->`表示
 --> src/main.rs:1:21
  |
1 | fn meaning_of_life(): u32 { 42 }
  |                     ^ 帮助: 使用`->`来代替
</code></pre>

or even in Spanish:

<pre lang="es">
<code class="language-text">error: el tipo de retorno se debe indicar mediante `->`
 --> src/main.rs:1:21
  |
1 | fn meaning_of_life(): u32 { 42 }
  |                     ^ ayuda: utilice `->` en su lugar
</code></pre>

Translated error messages will allow non-native speakers of English to use Rust
in their preferred language.

## What's the current status?
Implementation on diagnostic translation has started, but we're looking for
help!

The core infrastructure for diagnostic translation has been implemented in
`rustc`; this makes it possible for Rust to emit a diagnostic with translated
messages. However, every diagnostic in `rustc` has to be ported to use this new
infrastructure, otherwise they can't be translated. That's a lot of work, so
the diagnostics working group has chosen to combine the translation effort with
a transition to "diagnostic structs" (more on that later) and get both done at
once.

Once most diagnostic messages have been ported to the new infrastructure, then
the diagnostics working group will start creating a workflow for translation
teams to translate all of the diagnostic messages to different languages.

Every pull request related to diagnostic translation is tagged with
[`A-translation`][A-translation].

## Getting involved
There's a lot of work to do on diagnostic translation, but the good news is that
lots of the work can be done in parallel, and it doesn't require background in
compiler development or familiarity with `rustc` to contribute!

**If you are interested in getting involved, take a look at [#100717] to find
out where to get started!** You can ask for help in
[`#t-compiler/wg-diagnostics`] or reach out to [`@davidtwco`].

**Note:** This post isn't going to be updated as the working group iterates on
and improves the workflow for diagnostic translation, so always consult the
developer guide for the most recent documentation on [diagnostic
structs][diag_struct] or [diagnostic translation][diag_translation].

### 1. Setting up a local development environment
Before helping with the diagnostic translation effort, you'll need to get your
development environment set up, so [follow the instructions on the `rustc` dev
guide][getting_started].

### 2. Getting ready to port your first diagnostic
Almost all diagnostics in `rustc` are implemented using the traditional
`DiagnosticBuilder` APIs, which look something like this:

```rust
self.struct_span_err(self.prev_token.span, "return types are denoted using `->`")
    .span_suggestion_short(
        self.prev_token.span,
        "use `->` instead",
        "->".to_string(),
        Applicability::MachineApplicable,
    )
    .emit();
```

`struct_span_err` creates a new diagnostic given two things - a `Span` and a
message. `struct_span_err` isn't the only diagnostic function that you'll
encounter in the compiler's source, but the others are all pretty similar. You
can read more about `rustc`'s diagnostic infrastructure [in the `rustc` dev
guide][errors_and_lints].

`Span`s just identify some location in the user's source code and you can find
them used throughout the compiler for diagnostic reporting (for example, the
location `main.rs:1:21` from the earlier example would have been
`self.prev_token.span`).

In this example, the message is just a string literal (a `&'static str`) which
needs to be replaced by an identifier for the same message in whichever
language was requested.

There are two ways that a diagnostic will be ported to the new infrastructure:

1. If it's a simple diagnostic, without any logic to decide whether or not to
   add suggestions or notes or helps or labels, like in the example above,
   then...
    - [...use a diagnostic derive](#using-a-diagnostic-derive).
2. Otherwise...
    - [...manually implement `SessionDiagnostic`](#manually-implementing-sessiondiagnostic).

In both cases, diagnostics are represented as types. Representing diagnostics
using types is a goal of the diagnostic working group as it helps separate
diagnostic logic from the main code paths.

Every diagnostic type should implement `SessionDiagnostic` (either manually or
automatically). In the `SessionDiagnostic` trait, there's a member function
which converts the trait into a `Diagnostic` to be emitted.

#### Using a diagnostic derive...
Diagnostic derives (either `SessionDiagnostic` for whole diagnostics,
`SessionSubdiagnostic` for parts of diagnostics, or `DecorateLint` for lints)
can be used to automatically implement a diagnostic trait.

To start, create a new type in the `errors` module of the current crate (e.g.
`rustc_typeck::errors` or `rustc_borrowck::errors`) named after your
diagnostic. In our example, that might look like:

```rust
struct ReturnTypeArrow {

}
```

Next, we'll need to add fields with all the information we need - that's just a
`Span` for us:

```rust
struct ReturnTypeArrow {
    span: Span,
}
```

In most cases, this will just be the `Span`s that are used by the original
diagnostic emission logic and values that are interpolated into diagnostic
messages.

After that, we should add the derive, add our error attribute and annotate the
primary `Span` (that was given to `struct_span_err`).

```rust
#[derive(SessionDiagnostic)]
#[error(parser_return_type_arrow)]
struct ReturnTypeArrow {
    #[primary_span]
    span: Span,
}
```

Each diagnostic should have a unique slug. By convention, these always start
with the crate that the error is related to (`parser` in this example). This
slug will be used to find the actual diagnostic message in our translation
resources, which we'll see shortly.

Finally, we need to add any labels, notes, helps or suggestions:

```rust
#[derive(SessionDiagnostic)]
#[error(parser_return_type_arrow)]
struct ReturnTypeArrow {
    #[primary_span]
    #[suggestion(applicability = "machine-applicable", code = "->")]
    span: Span,
}
```

In this example, there's just a single suggestion - to replace the `:` with
a `->`.

Before we're finished, we have to [add the diagnostic messages to the
translation resources..](#adding-translation-resources)

For more documentation on diagnostic derives, see the [diagnostic structs
chapter of the `rustc` dev guide][diag_struct].

#### Manually implementing `SessionDiagnostic`...
Some diagnostics are too complicated to be generated from a diagnostic type
using the diagnostic derive. In these cases, `SessionDiagnostic` can be
implemented manually.

Using the same type as in ["Using a diagnostic
derive..."](#using-a-diagnostic-derive), we can implement `SessionDiagnostic`
as below:

```rust
use rustc_errors::{fluent, SessionDiagnostic};

struct ReturnTypeArrow { span: Span }

impl SessionDiagnostic for ReturnTypeArrow {
    fn into_diagnostic(self, sess: &'_ rustc_session::Session) -> DiagnosticBuilder<'_> {
        sess.struct_span_err(
            self.span,
            fluent::parser_return_type_arrow,
        )
        .span_suggestion_short(
            self.span,
            fluent::suggestion,
            "->".to_string(),
            Applicability::MachineApplicable,
        )
    }
}
```

Instead of using strings for the messages as in the original diagnostic
emission logic, typed identifiers referring to translation resources are used.
Now we just have to [add the diagnostic messages to the translation
resources..](#adding-translation-resources).

#### Examples
For more examples of diagnostics ported to use the diagnostic derive or written
manually, see the following pull requests:

- [#98353](https://github.com/rust-lang/rust/pull/98353)
- [#98415](https://github.com/rust-lang/rust/pull/98415)
- [#97093](https://github.com/rust-lang/rust/pull/97093)
- [#99213](https://github.com/rust-lang/rust/pull/99213)

For more examples, see the pull requests labelled [`A-translation`][A-translation].

#### Adding translation resources...
Every slug in a diagnostic derive or typed identifier in a manual
implementation needs to correspond to a message in a translation resource.

`rustc`'s translations use [Fluent][fluent], an asymmetric translation system.
For each crate in the compiler which emits diagnostics, there is a
corresponding Fluent resource at
`compiler/rustc_error_messages/locales/en-US/$crate.ftl`.

Error messages need to be added to this resource (a macro will then generate
the typed identifier corresponding to the message).

For our example, we should add the following Fluent to
`compiler/rustc_error_messages/locales/en-US/parser.ftl`:

```fluent
parser_return_type_arrow = return types are denoted using `->`
    .suggestion = use `->` instead
```

`parser_return_type_arrow` will generate a `parser::return_type_arrow` type (in
`rustc_errors::fluent`) that can be used with diagnostic structs and the
diagnostic builder.

Subdiagnostics are "attributes" of the primary Fluent message - by convention,
the name of attributes are the type of subdiagnostic, such as "suggestion", but
this can be changed when there are multiple of one kind of subdiagnostic.

Now that the Fluent resource contains the message, our diagnostic is ported!
More complex messages with interpolation will be able to reference other fields
in a diagnostic type (when implemented manually, those are provided as
arguments). See the diagnostic translation documentation [in the `rustc` dev
guide][diag_translation] for more examples.

### 3. Porting diagnostics
Now that you've got a rough idea what to do, you need to find some diagnostics
to port. There's lots of diagnostics to port, so the diagnostic working group
have split the work up to avoid anyone working on the same diagnostic as
someone else - but right now, there aren't many people involved, so just pick a
crate and start porting it :)

Please add the [`A-translation`][A-translation] label to any pull requests that
you make so we can keep track of who has made a contribution! You can use
`rustbot` to label your PR (if it wasn't labelled automatically by
`triagebot`):

```text
@rustbot label +A-translation
```

You can also assign a member of the diagnostics working group to review your PR
by posting a comment with the following content (or including this in the PR
description):

```text
r? rust-lang/diagnostics
```

Even if you aren't sure exactly how to proceed, give it a go and you can ask
for help in [`#t-compiler/wg-diagnostics`] or reach out to [`@davidtwco`].
Check out [#100717] for guidance on where to get started!

## FAQ

### Is this a feature that anyone wants?
Yes! Some language communities prefer native resources and some don't (and
preferences will vary within those communities too). For example,
Chinese-speaking communities have a mature ecosystem of programming language
resources which don't require knowing any English.

### Wouldn't translating X be more worthwhile?
There are many different areas within the Rust project where
internationalization would be beneficial. Diagnostics aren't being prioritized
over any other part of the project, it's just that there is interest within the
compiler team in supporting this feature.

### Couldn't compiler developer time be better spent elsewhere?
Compiler implementation isn't zero-sum: work on other parts of the compiler
aren't impacted by these efforts and working on diagnostic translation doesn't
prevent contributors working on anything else.

### Will translations be opt-in?
Translations will be opt-in, you won't need to use them if you don't want to.

### How will a user select the language?
Exactly how a user will choose to use translated error messages hasn't been
decided yet.

[getting_started]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html
[`#t-compiler/wg-diagnostics`]: https://rust-lang.zulipchat.com/#narrow/stream/147480-t-compiler.2Fwg-diagnostics
[`@davidtwco`]: https://github.com/davidtwco
[errors_and_lints]: https://rustc-dev-guide.rust-lang.org/diagnostics.html#error-messages
[diag_struct]: https://rustc-dev-guide.rust-lang.org/diagnostics/diagnostic-structs.html
[diag_translation]: https://rustc-dev-guide.rust-lang.org/diagnostics/translation.html
[fluent]: http://projectfluent.org/
[A-translation]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AA-translation+sort%3Aupdated-desc
[#100717]: https://github.com/rust-lang/rust/issues/100717
