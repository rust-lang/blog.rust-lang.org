+++
path = "9999/12/31/Project-Goals-2025-November-Update.md"
title = "Project goals update — November 2025"
authors = ["Tomas Sedovic"]

[extra]
team = "the Goals team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

The Rust project is currently working towards a [slate of 41 project goals](https://rust-lang.github.io/rust-project-goals/2025h2/goals.html), with 13 of them designated as [Flagship Goals](https://rust-lang.github.io/rust-project-goals/2025h2/goals.html#flagship-goals). This post provides selected updates on our progress towards these goals (or, in some cases, lack thereof). The full details for any particular goal are available in its associated [tracking issue on the rust-project-goals repository](https://github.com/rust-lang/rust-project-goals/issues?q=is%3Aissue%20state%3Aopen%20label%3AC-tracking-issue).

## Flagship goals


### "Beyond the &#x60;&amp;&#x60;"

<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Continue Experimentation with Pin Ergonomics <a href='https://github.com/rust-lang/rust-project-goals/issues/389' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#389)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="4"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/frank-king">Frank King</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/oli-obk">Oliver Scherer</a>), <a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/traviscross">TC</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/frank-king">Frank King</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/389#issuecomment-3562652614" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @frank-king posted on 2025-11-21:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Status update:</p>
<ul>
<li>[x] <a href="https://github.com/rust-lang/rust/pull/139751">pattern matching support of <code>&amp;pin const|mut T</code> types</a>, merged.</li>
<li>[x] <a href="https://github.com/rust-lang/rust/pull/148397"><code>&amp;pin</code> pattern and <code>ref pin mut</code> binding mode</a>, merged.</li>
<li>[ ] <a href="https://github.com/rust-lang/rust/pull/144537"><code>Drop::pin_drop</code></a>, waiting for review (new updates since the last review).
<ul>
<li><strong>Unresolved question</strong>: the current implementation requires changing the <code>src/docs/book</code> submodule, but the top repo and the sub repo must be changed together to pass the CI tests in both repos. It's because a default body is added to <code>Drop::drop</code> and it becomes a provided method instead of a required method in rustdoc. Is there any way to avoid that? (Possibly keep rustdoc treating <code>Drop::drop</code> as a required method?)</li>
</ul>
</li>
<li>[ ] <a href="https://github.com/rust-lang/rust/pull/144537">coercion between <code>&amp;pin const|mut T</code> and <code>&amp;{mut} T</code></a>, waiting for review (fresh).</li>
</ul>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Design a language feature to solve Field Projections <a href='https://github.com/rust-lang/rust-project-goals/issues/390' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#390)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="3" max="7"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/BennoLossin">Benno Lossin</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/tmandry">Tyler Mandry</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/BennoLossin">Benno Lossin</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->
<div style="padding: 12px 16px; background: var(--blockquote-bg-color); border-bottom: 1px solid #eee;">
<strong>TL;DR.</strong> <ul>
<li>
<p>We have made lot's of progress with the <a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3496572699">novel place-based proposal</a> made by <a href="https://github.com/Nadrieril">@Nadrieril</a>. Since the last update, he released his idea as a <a href="https://nadrieril.github.io/blog/2025/11/11/truly-first-class-custom-smart-pointers.html">blog post</a> and have had an immense amount of discussions on Zulip. There are still many open questions and problems left to solve. If you have any ideas, feel free to share them on Zulip.</p>
</li>
<li>
<p>At the beginning of this month, we explored <a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3476880992">moving projections and <code>&amp;own</code></a>. We also looked into <a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3477956614">reducing the number of projection traits</a>.</p>
</li>
<li>
<p>The PR https://github.com/rust-lang/rust/pull/146307 has been stale for this month, but will be picked up again in December.</p>
</li>
</ul>

</div>

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>3 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3476880992" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-11-01:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3>Moving Projections and <code>&amp;own</code></h3>
<p><em>Moving</em> projections are a third kind of projection that already exists in Rust today for <code>Box</code> as well as any local variable holding a struct. While we won't be including it in an MVP, we still want to make sure that we can extend the language with moving projections. Here is an example with <code>Box</code>:</p>


<pre><code class="language-rust">fn destructure_box(mut b: Box&lt;Struct&gt;) -&gt; Box&lt;Struct&gt; {
    let f1 = b.f1;
    b.f1 = F1::new();
    b
}
</code></pre>
<p>This projection moves the field out of the box, invalidating it in the process. To make it valid again, a new value has to be moved in for that field. Alternatively, the partially valid box can be dropped, this will drop all other fields of <code>Struct</code> and then deallocate the <code>Box</code>. Note that this last property is implemented by compiler magic today and moving projections would allow this special behavior for <code>Box</code> to be a library implementation instead.</p>
<p>To make this kind of projection available for all types, we can make it a proper operation by adding this trait:</p>


<pre><code class="language-rust">pub unsafe trait ProjectMove: Projectable {
    type OutputMove&lt;'a, F: Field&lt;Base = Self::Target&gt;&gt;;
    
    unsafe fn project_move&lt;'a, F: Field&lt;Base = Self::Target&gt;&gt;(
        this: *mut Self,
    ) -&gt; Self::OutputMove&lt;'a, F&gt;;
    
    unsafe fn drop_husk(husk: *mut Self);
}
</code></pre>
<p>Importantly, we also need a <code>drop_husk</code> function which is responsible for cleaning up the &quot;husk&quot; that remains when all fields have been move-projected. In the case of <code>Box</code>, it deallocates the memory. So for <code>Box</code> we could implement this trait like this:</p>


<pre><code class="language-rust">impl&lt;T&gt; ProjectMove for Box&lt;T&gt; {
    type OutputMove&lt;'a, F: Field&lt;Base = T&gt;&gt; = F::Type;

    unsafe fn project_move&lt;'a, F: Field&lt;Base = T&gt;&gt;(
        this: *mut Self,
    ) -&gt; F::Type {
        let ptr = unsafe { (*this).0.pointer.as_ptr() };
        ptr::read(unsafe {
            &lt;*const T as Project&gt;::project::&lt;'a, F&gt;(&amp;raw const ptr)
        })
    }

    unsafe fn drop_husk(husk: *mut Self) {
        // this is exactly the code run by `Box::drop` today, as the compiler
        // drops the `T` before `Box::drop` is run.
        let ptr = (*husk).0;
        unsafe {
            let layout = Layout::for_value_raw(ptr.as_ptr());
            if layout.size() != 0 {
                (*husk).1.deallocate(From::from(ptr.cast()), layout);
            }
        }
    }
}
</code></pre>
<p>To support moving back into a value we have two options:</p>
<ol>
<li>Add a <code>ProjectMoveBack</code> trait that declares an operation which accepts a value that is moved back into the projected one, or</li>
<li>Add <code>&amp;own</code> references.</li>
</ol>
<p>Until now, we have explored the second option, because there are lot's of other applications for <code>&amp;own</code>.</p>
<h5><code>&amp;own</code> References</h5>
<p>A small interlude on <code>&amp;own</code> references.</p>
<p>An <code>&amp;'a own T</code> is a special kind of exclusive reference that <em>owns</em> the value it points to. This means that if you drop an <code>&amp;own T</code>, you also drop the pointee. You can obtain an <code>&amp;own T</code> by constructing it directly to local variable <code>&amp;own my_local</code> or by deriving it from an existing <code>&amp;own</code> via field projections. Smart pointers generally also allow creating <code>&amp;own T</code> from <code>&amp;own SmartPtr&lt;T&gt;</code>.</p>
<p>One important difference to <code>&amp;mut T</code> is that <code>&amp;own</code> is not only temporally unique (i.e. there are no other references to that value not derived from it) but also unique for that value. In other words, one can create at most one <code>&amp;own T</code> to a local variable.</p>


<pre><code class="language-rust">let mut val = Struct { ... };
let x = &amp;own val; //~ HELP: ownership transferred here
drop(x);
let y = &amp;own val; //~ ERROR: cannot own `val` twice
</code></pre>
<p>Since the <code>drop(x)</code> statement drops <code>val</code>, the borrow checker must disallow any future access. However, we are allowed to move a value back into the memory of <code>val</code>:</p>


<pre><code class="language-rust">let mut val = Struct { ... };
let x = &amp;own val;
drop(x);
val = Struct { ... };
let y = &amp;own val;
</code></pre>
<p>The lifetime <code>'a</code> in <code>&amp;'a own T</code> is that of the backing memory. It means that when <code>'a</code> expires, the memory also is no longer valid (or rather it cannot be proven that it is valid after <code>'a</code>). For this reason an <code>&amp;'a own T</code> has to be dropped (or forgotten) before <code>'a</code> expires (since after that it cannot be dropped any more).</p>
<p><code>&amp;own T</code> itself supports moving projections (another indicator that having them is a good idea). However only for types that don't implement <code>Drop</code> (similar to normal struct destructuring -- there are also talks about lifting this requirement, but no new issues arise from projecting <code>&amp;own</code>).</p>
<h5><code>&amp;own</code> and pinning</h5>
<p>To make <code>&amp;pin own T</code> with <code>!(T: Unpin)</code> sound in the face of panics, we have to add drop flags or have unforgettable types. We explored a design using drop flags below; there are separate efforts to experimenting with a <code>Leak</code>/<code>Forget</code> trait ongoing, I think it might be a better solution than drop flags at least for <code>&amp;own</code>.</p>
<p>We need drop flags to ensure the drop guarantee of pinned values. The drop flag will be stored when the original <code>&amp;own</code> is created and it will live on the stack of the function that created it. They are needed for the following scenario:</p>


<pre><code class="language-rust">fn foo() {
    let x = Struct { ... };
    bar(&amp;pin own x);
}

fn bar(x: &amp;pin own Struct) {
    if random() {
        std::mem::forget(x);
    }
    if random() {
        panic!()
    }
}
</code></pre>
<p>Since <code>x</code> is pinned on the stack, it needs to be dropped before <code>foo</code> returns (even if it unwinds). When <code>bar</code> forgets the owned reference, the destructor is not run, if it now panics, the destructor needs to be run in <code>foo</code>. But since it gave away ownership of <code>x</code> to <code>bar</code>, it is possible that <code>bar</code> already dropped <code>x</code> (this is the case when the first <code>random()</code> call returns <code>false</code>). To keep track of this, we need a drop flag in the stack frame of <code>foo</code> that gets set to <code>true</code> when <code>x</code> is dropped.</p>
<p>There are several issues with drop flags:</p>
<ul>
<li>we can't have <code>&amp;'static own T</code> pointing to non-static values (for example coming from a <code>Box::leak_owned</code> function).</li>
<li>field projections complicate things: if we project to a field, then we could possibly forget one field, but drop another
<ul>
<li>solution: just store drop flags not only for the whole struct, but also all transitive fields that implement <code>Drop</code></li>
</ul>
</li>
<li>there is different behavior between <code>&amp;own T</code> and <code>&amp;pin own T</code>, the former can be forgotten and the destructor will not run, the latter can also be forgotten, but the destructor runs regardless.</li>
</ul>
<p>This last point convinces me that we actually want <code>&amp;pin own T: !Leak</code> when <code>T: !Leak</code>; but IIUC, that wouldn't prevent the following code from working:</p>


<pre><code class="language-rust">fn main() { 
    let x = Struct { ... };
    let x = &amp;pin own x;
    Box::leak(Box::new(x));
}
</code></pre>
<h5><code>DerefMove</code></h5>
<p>The <code>DerefMove</code> operation &amp; trait is something that has been discussed in the past (I haven't dug up any discussions on it though). It is the analogous operation of <code>&amp;own</code> to <code>Deref</code>.  We need to figure out the hierarchy wrt. <code>Deref</code> and <code>DerefMut</code>, but ignoring that issue for the moment, here is how <code>DerefMove</code> would look like:</p>


<pre><code class="language-rust">trait DerefMove: DropHusk {
    trait Target: ?Sized;

    fn deref_move(&amp;own self) -&gt; &amp;own Self::Target;
}
</code></pre>
<p>Note the super trait requirement <code>DropHusk</code> -- it provides a special drop operation for <code>Self</code> when the <code>&amp;own Self::Target</code> reference has been dropped. <code>Box&lt;T&gt;</code> for example would deallocate the backing memory via <code>DropHusk</code>. Its definition looks like this:</p>


<pre><code class="language-rust">pub unsafe trait DropHusk {
    unsafe fn drop_husk(husk: *mut Self);
}
</code></pre>
<p>We would of course also use this trait for <code>ProjectMove</code>. Implementing <code>DropHusk</code> on its own does nothing; implementing <code>DerefMove</code> or <code>ProjectMove</code> will make the compiler call <code>drop_husk</code> instead of <code>Drop::drop</code> when the value goes out of scope <em>after</em> it has been projected or <code>DerefMove::deref_move</code> has been called.</p>
<p>We observed that <code>DerefMove</code> is a lot more restrictive in its usability than <code>Deref</code>--- and we need projections to make it actually useful in the common case. The reason for this is that <code>&amp;own</code> can only be created once, but one would like to be able to create it once <em>per field</em> (which is exactly what moving projections allow). Consider this example:</p>


<pre><code class="language-rust">let b = Box::new(Struct { ... });
let field1 = &amp;own b.field1; // desugars to `DerefMove::deref_move`
let field2 = &amp;own b.field2; //~ ERROR: cannot own `b` twice
</code></pre>
<p>The <code>&quot;cannot own `b` twice</code> error comes from the way the deref desugaring works:</p>


<pre><code class="language-rust">let b = Box::new(Struct { ... });
let field1 = &amp;own DerefMove::deref_move(&amp;own b).f1;
let field2 = &amp;own DerefMove::deref_move(&amp;own b).f2;
//                                       ^^^ ERROR: cannot own `b` twice
</code></pre>
<p>Now it's clear that we're trying to create two <code>&amp;own</code> to the same value and that can't work (the issue also arises for <code>&amp;mut</code>, but that already is covered by <code>ProjectExclusive</code>).</p>
<p>We can write this instead:</p>


<pre><code class="language-rust">let b = Box::new(Struct { ... });
let b = &amp;own b;
let field1 = &amp;own b.field1;
let field2 = &amp;own b.field2;
</code></pre>
<p>But that's cumbersome.</p>
<p>We also note that <code>ProjectMove</code> is the correct projection for <code>ArcRef</code>, as it avoids any additional refcount updates. We can rely on the ergonomic refcounting proposal to provide ergonomic ways to clone the value &amp; perform more projections.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3477956614" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-11-02:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h2>Having a single <code>Project</code> trait</h2>
<p>The definition of the now 3 <code>Project*</code> traits are 100% verbatim the same (modulo renaming of course), so we spent some time trying to unify them into a single trait. While we cannot get rid of having to have three traits, we can merge them into a single one by adding a generic:</p>


<pre><code class="language-rust">#[sealed]
pub trait ProjectKind {
    type Ptr&lt;T: ?Sized&gt;;
}

pub enum Shared {}
pub enum Exclusive {}

impl ProjectKind for Shared {
    type Ptr&lt;T: ?Sized&gt; = *const T;
}

impl ProjectKind for Exclusive {
    type Ptr&lt;T: ?Sized&gt; = *mut T;
}

pub trait Projectable {
    type Target;
}

pub unsafe trait Project&lt;Kind: ProjectKind&gt;: Projectable {
    type Output&lt;'a, F: Field&lt;Base = Self::Target&gt;&gt;;

    unsafe fn project&lt;'a, F: Field&lt;Base = Self::Target&gt;&gt;(
        this: Kind::Ptr&lt;Self&gt;,
    ) -&gt; Self::Output&lt;'a, F&gt;;
}
</code></pre>
<p>We would need some more compiler magic to ensure that nobody implements this trait generically, so <code>impl&lt;K&gt; Project&lt;K&gt; for MyType</code>, to keep our approach extensible (this could be an attribute if it is also useful in other cases <code>#[rustc_deny_generic_impls]</code>).</p>
<p>The benefit of merging the definitions is that we only have one single trait that we need to document and we could also add documentation on the <code>ProjectKind</code> types. There are also ergonomic downsides, for example all output types are now called <code>Output</code> and thus need to be fully qualified if multiple projection impls exist (<code>&lt;MyType as Project&lt;Exclusive&gt;&gt;::Output&lt;'_, F&gt;</code> vs <code>MyType::OutputExclusive&lt;'_, F&gt;</code>).</p>
<p>To make this proposal compatible with moving projections, we also either need more compiler magic to ensure that if <code>Kind = Move</code> we require <code>Self: DropHusk</code>. Or we could use associated traits and add one to <code>ProjectKind</code> that's then used in <code>Project</code> (<code>Kind = Shared</code> would then set this to <code>Pointee</code>).</p>
<p>This approach also makes me think a bit more about the syntax, if we discover more projections in the future, it might make sense to go for an extensible approach, like <code>@keyword expr{-&gt;,.@,.,~}ident</code> (so for example <code>@move x-&gt;y</code> or <code>@mut x.y</code>).</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3496572699" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-11-06:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h2>A new Perspective: <a href="https://hackmd.io/%5B@Nadrieril%5D%5B%5D/HJ0tuCO1-e">Projections via Places</a></h2>
<p><a href="https://github.com/Nadrieril">@Nadrieril</a> opened <a href="https://rust-lang.zulipchat.com/#narrow/channel/522311-t-lang.2Fcustom-refs/topic/Field.20projections.20and.20places/with/554030563">this zulip thread</a> with the idea that &quot;<em>The normal rust way to reborrow a field uses places</em>&quot;. He then proceeded to brainstorm a similar design for field projections with a crucial difference: making places the fundamental building block. We had a very long discussion in that thread (exchanging the existing ideas about field projection and the novel place-involving ones) that culminated in this awesome writeup by <a href="https://github.com/Nadrieril">@Nadrieril</a>: <a href="https://hackmd.io/%5B@Nadrieril%5D%5B%5D/HJ0tuCO1-e">https://hackmd.io/[@Nadrieril][]/HJ0tuCO1-e</a>. It is a very thorough document, so I will only be able to summarize it partially here:</p>
<ul>
<li>instead of the <code>Project*</code> traits, we have the <code>Place*</code> traits which govern what kind of place operations are possible on <code>*x</code> given <code>x: MySmartPtr</code>, those are reading, writing and borrowing.</li>
<li>we can allow custom smart pointer reborrowing possibly using the syntax <code>@MySmartPtr &lt;place-expr&gt;</code></li>
<li>we need multi-projections to allow simultaneous existence of <code>&amp;mut x.field.a</code> and <code>&amp;mut x.field.b</code></li>
</ul>
<p>We still have many things to flesh out in this proposal (some of these <a href="https://rust-lang.zulipchat.com/#narrow/channel/522311-t-lang.2Fcustom-refs/topic/Field.20projections.20and.20places/near/553980747">pointed out</a> by <a href="https://github.com/Nadrieril">@Nadrieril</a>):</p>
<ul>
<li>how do FRTs still fit into the equation? And what are the types implementing the <code>Projection</code> trait?</li>
<li>What do we do about non-indirected place containers like <code>MaybeUninit&lt;T&gt;</code>, <code>UnsafeCell&lt;T&gt;</code> and <code>ManuallyDrop&lt;T&gt;</code>?</li>
<li>does <code>BorrowKind</code> work as a model for the borrow checker?</li>
<li>how do we make <code>match</code> ergonomics work nicely?</li>
<li>how do we get around the orphan rule limitations?</li>
<li>several smaller issues/questions...</li>
</ul>
<p>This is a very interesting viewpoint and I'm inclined to make this the main proposal idea. The traits are not too different from the current field projection design and the special borrow checker behavior was also intended at least for the first level of fields. So this is a natural evolution of the field projection proposal. Thanks a lot to <a href="https://github.com/Nadrieril">@Nadrieril</a> for the stellar writeup!</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Reborrow traits <a href='https://github.com/rust-lang/rust-project-goals/issues/399' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#399)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="4"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/aapoalas">Aapo Alasuutari</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/oli-obk">Oliver Scherer</a>), <a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/tmandry">Tyler Mandry</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/aapoalas">Aapo Alasuutari</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/399#issuecomment-3518622087" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @aapoalas posted on 2025-11-11:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>We've worked towards coherence checking of the CoerceShared trait, and have come to a conclusion that (at least as a first step) only one lifetime, the first one, shall participate in reborrowing. Problems abound with how to store the field mappings for CoerceShared.</p>

</div>
</div>
</div>
</details>

</div>


### "Flexible, fast(er) compilation"

<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
build-std <a href='https://github.com/rust-lang/rust-project-goals/issues/274' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#274)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="10"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/davidtwco">David Wood</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/cargo">cargo</a> (<a href="https://github.com/ehuss">Eric Huss</a>), <a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/davidtwco">David Wood</a>), <a href="https://github.com/rust-lang/libs-team">libs</a> (<a href="https://github.com/Amanieu">Amanieu d'Antras</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/adamgemmell">Adam Gemmell</a>, <a href="https://github.com/davidtwco">David Wood</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-3566826267" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @davidtwco posted on 2025-11-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Our first RFC - rust-lang/rfcs#3873 - is in the FCP process, waiting on boxes being checked. rust-lang/rfcs#3874 and rust-lang/rfcs#3875 are receiving feedback which is being addressed.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Production-ready cranelift backend <a href='https://github.com/rust-lang/rust-project-goals/issues/397' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#397)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><img src="https://img.shields.io/badge/Status-Will%20not%20complete%20%3A%28-yellow" alt="Will not complete"></img>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/folkertdev">Folkert de Vries</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/bjorn3">bjorn3</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/bjorn3">bjorn3</a>, <a href="https://github.com/folkertdev">Folkert de Vries</a>, [Trifecta Tech Foundation]</p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Promoting Parallel Front End <a href='https://github.com/rust-lang/rust-project-goals/issues/121' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#121)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="6"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/SparrowLii">Sparrow Li</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/SparrowLii">Sparrow Li</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Relink don&#x27;t Rebuild <a href='https://github.com/rust-lang/rust-project-goals/issues/400' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#400)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><img src="https://img.shields.io/badge/Status-Will%20not%20complete%20%3A%28-yellow" alt="Will not complete"></img>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/yaahc">Jane Lusby</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/cargo">cargo</a> (<a href="https://github.com/weihanglo">Weihang Lo</a>), <a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/oli-obk">Oliver Scherer</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p>@dropbear32, @osiewicz</p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/400#issuecomment-3564457357" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @yaahc posted on 2025-11-21:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>linking this here so people know why there hasn't been any progress on this project goal.</p>
<p><a href="https://rust-lang.zulipchat.com/#narrow/channel/131828-t-compiler/topic/2025H2.20Goal.20Review/near/536084528">#t-compiler &gt; 2025H2 Goal Review @ 💬</a></p>

</div>
</div>
</div>
</details>

</div>


### "Higher-level Rust"

<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Ergonomic ref-counting: RFC decision and preview <a href='https://github.com/rust-lang/rust-project-goals/issues/107' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#107)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="6" max="13"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/nikomatsakis">Niko Matsakis</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/spastorino">Santiago Pastorino</a>), <a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/nikomatsakis">Niko Matsakis</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/nikomatsakis">Niko Matsakis</a>, <a href="https://github.com/spastorino">Santiago Pastorino</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/107#issuecomment-3492295013" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-11-05:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Three new blog posts:</p>
<ul>
<li><a href="https://smallcultfollowing.com/babysteps/blog/2025/10/22/explicit-capture-clauses/">Explicit capture clauses</a></li>
<li><a href="https://smallcultfollowing.com/babysteps/blog/2025/11/05/bikeshedding-handle/">Bikeshedding <code>Handle</code> and other follow-up thoughts</a></li>
<li><a href="https://smallcultfollowing.com/babysteps/blog/2025/11/05/maybe-alias/">But then again...maybe alias?</a></li>
</ul>
<p>The most important conclusions from those posts are</p>
<ul>
<li>Explicit capture clauses would be useful, I proposed one specific syntax but bikeshedding will be required. To be &quot;ergonomic&quot; we need the ability to refer to full places, e.g., <code>move(cx.foo.clone()) || use(cx.foo)</code>.</li>
<li>We should consider <code>Alias</code> or <code>Share</code> as the name for <code>Handle</code> trait; I am currently leaning towards <code>Alias</code> because it can be used as both a noun and a verb and is a bit more comparable to clone -- i.e., you can say &quot;an <code>alias</code> of <code>foo</code>&quot; just like you'd say &quot;a <code>clone</code> of <code>foo</code>&quot;.</li>
<li>We should look for solutions that apply well to <code>clone</code> <em>and</em> <code>alias</code> so that higher-level Rust gets the ergonomic benefits even when cloning &quot;heavier-weight&quot; types to which <code>Alias</code> does not apply.</li>
</ul>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/107#issuecomment-3523163040" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-11-12:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>New blog post:</p>
<ul>
<li>https://smallcultfollowing.com/babysteps/blog/2025/11/10/just-call-clone/</li>
</ul>
<p>Exploring one way to make things more ergonomic while remaining explicit, which is to make <code>.clone()</code> and <code>.alias()</code> (1) understood by move closure desugaring and (2) optimized away when redundant.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Stabilize cargo-script <a href='https://github.com/rust-lang/rust-project-goals/issues/119' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#119)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="36" max="50"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/epage">Ed Page</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/cargo">cargo</a> (<a href="https://github.com/epage">Ed Page</a>), <a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/joshtriplett">Josh Triplett</a>), <a href="https://www.rust-lang.org/governance/teams">lang-docs</a> (<a href="https://github.com/joshtriplett">Josh Triplett</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/epage">Ed Page</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/119#issuecomment-3563046549" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @epage posted on 2025-11-21:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Key developments</p>
<ul>
<li>rust-lang/rust#148051</li>
</ul>
<p>Blockers:</p>
<ul>
<li>rustdoc deciding on and implementing how they want frontmatter handled in doctests</li>
</ul>

</div>
</div>
</div>
</details>

</div>


### "Unblocking dormant traits"

<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Evolving trait hierarchies <a href='https://github.com/rust-lang/rust-project-goals/issues/393' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#393)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="1" max="7"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/cramertj">Taylor Cramer</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/cramertj">Taylor Cramer</a>), <a href="https://github.com/rust-lang/types-team">types</a> (<a href="https://github.com/oli-obk">Oliver Scherer</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/cramertj">Taylor Cramer</a>, <a href="https://github.com/cramertj">Taylor Cramer</a> &amp; others</p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
In-place initialization <a href='https://github.com/rust-lang/rust-project-goals/issues/395' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#395)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="6"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/Darksonn">Alice Ryhl</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/cramertj">Taylor Cramer</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/BennoLossin">Benno Lossin</a>, <a href="https://github.com/Darksonn">Alice Ryhl</a>, <a href="https://github.com/compiler-errors">Michael Goulet</a>, <a href="https://github.com/cramertj">Taylor Cramer</a>, <a href="https://github.com/joshtriplett">Josh Triplett</a>, <a href="https://github.com/nbdd0121">Gary Guo</a>, <a href="https://github.com/yoshuawuyts">Yoshua Wuyts</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/395#issuecomment-3532032037" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @Darksonn posted on 2025-11-14:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>On Nov 12th, there was a mini-design meeting organized by Xiangfei Ding on inplace initialization. The attendees were Xiangfei Ding, Alice Ryhl, Benno Lossin, Tyler Mandry, and Taylor Cramer.</p>
<p>We discussed this document: https://hackmd.io/@rust-for-linux-/H11r2RXpgl</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Next-generation trait solver <a href='https://github.com/rust-lang/rust-project-goals/issues/113' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#113)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="8"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/lcnr">lcnr</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/types-team">types</a> (<a href="https://github.com/lcnr">lcnr</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/BoxyUwU">Boxy</a>, <a href="https://github.com/compiler-errors">Michael Goulet</a>, <a href="https://github.com/lcnr">lcnr</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-3527191755" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @lcnr posted on 2025-11-13:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>The new solver is now officially used by Rust Analyzer: https://rust-analyzer.github.io/thisweek/2025/10/27/changelog-299.html. A huge shoutout to <a href="https://github.com/jackh726">Jack Huey</a> <a href="https://github.com/ChayimFriedman2">Chayim Refael Friedman</a> <a href="https://github.com/ShoyuVanilla">Shoyu Vanilla</a> and <a href="https://github.com/lnicola">Laurențiu Nicola</a> for that work.</p>
<p>On the <code>rustc</code> end <a href="https://github.com/lqd">Rémy Rakic</a> spent a lot of time triaging the most recent crater run. This uncovered a bunch of new edge cases, resulting in <a href="https://github.com/rust-lang/trait-system-refactor-initiative/issues?q=is%3Aissue%20state%3Aopen%20created%3A%3E2025-10-23%20-created%3A%3E2025-11-13">6 new tracked issues</a>.</p>
<p>We've also merged fixes for 4 minor issues over the last 3 weeks: https://github.com/rust-lang/rust/pull/148292 https://github.com/rust-lang/rust/pull/148173 https://github.com/rust-lang/rust/pull/147840. Thanks to <a href="https://github.com/jdonszelmann">Jana Dönszelmann</a>, <a href="https://github.com/tiif">tiif</a> and @adwinwhite for implementing these. @adwinwhite was also instrumental in diagnosing the underlying issue of https://github.com/rust-lang/trait-system-refactor-initiative/issues/245.</p>
<p>Going forward, we intend to continue the crater triage while fixing remaining issues until we're ready for stabilization :&gt; the remaining issues are tracked in https://github.com/orgs/rust-lang/projects/61/views/1.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Stabilizable Polonius support on nightly <a href='https://github.com/rust-lang/rust-project-goals/issues/118' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#118)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="10" max="21"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/lqd">Rémy Rakic</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/types-team">types</a> (<a href="https://github.com/jackh726">Jack Huey</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/amandasystems">Amanda Stjerna</a>, <a href="https://github.com/lqd">Rémy Rakic</a>, <a href="https://github.com/nikomatsakis">Niko Matsakis</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-3576282810" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @lqd posted on 2025-11-25:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Key developments:</p>
<ul>
<li>I prototyped building blocks to fix the liveness soundness issue, but this was deemed too brittle.</li>
<li>so we prepared a meeting for the types team to discuss the problem, and possible solutions.</li>
<li>it turns out the issue is related to another soundness issue for opaque types in the new trait solver, https://github.com/rust-lang/trait-system-refactor-initiative/issues/159, and that tiif is already working on. The same solution is needed for both issues: with the full implied bounds available for opaque types in liveness, we'll able to require all the regions outliving the opaque lower bound to be live, while ignoring the unrelated regions (that the hidden type <em>cannot</em> use anyway). There will be no relevant dead region through which loans flow, and code relying on unused lifetimes being dead (like a lot of ed2024 code with the default capture changes) will still compile</li>
<li>we prepared another types-team meeting to discuss polonius in general, and the alpha algorithm in particular, to share knowledge among the team. This will also be helpful to then on apply member constraints in a location-sensitive manner, since right now they're applied at the SCC level and we need to make sure these constraints with the choice regions are present in the localized subset graph.</li>
<li>niko and tiif have made a lot of progress on adding support for borrow checking in <code>a-mir-formality</code>, so I've also joined these meetings, since we'll also want to model the alpha.</li>
<li>I've looked into Prusti's Place Capability Graphs, and plan to see how to integrate the alpha there, and if possible with the fuzzing capabilities mentioned in the paper, with the usual goal to expand testing as we've mentioned many times</li>
<li>we also had some discussion for a possible masters' student project, and thought about different practical and theoretical topics</li>
</ul>

</div>
</div>
</div>
</details>

</div>


## Goals looking for help


<br>

## Other goal updates

<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Add a team charter for rustdoc team <a href='https://github.com/rust-lang/rust-project-goals/issues/387' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#387)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><img src="https://img.shields.io/badge/Status-Completed%20%3D%29-green" alt="Completed"></img>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/rust">rustdoc</a> (<a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a>)</p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/387#issuecomment-3562439390" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @GuillaumeGomez posted on 2025-11-21:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Done in https://github.com/rust-lang/rust-forge/pull/852.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Borrow checking in a-mir-formality <a href='https://github.com/rust-lang/rust-project-goals/issues/122' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#122)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="12"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/nikomatsakis">Niko Matsakis</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/types-team">types</a> (<a href="https://github.com/nikomatsakis">Niko Matsakis</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/nikomatsakis">Niko Matsakis</a>, <a href="https://github.com/tiif">tiif</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>3 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/122#issuecomment-3492332522" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-11-05:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p><a href="https://github.com/tiif">tiif</a> and I have been meeting weekly here and pushing changes to <a href="https://github.com/nikomatsakis/a-mir-formality-ndm/tree/living-large">the <code>living-large</code> branch of <code>a-mir-formality/nikomatsakis</code></a>.</p>
<p>We are making progress, we have a minirust type checker and the start of a borrow checker. We've decided to try to use a &quot;judgment-like&quot; approach rather than modeling this as dataflow, as I believe it will give greater insight into the &quot;structure&quot; of the trait checker.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/122#issuecomment-3522372826" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-11-12:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p><a href="https://github.com/tiif">tiif</a>, <a href="https://github.com/jackh726">Jack Huey</a>, and I met today and did <a href="https://github.com/nikomatsakis/a-mir-formality-ndm/commit/18fb2a0e574de9c70fe06cb60a095c566c14d5ca">more work</a> on the &quot;living-large&quot; branch. The borrow checker judgments are taking shape. My expectation is that we will walk the CFG, tracking the sets of borrows that have occurred so far. At each statement, we will have a judgment that looks at (a) the subtyping relations generated by the type check (flow-insensitive, like NLL); (b) the loans issued so far and not killed; and (c) the live places that may be accessed later. We'll require then that if you are accessing a place P, then there are no loans accessible from a live place that have borrowed P in an incompatible way.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/122#issuecomment-3553267434" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-11-19:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Continued work this week:</p>
<p>Elaborated some on the definition of the when an access or a statement is valid. We are working our way towards what we believe will be a &quot;largely accurate&quot; model of today's NLL -- obviously we'll then want to test it and compare behavior around various edge cases.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
C++/Rust Interop Problem Space Mapping <a href='https://github.com/rust-lang/rust-project-goals/issues/388' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#388)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="2"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/baumanj">Jon Bauman</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/oli-obk">Oliver Scherer</a>), <a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/tmandry">Tyler Mandry</a>), <a href="https://github.com/rust-lang/libs-team">libs</a> (<a href="https://github.com/dtolnay">David Tolnay</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/baumanj">Jon Bauman</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/388#issuecomment-3583097293" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @baumanj posted on 2025-11-26:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p><strong>Key developments</strong>: <em>What has happened since the last time. It's perfectly ok to list &quot;nothing&quot; if that's the truth, we know people get busy.</em></p>
<p>Nothing! This is the first update and I have yet to focus attention on the project goal. For context, I am <a href="https://rustfoundation.org/interop-initiative/#jon-bauman-c-rust-interop-software-engineer-lead">employed by the Rust Foundation leading the C++ Interoperability initiative</a> and so far have been executing against the strategy detailed in <a href="https://github.com/rustfoundation/interop-initiative/blob/main/problem-statement.md">the problem statement</a>. Owing to <a href="https://github.com/cplusplus/papers/issues/2475">greater than anticipated success</a> and deadlines related to WG21 meetings, I've been focusing on the <a href="https://github.com/rustfoundation/interop-initiative/blob/main/problem-statement.md#social-interoperability">Social Interoperability strategy</a> recently. I have just reached a point where I can turn more attention to the other strategies and so expect to make progress on this goal soon.</p>
<p><strong>Blockers</strong>: <em>List any Rust teams you are waiting on and what you are waiting for.</em></p>
<p>None; I'm getting excellent support from the Project in everything I'm doing. My successes thus far would not have been possible without them, and there are too many to enumerate in this space. There will be a blog post coming soon detailing the past year of work in the initiative where I intend to go into detail. Watch <a href="https://rustfoundation.org/interop-initiative/">this space</a> for updates.</p>
<p><strong>Help wanted</strong>: <em>Are there places where you are looking for contribution or feedback from the broader community?</em></p>
<p>I am always interested in contribution and feedback. If you're interested, please reach out via <a href="mailto:contact@rustfoundation.org">interop@rustfoundation.org</a> or <a href="https://rust-lang.zulipchat.com/#narrow/channel/427678-t-lang.2Finterop">t-lang/interop</a>.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Comprehensive niche checks for Rust <a href='https://github.com/rust-lang/rust-project-goals/issues/262' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#262)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="1" max="7"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/1c3t3a">Bastian Kersting</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/saethlin">Ben Kimock</a>), <a href="https://github.com/rust-lang/opsem-team">opsem</a> (<a href="https://github.com/saethlin">Ben Kimock</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/1c3t3a">Bastian Kersting</a>], <a href="https://github.com/jakos-sec">Jakob Koschel</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Const Generics <a href='https://github.com/rust-lang/rust-project-goals/issues/100' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#100)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="1" max="7"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/BoxyUwU">Boxy</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/nikomatsakis">Niko Matsakis</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/BoxyUwU">Boxy</a>, <a href="https://github.com/camelid">Noah Lev</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3492411671" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BoxyUwU posted on 2025-11-05:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Since the lang meeting most progress on this project goal has been unrelated to <code>adt_const_params</code>.</p>
<p>There's been a large amount of work on <code>min_generic_const_args</code>, specifically <a href="https://github.com/camelid">Noah Lev</a>'s PR (rust-lang/rust#139558) which once landed the core of the impl work for the feature will be done. I've reviewed it together with <a href="https://github.com/oli-obk">Oliver Scherer</a> and it's pretty much ready to go other than some small reviews.</p>
<p>Once this PR lands I'm hoping that there should be a fair amount of &quot;smallish&quot; PRs that can be made which could be a good set of PRs to mentor new-ish contributors on.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3591765845" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BoxyUwU posted on 2025-11-29:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Once again most progress here has been on <code>min_generic_const_args</code>.</p>
<p><a href="https://github.com/camelid">Noah Lev</a>'s PR (rust-lang/rust#139558)  has now landed, as well as an additional PR of his: rust-lang/rust#148716. Between the two of these the core impl should be &quot;mostly done&quot; now, atleast with no additional feature gates enabled :).</p>
<p>The next big step is to make the <code>min_generic_const_args</code> prototype work well with <code>adt_const_params</code> which I've implemented myself in rust-lang/rust#149136 and rust-lang/rust#149114. These PRs still need to be reviewed but the bulk of the impl work there is now done. These PRs allow for constructing ADTs where the field values may themselves be const parameters or non-concrete uses of <code>type_const</code>s (ie the values are const argument positions).</p>
<p>Once my PRs have landed I would consider mgca as a <em>prototype</em> to be truly &quot;done&quot; though not done as an actual feature. Huge thanks to camelid for sticking through a bunch of fairly painful PRs to get us to this point.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Continue resolving &#x60;cargo-semver-checks&#x60; blockers for merging into cargo <a href='https://github.com/rust-lang/rust-project-goals/issues/104' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#104)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="2" max="10"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/obi1kenobi">Predrag Gruevski</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/cargo">cargo</a> (<a href="https://github.com/epage">Ed Page</a>), <a href="https://github.com/rust-lang/rust">rustdoc</a> (<a href="https://github.com/adotinthevoid">Alona Enraght-Moony</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/obi1kenobi">Predrag Gruevski</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/104#issuecomment-3477459959" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @obi1kenobi posted on 2025-11-02:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Status update as of November 1</p>
<p><strong>Key developments:</strong></p>
<ul>
<li>Draft PR for exposing implied bounds in rustdoc JSON: https://github.com/rust-lang/rust/pull/148379</li>
<li>A concrete plan for how that new info turns into dozens of new lints covering many kinds of bounds</li>
</ul>
<p>Linting <code>?Sized</code> and <code>'static</code> bounds turned out to be quite a bit more complex than I anticipated. The key issue is that seeing <code>T: Foo + ?Sized</code> does not guarantee that <code>T</code> can be unsized, since we might have <code>Foo: Sized</code> which renders the <code>?Sized</code> relaxation ineffective. Similarly, seeing <code>T: Foo</code> might also non-obviously imply <code>T: 'static</code> via a similar implied bound.</p>
<p>Failure to correctly account for implied bounds would lead to catastrophic false-positives and false-negatives. For example, changing <code>T: Foo</code> to <code>T: Foo + 'static</code> could be a major breaking change or a no-op, depending on whether we have <code>Foo: 'static</code> (either directly or implicitly via other trait bounds).</p>
<p>We cannot determine implied bounds using information present in rustdoc JSON today, so the rustdoc team and I have been iterating on the best way to compute and include that information in rustdoc JSON. Assuming something similar to <a href="https://github.com/rust-lang/rust/pull/148379">the aforementioned PR</a> becomes part of rustdoc JSON, <code>cargo-semver-checks</code> stands to gain several dozen new lints covering these tricky cases over trait associated types, generic type parameters, and APIT/RPIT/RPITIT.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/104#issuecomment-3567397762" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @obi1kenobi posted on 2025-11-23:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Google Summer of Code 2025 is complete + finally some movement on cross-crate linting! 🚀</p>
<p><strong>Key developments</strong></p>
<ul>
<li>Two students had a successful conclusion of Google Summer of Code working on <code>cargo-semver-checks</code> — <a href="https://blog.rust-lang.org/2025/11/18/gsoc-2025-results/#make-cargo-semver-checks-faster">find more</a> <a href="https://blog.rust-lang.org/2025/11/18/gsoc-2025-results/#enable-witness-generation-in-cargo-semver-checks">details here</a>!</li>
<li>rustdoc JSON now includes rlib information, following the design for cross-crate rustdoc JSON info created at RustWeek 2025: https://github.com/rust-lang/rust/pull/149043</li>
<li>A cargo issue was discovered that prevents this rlib info from being used; it's currently being triaged: https://github.com/rust-lang/cargo/issues/16291</li>
<li>Once that's resolved, we'll have enough here for a basic prototype. Getting features right in dependencies will likely require more work due to having many more cargo-related edge cases.</li>
</ul>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Develop the capabilities to keep the FLS up to date <a href='https://github.com/rust-lang/rust-project-goals/issues/391' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#391)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="5"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/PLeVasseur">Pete LeVasseur</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/rust">bootstrap</a> (<a href="https://github.com/kobzol">Jakub Beránek</a>), <a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/nikomatsakis">Niko Matsakis</a>), <a href="https://github.com/rust-lang/spec">spec</a> (<a href="https://github.com/PLeVasseur">Pete LeVasseur</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/PLeVasseur">Pete LeVasseur</a>, Contributors from Ferrous Systems and others TBD, <code>t-spec</code> and contributors from Ferrous Systems</p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/391#issuecomment-3492360081" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @PLeVasseur posted on 2025-11-05:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p><a href="https://github.com/rust-lang/fls-team/blob/main/meetings/2025-10-31.md">Meeting minutes from meeting held on 2025-10-31</a> (thank you to <a href="https://github.com/tomassedovic">Tomas Sedovic</a> 🥰)</p>
<p>Top-level:</p>
<ul>
<li>Keep high quality bar, merge small, well-vetted changes when possible</li>
<li>Need concentrated effort to get the <a href="https://github.com/rust-lang/fls/pull/580">1.90 FLS updates</a> merged
<ul>
<li><a href="https://github.com/kirtchev-adacore">Hristian Kirtchev</a> and <a href="https://github.com/tshepang">Tshepang Mbambo</a> are navigating this currently with <a href="https://github.com/traviscross">TC</a></li>
</ul>
</li>
<li>Once 1.90 merged, we attempt first go as a team at 1.91</li>
</ul>
<p>Discussion:</p>
<ul>
<li>Suggest that everyone read the Glossary as a starting point</li>
<li>How to best triage / handle incoming issues?
<ul>
<li><a href="https://github.com/traviscross">TC</a> and <a href="https://github.com/PLeVasseur">Pete LeVasseur</a> moved labels onto FLS repo that were needed</li>
<li><a href="https://github.com/PLeVasseur">Pete LeVasseur</a> created <a href="https://github.com/rust-lang/fls/pull/585">issue template</a>, that's in review, to help focus triage</li>
</ul>
</li>
</ul>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/391#issuecomment-3563469397" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @PLeVasseur posted on 2025-11-21:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Meeting notes here: <a href="https://hackmd.io/@rust-fls-team/SkvwkHK1-x">2025-11-14 - <code>t-fls</code> Meeting</a></p>
<p><strong>Key developments</strong>: <a href="https://github.com/rust-lang/fls/pull/580">PR</a> merged for 1.90 update of the FLS. We're preparing now to work on the 1.91 update of the FLS.
<strong>Blockers</strong>: None currently
<strong>Help wanted</strong>: Anyone that's familiar with the <a href="https://doc.rust-lang.org/stable/reference/introduction.html">Rust Reference</a> is more than encouraged to read through the <a href="https://rust-lang.github.io/fls/">FLS</a> to get a sense of it and where further alignment may be possible. Feel free to open issues on the <a href="https://github.com/rust-lang/fls">FLS repo</a> as you find things.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Emit Retags in Codegen <a href='https://github.com/rust-lang/rust-project-goals/issues/392' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#392)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="1" max="7"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/icmccorm">Ian McCormack</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/RalfJung">Ralf Jung</a>), <a href="https://github.com/rust-lang/opsem-team">opsem</a> (<a href="https://github.com/RalfJung">Ralf Jung</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/icmccorm">Ian McCormack</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/392#issuecomment-3519002092" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @icmccorm posted on 2025-11-11:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>We've posted a <a href="https://internals.rust-lang.org/t/pre-rfc-emit-retags-in-codegen/23706">pre-RFC</a> for feedback, and we'll continue updating and expanding the draft <a href="https://borrowsanitizer.com/dev/retags.html">here</a>. This reflects most of the current state of the implementation, aside from tracking interior mutability precisely, which is still TBD but is described in the RFC.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Expand the Rust Reference to specify more aspects of the Rust language <a href='https://github.com/rust-lang/rust-project-goals/issues/394' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#394)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="4"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/joshtriplett">Josh Triplett</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://www.rust-lang.org/governance/teams">lang-docs</a> (<a href="https://github.com/joshtriplett">Josh Triplett</a>), <a href="https://github.com/rust-lang/spec">spec</a> (<a href="https://github.com/joshtriplett">Josh Triplett</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/Amanieu">Amanieu d'Antras</a>, <a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a>, <a href="https://github.com/jackh726">Jack Huey</a>, <a href="https://github.com/joshtriplett">Josh Triplett</a>, <a href="https://github.com/lcnr">lcnr</a>, <a href="https://github.com/m-ou-se">Mara Bos</a>, <a href="https://github.com/petrochenkov">Vadim Petrochenkov</a>, <a href="https://github.com/yaahc">Jane Lusby</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/394#issuecomment-3523004665" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @joshtriplett posted on 2025-11-12:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>We're putting together a prototype/demo of our reference changes at https://rust-lang.github.io/project-goal-reference-expansion/ . This includes a demonstration of tooling changes to provide stability markers (both &quot;documenting unstable Rust&quot; and &quot;unstable documentation of stable Rust&quot;).</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Finish the libtest json output experiment <a href='https://github.com/rust-lang/rust-project-goals/issues/255' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#255)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="8"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/epage">Ed Page</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/cargo">cargo</a> (<a href="https://github.com/epage">Ed Page</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/epage">Ed Page</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/255#issuecomment-3563042951" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @epage posted on 2025-11-21:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Key developments:</p>
<ul>
<li>libtest2:
<ul>
<li><code>#[test]</code> macro added</li>
<li>Support for <code>should_panic</code></li>
<li>Support for <code>ignore</code></li>
<li>Support for custom error types</li>
<li>compile-fail tests for macros</li>
</ul>
</li>
</ul>
<p>Blockers</p>
<ul>
<li>None</li>
</ul>
<p>Help wanted:</p>
<ul>
<li>Round out <a href="https://github.com/assert-rs/libtest2/issues?q=is%3Aissue%20state%3Aopen%20label%3AA-libtest2">libtest compatibility</a></li>
</ul>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Finish the std::offload module <a href='https://github.com/rust-lang/rust-project-goals/issues/109' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#109)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="4" max="8"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/ZuseZ4">Manuel Drehwald</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/ZuseZ4">Manuel Drehwald</a>), <a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/traviscross">TC</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/ZuseZ4">Manuel Drehwald</a>, LLVM offload/GPU contributors</p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-3554422242" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @ZuseZ4 posted on 2025-11-19:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h2>Automatic Differentiation</h2>
<p>Time for the next update. By now, we've had <code>std::autodiff</code> for around a year in upstream rustc, but not in nightly. In order to get some more test users, I asked the infra team to re-evaluate just shipping <code>autodiff</code> as-is. This means that for the moment, we will increase the binary size of rustc by ~5%, even for nightly users who don't use this feature. We still have an open issue to avoid this overhead by using dlopen, please reach out if you have time to help. Thankfully, my request was accepted, so I spent most of my time lately preparing that release.</p>
<ol>
<li>As part of my cleanup I went through old issues, and realized we now partly support rlib's! That's a huge improvement, because it means you can use autodiff not only in your <code>main.rs</code> file, but also in dependencies (either <code>lib.rs</code>, or even rely on crates that use autodiff). With the help of <a href="https://github.com/saethlin">Ben Kimock</a> I figured out how to get the remaining cases covered, hopefully the PR will land soon.</li>
<li>I started documentation improvements in https://github.com/rust-lang/rust/pull/149082 and https://github.com/rust-lang/rust/pull/148201, which should be visible on the website from tomorrow onwards. They are likely still not perfect, so please keep opening issues if you have questions.</li>
<li>We now provide a helpful error message if a user forgets enabling <code>lto=fat</code>: https://github.com/rust-lang/rust/pull/148855</li>
<li>After two months of work, @sgasho managed to add Rust CI to enzyme! Unfortunately, Enzyme devs broke and disabled it directly, so we'll need to talk about maintaining it as part of shipping Enzyme in nightly.</li>
</ol>
<p>I have the following elements on my TODO list as part shipping AD on nightly</p>
<ol>
<li>Re-enable macOS build (probably easy)</li>
<li>Talk with Enzyme Devs about maintenance</li>
<li>Merge rlib support (under review)</li>
<li>upstream ADbenchmarks from r-l/enzyme to r-l/r as codegen tests (easy)</li>
<li>Write a block post/article for https://blog.rust-lang.org/inside-rust/</li>
</ol>
<h2>GPU offload</h2>
<ol>
<li>The llvm dev talk about GPU programming went great, I got to talk to a lot of other developers in the area of llvm offload. I hope to use some of the gained knowledge soon. Concrete steps planned are the integration of libc-gpu for IO from kernels, as well as moving over my code from the OpenMP API to the slightly lower level liboffload API.</li>
<li>We confirmed that our gpu offload prototype works on more hardware. By now we have the latest AMD APU generation covered, as well as an MI 250X and an RTX 4050. My own Laptop with a slightly older <code>AMD Ryzen 7 PRO 7840U</code> unfortunately turned out to be not supported by AMD drivers.</li>
<li>The offload intrinsic PR by <a href="https://github.com/Sa4dUs">Marcelo Domínguez</a> is now marked as ready, and I left my second round of review. Hopefully, we can land it soon!</li>
<li>I spend some time trying to build and potentially ship the needed offload changes in nightly, unfortunately I still fail to build it in CI: https://github.com/rust-lang/rust/pull/148671.</li>
</ol>
<p>All in all, I think we made great progress over the last month, and it's motivating that we finally have no blockers left for flipping the <code>llvm.enzyme</code> config on our nightly builds.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Getting Rust for Linux into stable Rust: compiler features <a href='https://github.com/rust-lang/rust-project-goals/issues/407' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#407)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="29"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/tomassedovic">Tomas Sedovic</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/WesleyWiser">Wesley Wiser</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p>(depending on the flag)</p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/407#issuecomment-3553298581" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-11-19:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update from the 2025-11-05 meeting.</p>
<h2><code>-Zharden-sls</code> / <a href="https://github.com/rust-lang/rust/pull/136597">rust#136597</a></h2>
<p>Wesley Wiser left a comment on the PR, Andr</p>
<h2>-Zno-jump-tables / <a href="https://github.com/rust-lang/rust/pull/145974">rust#145974</a></h2>
<p>Merged, expected to ship in Rust 1.93. The Linux kernel added support for the new name for the option (<code>-Cjump-tables=n</code>).</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/407#issuecomment-3589135100" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-11-28:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update form the 2025-11-19 meeting:</p>
<h2><code>-Zharden-sls</code> / <a href="https://github.com/rust-lang/rust/pull/136597">rust#136597</a></h2>
<p>Andrew addressed the comment and rebased the PR. It's waiting for a review again.</p>
<h2><code>#![register_tool]</code> / <a href="https://github.com/rust-lang/rust/issues/66079">rust#66079</a></h2>
<p>Tyler Mandry had an alternative proposal where lints would be defined in an external crate and could be brought in via <code>use</code> or something similar: https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/namespaced.20tool.20attrs.</p>
<p>A concern people had was the overhead of having to define a new crate and the potential difficulty with experimenting on new lints.</p>
<p>Tyler suggested adding this as a future possibility to <a href="https://github.com/rust-lang/rfcs/pull/3808">RFC#3808</a> and FCPing it.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Getting Rust for Linux into stable Rust: language features <a href='https://github.com/rust-lang/rust-project-goals/issues/116' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#116)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="17" max="53"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/tomassedovic">Tomas Sedovic</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/joshtriplett">Josh Triplett</a>), <a href="https://www.rust-lang.org/governance/teams">lang-docs</a> (<a href="https://github.com/traviscross">TC</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/dingxiangfei2009">Ding Xiang Fei</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3553428854" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-11-19:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update from the 2025-11-05 meeting.</p>
<h2><code>Deref</code>/<code>Receiver</code></h2>
<p><a href="https://github.com/dingxiangfei2009">Ding Xiang Fei</a> posted his reasoning for the trait split in the <a href="https://rust-lang.zulipchat.com/#narrow/channel/522311-t-lang.2Fcustom-refs/topic/Consequences.20of.20making.20Deref.20a.20subtrait.20of.20Receiver/with/547014978">Zulip thread</a> and suggested adding a second RFC to explain.</p>
<p><a href="https://github.com/traviscross">TC</a> recommended writing a Reference PR. The style forces one to explain the model clearly which should then make writing the RFC easier.</p>
<p>The lang experiment PR for arbitrary self types have feature gates for the two options we're exploring.</p>
<h2>Arbitrary Self Types and <code>derive(CoercePointee)</code> / <a href="https://github.com/rust-lang/rust/issues/44874">tracking issue #44874</a></h2>
<p>theemathas opened an issue <a href="https://github.com/rust-lang/rust/issues/148399"><code>derive(CoercePointee)</code> accepts <code>?Sized + Sized</code> #148399</a>. This isn't a critical issue, just an error that arguably should be a lint.</p>
<p><a href="https://github.com/BoxyUwU">Boxy</a> opened a fix for a <code>derive(CoercePointee)</code> blocker: <a href="https://github.com/rust-lang/rust/pull/136776">Forbid freely casting lifetime bounds of dyn-types
</a>.</p>
<h2><a href="https://github.com/rust-lang/rfcs/pull/3851">RFC #3851: Supertrait Auto-impl</a></h2>
<p><a href="https://github.com/dingxiangfei2009">Ding Xiang Fei</a> is working on the implementation (the parser and HIR interface for it). Ding's also working on a more complete section dedicated to <a href="https://github.com/rust-lang/rfcs/pull/3851#discussion_r2430827437">questions raised by obi1kenobi</a></p>
<h2>Field projections</h2>
<p><a href="https://github.com/BennoLossin">Benno Lossin</a> has been posting super detailed updates on <a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3438476150">the tracking issue</a></p>
<p>We've discussed the idea of virtual places (see <a href="https://rust-lang.zulipchat.com/#narrow/channel/522311-t-lang.2Fcustom-refs/topic/Field.20projections.20and.20places/with/553831123">Zulip thread where they were proposed</a>).</p>
<h2>Inlining C code into Rust code</h2>
<p>Matt Mauer had an idea to compile C code into LLVM bytecode (instead of object file) and then the llvm-link tool to merge them together and treat everything in the second bytecode file as a static inlined function. Matt suggested we could integrate this into the rustc passes.</p>
<p>This would make it easy to inline certain functions into Rust code without full LTO.</p>
<p>Relevant <a href="https://rust-lang.zulipchat.com/#narrow/channel/131828-t-compiler/topic/pre-MCP.20vibe.20check.3A.20-Zinternalize-bitcode/near/546294314">Zulip thread</a>.</p>
<p>This sounds like a good candidate for the next Project Goals period.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3589241016" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-11-28:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update from the 2025-11-19 meeting.</p>
<h2><code>rustdoc</code> checking for private and hidden items (<a href="https://github.com/rust-lang/rust/issues/149105">rust##149105</a> &amp; <a href="https://github.com/rust-lang/rust/issues/149106">rust#149106</a>)</h2>
<p>Miguel proposed Rust Doc checking for invalid links to items that are hidden or private even if no docs are built for them. This can help catch typos or dead links because the docs became out of date.</p>
<p>Guillaume was much more open to this being a toggle, lolbinarycat opened a PR here: https://github.com/rust-lang/rust/pull/141299</p>
<h2><code>unsafe_op_in_unsafe_fn</code> not respected in imported declarative macros <a href="https://github.com/rust-lang/rust/issues/112504">rust#112504</a></h2>
<p>This lint doesn't trigger when importing a declarative macro that's calling unsafe code without having an <code>unsafe</code> block and without a <code>SAFETY</code> comment.</p>
<p>The lint is only triggered when the macro was actually used.</p>
<h2>Fix for <code>imports_granularity</code> is not respected for <code>#[cfg]</code>'d items / <a href="https://github.com/rust-lang/rustfmt/issues/6666">rustfmt#6666</a></h2>
<p>Ding opened a PR to fix this: https://github.com/rust-lang/rustfmt/issues/6666</p>
<h2>rustfmt trailing comma hack</h2>
<p>Ding and Manish were talking about writing up a proper fix for the vertical layout that's currently being solved by the <code>, //,</code> hack</p>
<h2><code>TypeId</code> layout</h2>
<p>This has been discussed in https://github.com/rust-lang/rust/pull/148265 and https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/TypeID.20design/near/560189854.</p>
<p>Apiraino proposed a compiler design meeting here: https://github.com/rust-lang/compiler-team/issues/941. That meeting has not been scheduled yet, though.</p>
<h2><code>Deref</code> / <code>Receiver</code></h2>
<p>Following TC's recommendation, Ding is drafting the Reference PR.</p>
<h2>Arbitrary Self Types and <code>derive(CoercePointee)</code></h2>
<p>Ding opened a PR to fix unsoundness in the <code>DispatchFromDyn</code> trait: https://github.com/rust-lang/rust/pull/149068</p>
<p>Theemathas opened a question on whether <code>Receiver</code> should by dyn-compatible: https://github.com/rust-lang/rust/issues/149094</p>
<h2><a href="https://github.com/rust-lang/rfcs/pull/3848">RFC #3848: Pass pointers to <code>const</code> in assembly</a></h2>
<p>Merged!</p>
<h2>In-place initialization</h2>
<p>Benno noted that Effects and In-place Init are not compatible with each other: https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init/topic/Fundamental.20Issue.20of.20Effects.20and.20In-place-init/with/558268061</p>
<p>This is going to affect any in-place init proposal.</p>
<p>Benno proposes fixing this with keyword generics. This is a topic that will receive a lot of discussion doing forward.</p>
<h2>Alice has been nominated and accepted as <code>language-advisor</code>. Fantastic news and congratulations!</h2>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Implement Open API Namespace Support <a href='https://github.com/rust-lang/rust-project-goals/issues/256' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#256)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="6"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><img src="https://img.shields.io/badge/Help%20wanted-yellow" alt="Help Wanted" /></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/cargo">cargo</a> (<a href="https://github.com/epage">Ed Page</a>), <a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/b-naber">b-naber</a>), <a href="https://github.com/rust-lang/crates.io">crates-io</a> (<a href="https://github.com/carols10cents">Carol Nichols</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/b-naber">b-naber</a>, <a href="https://github.com/epage">Ed Page</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
MIR move elimination <a href='https://github.com/rust-lang/rust-project-goals/issues/396' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#396)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="5"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/Amanieu">Amanieu d'Antras</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/Amanieu">Amanieu d'Antras</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/Amanieu">Amanieu d'Antras</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/396#issuecomment-3536586486" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @Amanieu posted on 2025-11-15:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>An <a href="https://hackmd.io/QMmT27rBS226tyeeYtA94w">RFC draft</a> covering the MIR changes necessary to support this optimization has been written and is currently being reviewed by T-opsem. It has already received one round of review and the feedback has been incorporated in the draft.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Prototype a new set of Cargo &quot;plumbing&quot; commands <a href='https://github.com/rust-lang/rust-project-goals/issues/264' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#264)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="8"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><img src="https://img.shields.io/badge/Help%20wanted-yellow" alt="Help Wanted" /></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><img src="https://img.shields.io/badge/Help%20wanted-yellow" alt="Help wanted" />, <a href="https://github.com/epage">Ed Page</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Prototype Cargo build analysis <a href='https://github.com/rust-lang/rust-project-goals/issues/398' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#398)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="1" max="4"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/weihanglo">Weihang Lo</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/cargo">cargo</a> (<a href="https://github.com/weihanglo">Weihang Lo</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><img src="https://img.shields.io/badge/Help%20wanted-yellow" alt="Help wanted" /> <a href="https://github.com/weihanglo">Weihang Lo</a>, <a href="https://github.com/weihanglo">Weihang Lo</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/398#issuecomment-3485998755" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @weihanglo posted on 2025-11-04:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Instead of using a full-fledged database like SQLite, we switched to a basic JSONL-based logging system to collect build metrics. A simple design doc can be found here: <a href="https://hackmd.io/K5-sGEJeR5mLGsJLXqsHrw">https://hackmd.io/K5-sGEJeR5mLGsJLXqsHrw</a>.</p>
<p>Here are the recent pull requests:</p>
<ul>
<li>https://github.com/rust-lang/cargo/pull/16150</li>
<li>https://github.com/rust-lang/cargo/pull/16179</li>
</ul>
<p>To enable it, set <code>CARGO_BUILD_ANALYSIS_ENABLED=true</code> or set the Cargo config file like this:</p>


<pre><code class="language-toml">[build.analysis]
enabled = true
</code></pre>
<p>As of today (nightly-2025-11-03), it currently emits <code>build-started</code> and <code>timing-info</code> two log events to <code>$CARGO_HOME/log/</code> (<code>~/.cargo/log/</code> by default). The shape of <code>timing-info</code> JSON is basically the shape of the unstable <code>--timing=json</code>. I anticipate when this is stabilized we don't need <code>--timing=json</code>.</p>
<p>The <code>build.analysis.enable</code> is a non-blocking unstable feature. Unless bugs, should be able to set unconditionally even on stable toolchain. When not supported, it would just warn the unknown config merely.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/398#issuecomment-3571897575" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @weihanglo posted on 2025-11-24:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p><strong>Key developments</strong>: Started emitting basic fingerprint information, and kicked off the refactor of rendering HTML timing report for future report replay through <code>cargo report timings</code> command.</p>
<ul>
<li>https://github.com/rust-lang/cargo/pull/16203</li>
<li>https://github.com/rust-lang/cargo/pull/16282</li>
</ul>
<p><strong>Blockers</strong>: no except my own availability</p>
<p><strong>Help wanted</strong>: Mendy on Zulip brought up log compression (<a href="https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/build.20analysis.20log.20format/near/558293563">#t-cargo &gt; build analysis log format @ 💬</a>) but I personally don't have time looking at it durnig this period. Would love to see people create an issue in rust-lang/cargo and help explore the idea.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
reflection and comptime <a href='https://github.com/rust-lang/rust-project-goals/issues/406' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#406)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="7"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/oli-obk">Oliver Scherer</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/oli-obk">Oliver Scherer</a>), <a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/scottmcm">Scott McMurray</a>), <a href="https://github.com/rust-lang/libs-team">libs</a> (<a href="https://github.com/joshtriplett">Josh Triplett</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p>oli-obk</p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/406#issuecomment-3523383115" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-11-12:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Another related PR:</p>
<p>https://github.com/rust-lang/rust/pull/148820</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Rework Cargo Build Dir Layout <a href='https://github.com/rust-lang/rust-project-goals/issues/401' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#401)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="2"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/ranger-ross">Ross Sullivan</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/cargo">cargo</a> (<a href="https://github.com/weihanglo">Weihang Lo</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/ranger-ross">Ross Sullivan</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/401#issuecomment-3562675932" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @ranger-ross posted on 2025-11-21:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3>Status update November 21, 2025</h3>
<p>October was largely spent <a href="https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/Build.20cache.20and.20locking.20design/with/553994078">working out design details</a> of the build cache and locking design.</p>
<p>https://github.com/rust-lang/cargo/pull/16155 was opened with an initial implementation for fine grain locking for Cargo's <code>build-dir</code> however it needs to be reworked after the design clarifications mentioned above.</p>
<p>In November I had a change of employer so I my focus was largely on that. However, we did make some progress towards locking in https://github.com/rust-lang/cargo/pull/16230 which no longer lock the <code>artifact-dir</code> for <code>cargo check</code>. This is expected to land in <code>1.93.0</code>.</p>
<p>I'm hoping to push fine grain locking forward later this month and in December.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Run more tests for GCC backend in the Rust&#x27;s CI <a href='https://github.com/rust-lang/rust-project-goals/issues/402' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#402)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><img src="https://img.shields.io/badge/Status-Completed%20%3D%29-green" alt="Completed"></img>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/WesleyWiser">Wesley Wiser</a>), <a href="https://github.com/rust-lang/infra-team">infra</a> (<a href="https://github.com/marcoieni">Marco Ieni</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/402#issuecomment-3553408387" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @GuillaumeGomez posted on 2025-11-19:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>This project goal has been completed. I updated the first issue to reflect it. Closing the issue then.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Rust Stabilization of MemorySanitizer and ThreadSanitizer Support <a href='https://github.com/rust-lang/rust-project-goals/issues/403' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#403)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="5"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/jakos-sec">Jakob Koschel</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p>[<a href="https://github.com/1c3t3a">Bastian Kersting</a>](https://github.com/1c3t3a), [<a href="https://github.com/jakos-sec">Jakob Koschel</a>](https://github.com/jakos-sec)</p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/403#issuecomment-3562409693" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @jakos-sec posted on 2025-11-21:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>We've had a bunch of discussions and I opened a MCP (<a href="https://github.com/rust-lang/compiler-team/issues/943">link</a>, <a href="https://rust-lang.zulipchat.com/#narrow/channel/233931-t-compiler.2Fmajor-changes/topic/Allow.20using.20prebuilt.20sanitizer.20libraries.20compiler-team.23943/near/554742718">zulip</a>).</p>
<p>I think the final sentiment was creating new targets for the few sanitizers and platforms that are critical. I'm in the process of prototyping something to get new feedback on it.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Rust Vision Document <a href='https://github.com/rust-lang/rust-project-goals/issues/269' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#269)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="10"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/nikomatsakis">Niko Matsakis</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p>vision team</p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/269#issuecomment-3492115888" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-11-05:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update:</p>
<p><a href="https://github.com/jackh726">Jack Huey</a> has been doing great work building out a system for analyzing interviews. We are currently looking at slicing the data along a few dimensions:</p>
<ul>
<li>What you know (e.g., experience in other languages, how much experience with Rust)</li>
<li>What you are trying to do (e.g., application area)</li>
<li>Where you are trying to do it (e.g., country)</li>
</ul>
<p>and asking <em>essentially</em> the same set of questions for each, e.g., what about Rust worked well, what did not work as well, what got you into Rust, etc.</p>
<p>Our plan is to prepare a draft of an RFC with some major conclusions and next steps also a repository with more detailed analysis (e.g., a deep dive into the Security Critical space).</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
rustc-perf improvements <a href='https://github.com/rust-lang/rust-project-goals/issues/275' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#275)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="4" max="14"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/Jamesbarford">James</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/davidtwco">David Wood</a>), <a href="https://github.com/rust-lang/infra-team">infra</a> (<a href="https://github.com/kobzol">Jakub Beránek</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/Jamesbarford">James</a>, <a href="https://github.com/Kobzol">Jakub Beránek</a>, <a href="https://github.com/davidtwco">David Wood</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/275#issuecomment-3554553608" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @Kobzol posted on 2025-11-19:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>The new system has been running in production without any major issues for a few weeks now. In a few weeks, I plan to start using the second collector, and then announce the new system to Project members to tell them how they can use its new features.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Stabilize public/private dependencies <a href='https://github.com/rust-lang/rust-project-goals/issues/272' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#272)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="10"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><img src="https://img.shields.io/badge/Help%20wanted-yellow" alt="Help Wanted" /></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/cargo">cargo</a> (<a href="https://github.com/epage">Ed Page</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><img src="https://img.shields.io/badge/Help%20wanted-yellow" alt="Help wanted" />, <a href="https://github.com/epage">Ed Page</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Stabilize rustdoc &#x60;doc_cfg&#x60; feature <a href='https://github.com/rust-lang/rust-project-goals/issues/404' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#404)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="4"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/rust">rustdoc</a> (<a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
SVE and SME on AArch64 <a href='https://github.com/rust-lang/rust-project-goals/issues/270' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#270)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="2" max="27"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/davidtwco">David Wood</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/davidtwco">David Wood</a>), <a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/nikomatsakis">Niko Matsakis</a>), <a href="https://github.com/rust-lang/libs-team">libs</a> (<a href="https://github.com/Amanieu">Amanieu d'Antras</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/davidtwco">David Wood</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3492255970" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-11-05:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Notes from our meeting today:</p>
<h2>Syntax proposal: <code>only</code> keyword</h2>
<p>We are exploring the use of a new <code>only</code> keyword to identify &quot;special&quot; bounds that will affect the default bounds applied to the type parameter. Under this proposal, <code>T: SizeOfVal</code> is a regular bound, but <code>T: only SizeOfVal</code> indicates that the <code>T: const Sized</code> default is suppressed.</p>
<p>For the initial proposal, <code>only</code> can only be applied to a known set of traits; one possible extension would be to permit traits with <code>only</code> supertraits to also have <code>only</code> applied to them:</p>


<pre><code class="language-rust">trait MyDeref: only SizeOfVal { }
fn foo&lt;T: only MyDeref&gt;() { }

// equivalent to

trait MyDeref: only SizeOfVal { }
fn foo&lt;T: MyDeref + only SizeOfVal&gt;() { }
</code></pre>
<p>We discussed a few other syntactic options:</p>
<ul>
<li>A <code>^SizeOfVal</code> sigil was appealing due to the semver analogy but rejected on the basis of it being cryptic and hard to google.</li>
<li>The idea of applying the keyword to the <em>type parameter</em> <code>only T: SizeOfVal</code> sort of made sense, but it would not compose well if we add additional families of &quot;opt-out&quot; traits like <code>Destruct</code> and <code>Forget</code>, and it's not clear how it applies to supertraits.</li>
</ul>
<h2>Transitioning target</h2>
<p>After testing, we confirmed that relaxing <code>Target</code> bound will result in significant breakage without some kind of transitionary measures.</p>
<p>We discussed the options for addressing this. One option would be to leverage <a href="https://github.com/Jules-Bertholet/rfcs/blob/implementable-trait-alias/text/3437-implementable-trait-alias.md#removing-a-sized-bound">&quot;Implementable trait aliases&quot; RFC</a> but that would require a new trait (<code>Deref20XX</code>) that has a weaker bound an alias <code>trait Deref = Deref20XX&lt;Target: only SizeOfVal&gt;</code>. That seems very disruptive.</p>
<p>Instead, we are considering an edition-based approach where (in Rust 2024) a <code>T: Target</code> bound is defaulted to <code>T: Deref&lt;Target: only SizeOfVal&gt;</code> and (in Rust 20XX) <code>T: Target</code> is defaulted to <code>T: Deref&lt;Target: only Pointee&gt;</code>. The edition transition would therefore convert bounds to one of those two forms to be fully explicit.</p>
<p>One caveat here is that this edition transition, if implemented naively, would result in stronger bounds than are needed much of the time. Therefore, we will explore the option of using bottom-up analysis to determine when transitioning whether the 20XX bound can be used instead of the more conservative 2024 bound.</p>
<h2>Supertrait bounds</h2>
<p>We explored the implications of weakening supertrait bounds a bit, looking at this example</p>


<pre><code class="language-rust">trait FooTr&lt;T: ?Sized&gt; {}

struct Foo&lt;T: ?Sized&gt;(std::marker::PhantomData&lt;T&gt;);

fn bar&lt;T: ?Sized&gt;() {}

trait Bar: FooTr&lt;Self&gt; /*: no longer MetaSized */ {
  //       ^^^^^^^^^^^ error!
    // real examples are `Pin` and `TypeOf::of`:
    fn foo(&amp;self, x: Foo&lt;Self&gt;) {
        //        ^^^^^^^^^^^^ error!
        bar::&lt;Self&gt;();
        // ^^^^^^^^^^ error!
          
      
        // real examples are in core::fmt and core::iter:
        trait DoThing {
            fn do_thing() {}
        }
        
        impl&lt;T: ?Sized&gt; DoThing for T {
            default fn do_thing() {}
        }
        
        impl&lt;T: Sized&gt; DoThing for T {
            fn do_thing() {}
        }
        
        self.do_thing();
        // ^^^^^^^^^^^^^ error!
        // specialisation case is not an issue because that feature isn't stable, we can adjust core, but is a hazard with expanding trait hierarchies in future if stabilisation is ever stabilised
    }
}
</code></pre>
<p>The <code>experimental_default_bounds</code> work originally added <code>Self: Trait</code> bounds to default methods but <a href="https://github.com/rust-lang/rust/commit/bd089e1e6e52256c0535f19f58b4b6fe9609b70c">moved away from that</a> because it could cause region errors (<a href="https://github.com/rust-lang/rust/issues/138781#issuecomment-3144652079">source 1</a> / <a href="https://github.com/rust-lang/rust/pull/144679#issuecomment-3144657692">source 2</a>). We expect the same would apply to us but we are not sure.</p>
<p>We decided not to do much on this, the focus remains on the <code>Deref::Target</code> transition as it has more uncertainty.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3566824156" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @davidtwco posted on 2025-11-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>No progress since [<a href="https://github.com/nikomatsakis">Niko Matsakis</a>'s last comment](https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3492255970) - intending to experiment with resolving challenges with <code>Deref::Target</code> and land the SVE infrastructure with unfinished parts for experimentation.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Type System Documentation <a href='https://github.com/rust-lang/rust-project-goals/issues/405' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#405)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="1" max="4"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/BoxyUwU">Boxy</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/rust-lang/types-team">types</a> (<a href="https://github.com/boxyuwu">Boxy</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/BoxyUwU">Boxy</a>, <a href="https://github.com/lcnr">lcnr</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/405#issuecomment-3492452349" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BoxyUwU posted on 2025-11-05:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>A bit late on this update but I've sat down with <a href="https://github.com/lcnr">lcnr</a> a little while back and we tried to come up with a list of topics that we felt fell under type system documentation. This is an entirely unordered list and some topics may already be adequately covered in the dev guide already.</p>
<p>Regardless this effectively serves as a &quot;shiny future&quot; for everything I'd like to have documentation about somewhere (be it dev guide or in-tree module level documentation):</p>
<ul>
<li>opaque types
<ul>
<li>non defining vs defining uses</li>
<li>member constraints (borrowck overlap)</li>
<li>checking item bounds</li>
<li>high level normalization/opaque type storage approach (new solver)</li>
<li>normalization incompleteness</li>
<li>method/function incompleteness</li>
<li>how does <code>use&lt;...&gt;</code> work</li>
<li><code>'erased</code> regions causes problems with outlives item bounds in liveness</li>
<li>consistency across defining scopes</li>
<li>RPITIT inference? does this have special stuff</li>
<li>capturing of bound vars in opaques under binders, <code>Fn</code> bounds are somewhat special in relation to this</li>
<li>opaques inheriting late bound function parameters</li>
</ul>
</li>
<li>non opaque type, <code>impl Trait</code>
<ul>
<li>RPITIT in traits desugaring</li>
<li><code>impl Trait</code> in bindings</li>
<li>APIT desugaring impl details</li>
</ul>
</li>
<li>const generics
<ul>
<li>anonymous constants</li>
<li>ConstArgHasType</li>
<li>TSVs vs RVs and generally upstream doc from lang meeting to dev guide</li>
<li>deterministic CTFE requirement</li>
</ul>
</li>
<li>HIR typeck
<ul>
<li>expectations (and how used incorrectly :3)</li>
<li>method lookup + assorted code cleanups</li>
<li>coercions</li>
<li>auto-deref/reborrows (in coercions/method selection)</li>
<li>closure signature inference</li>
<li>fudge_inference_if_ok :&gt;</li>
<li>diverging block handling :3</li>
<li>fallback :3</li>
</ul>
</li>
<li>MIR borrowck
<ul>
<li>MIR typeck
<ul>
<li>why do we want two typecks</li>
<li>region dependent goals in new solver (interaction with lack-of region uniquification)</li>
</ul>
</li>
<li>overlaps with opaque types</li>
<li>compute region graph</li>
<li>closure requirements</li>
<li>borrowck proper</li>
</ul>
</li>
<li>compare predicate entailment :&gt;
<ul>
<li>param env jank</li>
<li>implied bounds handling</li>
</ul>
</li>
<li>trait objects: recent FCPs :3
<ul>
<li>dyn compatibility soundness interactions (see coerce pointee/arbitrary self types stuff)</li>
<li>dyn compatibility for impl reasons (monomorphization)</li>
<li>projection bounds handling</li>
<li>args not required for wf</li>
</ul>
</li>
<li><code>ty::Infer</code> in <code>ty</code> overview</li>
<li>generalization</li>
<li>coroutines
<ul>
<li>deferred coroutine obligations</li>
<li>witness types?</li>
<li>why <code>-Zhigher-ranked-assumptions</code> exists</li>
</ul>
</li>
<li>binders and universes <code>existsA forallB A == B</code>
<ul>
<li>build more of an intuition than current docs :thinking_face:</li>
</ul>
</li>
<li>talk about hr implied bounds there/be more explicit/clear in https://rustc-dev-guide.rust-lang.org/traits/implied-bounds.html?highlight=implied#proving-implicit-implied-bounds</li>
<li>incompleteness
<ul>
<li>what is it</li>
<li>what kinds are OK (not entirely sure yet. small explanation and add a note)</li>
</ul>
</li>
<li>trait solving
<ul>
<li>cycles</li>
<li>general overview of how trait solving works as a concept (probably with example and handwritten proof trees)
<ul>
<li>important: first go &quot;prove stuff by recursively proving nested requirements&quot;, then later introduce candidates</li>
<li>clauses/predicates</li>
</ul>
</li>
<li>running pending goals in a loop</li>
<li>what kinds of incompleteness (overlap with opaques)</li>
<li>builtin impls and how to add them</li>
</ul>
</li>
<li>hir to ty lowering :&gt;
<ul>
<li>itemctxt vs fnctxt behaviours</li>
<li>normalization in lowering</li>
<li>lowering should be lossy</li>
<li>idempotency(?)</li>
<li>cycles from param env construction</li>
<li>const generics jank about Self and no generic parameters allowed</li>
</ul>
</li>
<li>well formedness checking + wf disambiguation page</li>
<li>normalization &amp; aliases
<ul>
<li>be more clear about normalizing ambig aliases to infer vars :thinking_face:</li>
<li>normalize when equating infer vars with aliases (overlap with generalization?)</li>
<li>item bounds checking</li>
<li>interactions with implied bounds (overlap with implied bounds and hir ty lowering)</li>
</ul>
</li>
<li>variance</li>
</ul>
<p>Since making this list I've started working on writing documentation about coercions/adjustments. So far this has mostly resulted in spending a lot of time reading the relevant code in rustc. I've discovered a few bugs and inconsistencies in behaviour and made some nice code cleanups which should be valuable for people learning how coercions are implemented already. This can be seen in #147565</p>
<p>I intend to start actually writing stuff in the dev guide for coercions/adjustments now as that PR is almost done.</p>
<p>I also intend to use a zulip thread (<a href="https://rust-lang.zulipchat.com/#narrow/channel/196385-t-compiler.2Frustc-dev-guide/topic/Type.20System.20Docs.20Rewrite/with/543632377">#t-compiler/rustc-dev-guide &gt; Type System Docs Rewrite</a>) for more &quot;lightweight&quot; and informal updates on this project goal, as well as just miscellaneous discussion about work relating to this project goal</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/405#issuecomment-3591757405" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BoxyUwU posted on 2025-11-29:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>I've made a tracking issue on the dev guide repo for this project goal: rust-lang/rustc-dev-guide#2663. I've also written documentation for coercions: rust-lang/rustc-dev-guide#2662. There have been a few extra additions to the list in the previous update.</p>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Unsafe Fields <a href='https://github.com/rust-lang/rust-project-goals/issues/273' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#273)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--body-background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="1" max="13"></progress>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Point of contact</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/jswrenn">Jack Wrenn</a></p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Champions</td>
<td style="padding: 8px 16px;"><p><a href="http://github.com/rust-lang/compiler-team">compiler</a> (<a href="https://github.com/jswrenn">Jack Wrenn</a>), <a href="http://github.com/rust-lang/lang-team">lang</a> (<a href="https://github.com/scottmcm">Scott McMurray</a>)</p>
</td>
</tr>
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; color: #666;">Task owners</td>
<td style="padding: 8px 16px;"><p><a href="https://github.com/jhpratt">Jacob Pratt</a>, <a href="https://github.com/jswrenn">Jack Wrenn</a>, <a href="https://github.com/veluca93">Luca Versari</a></p>
</td>
</tr>
</table>

<!-- TL;DR Section -->

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
