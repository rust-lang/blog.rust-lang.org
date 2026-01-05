+++
path = "9999/12/31/project-goals-2025-december-update"
title = "Project-Goals-2025-December-Update"
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
<a href="https://github.com/rust-lang/rust-project-goals/issues/389#issuecomment-3667723344" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @frank-king posted on 2025-12-18:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<ul>
<li>
<p><strong>Key developments</strong>: <a href="https://github.com/rust-lang/rust/pull/149263">forbid manual impl of <code>Unpin</code> for <code>#[pin_v2]</code> types</a>.</p>
</li>
<li>
<p><strong>Blockers</strong>: PRs waiting for review:</p>
<ul>
<li><a href="https://github.com/rust-lang/rust/pull/144537">impl <code>Drop::pin_drop</code></a> (the submodule issue)</li>
<li><a href="https://github.com/rust-lang/rust/pull/149130">coercion of <code>&amp;pin mut|const T</code> &lt;-&gt; <code>&amp;[mut] T</code></a></li>
</ul>
</li>
<li>
<p><strong>Help wanted</strong>: None yet.</p>
</li>
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

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>5 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3621913656" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-12-07:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Since we have chosen virtual places as the new approach, we reviewed what open questions are most pressing for the design. Our discussion resulted in the following five questions:</p>
<ol>
<li>Should we have 1-level projections xor multi-level projections?</li>
<li>What is the semantic meaning of the borrow checker rules (<code>BorrowKind</code>)?</li>
<li>How should we add &quot;canonical projections&quot; for types such that we have nice and short syntax (like <code>x~y</code> or <code>x.@y</code>)?</li>
<li>What to do about non-indirected containers (Cell, MaybeUninit, Mutex, etc)?</li>
<li>How does one inspect/query <code>Projection</code> types?</li>
</ol>
<p>We will focus on these questions in December as well as implementing FRTs.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3644702112" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-12-12:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h2>Canonical Projections</h2>
<p>We have discussed canonical projections and come up with the following solution:</p>


<pre><code class="language-rust">pub trait CanonicalReborrow: HasPlace {
    type Output&lt;'a, P: Projection&lt;Source = Self::Target&gt;&gt;: HasPlace&lt;Target = P::Target&gt;
    where
        Self: PlaceBorrow&lt;'a, P, Self::Output&lt;'a, P&gt;&gt;;
}
</code></pre>
<p>Implementing this trait permits using the syntax <code>@$place_expr</code> where the place's origin is of the type <code>Self</code> (for example <code>@x.y</code> where <code>x: Self</code> and <code>y</code> is an identifier or tuple index, or <code>@x.y.z</code> etc). It is desugared to be:</p>


<pre><code class="language-rust">@&lt;&lt;Self as CanonicalReborrow&gt;::Output&lt;'_, projection_from_place_expr!($place_expr)&gt;&gt; $place_expr
</code></pre>
<p>(The names of the trait, associated type and syntax are not final, better suggestions welcome.)</p>
<h3>Reasoning</h3>
<ul>
<li>We need the <code>Output</code> associated type to support the <code>@x.y</code> syntax for <code>Arc</code> and <code>ArcRef</code>.</li>
<li>We put the FRT and lifetime parameter on <code>Output</code> in order to force implementers to always provide a canonical reborrow, so if <code>@x.a</code> works, then <code>@x.b</code> also works (when <code>b</code> also is a field of the struct contained by <code>x</code>).
<ul>
<li>This (sadly or luckily) also has the effect that making <code>@x.a</code> and <code>@x.b</code> return different wrapper types is more difficult to implement and requires a fair bit of trait dancing. We should think about discouraging this in the documentation.</li>
</ul>
</li>
</ul>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3659055067" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-12-16:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h2>Non-Indirected Containers</h2>
<p>Types like <code>MaybeUninit&lt;T&gt;</code>, <code>Cell&lt;T&gt;</code>, <code>ManuallyDrop&lt;T&gt;</code>, <code>RefCell&lt;T&gt;</code> etc. currently do not fit into our virtual places model, since they don't have an indirection. They contain the place directly inline (and some are even <code>repr(transparent)</code>). For this reason, we currently don't have projections available for <code>&amp;mut MaybeUninit&lt;T&gt;</code>.</p>
<p>Enter our new trait <code>PlaceWrapper</code> which these types implement in order to make projections available for them. We call these types <em>place wrappers</em>. Here is the definition of the trait:</p>


<pre><code class="language-rust">pub unsafe trait PlaceWrapper&lt;P: Projection&lt;Source = Self::Target&gt;&gt;: HasPlace {
    type WrappedProjection: Projection&lt;Source = Self&gt;;

    fn wrap_projection(p: P) -&gt; Self::WrappedProjection;
}
</code></pre>
<p>This trait should only be implemented when <code>Self</code> doesn't contain the place as an indirection (so for example <code>Box</code> must not implement the trait). When this trait is implemented, then <code>Self</code> has &quot;virtual fields&quot; available (actually all kinds of place projections). The name of these virtual fields/projections is the same as the ones of the contained place. But their output type is controlled by this trait.</p>
<p>As an example, here is the implementation for <code>MaybeUninit</code>:</p>


<pre><code class="language-rust">impl&lt;T, P: Projection&lt;Source = T&gt;&gt; PlaceWrapper&lt;P&gt; for MaybeUninit&lt;T&gt; {
    type WrappedProjection = TransparentProjection&lt;P, MaybeUninit&lt;T&gt;, MaybeUninit&lt;P::Target&gt;&gt;;

    fn wrap_projection(p: P) -&gt; Self::WrappedProjection {
        TransparentProjection(p, PhantomData, PhantomData)
    }
}
</code></pre>
<p>Where <code>TransparentProjection</code> will be available in the standard library defined as:</p>


<pre><code class="language-rust">pub struct TransparentProjection&lt;P, Src, Tgt&gt;(P, PhantomData&lt;Src&gt;, PhantomData&lt;Tgt&gt;);

impl&lt;P: Projection, Src, Tgt&gt; Projection for TransparentProjection&lt;P, Src, Tgt&gt; {
    type Source = Src;
    type Target = Tgt;

    fn offset(&amp;self) -&gt; usize {
        self.0.offset()
    }
}
</code></pre>
<p>When there is ambiguity, because the wrapper and the wrapped types both have the same field, the wrapper's field takes precedence (this is the same as it currently works for <code>Deref</code>). It is still possible to refer to the wrapped field by first dereferencing the container, so <code>x.field</code> refers to the wrapper's <code>field</code> and <code>(*x).field</code> refers to the field of the wrapped type.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3677624396" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-12-20:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h2>Field-by-Field Projections vs One-Shot Projections</h2>
<p>We have used several different names for these two ways of implementing projections. The first is also called 1-level projections and the second multi-level projections.</p>
<p>The field-by-field approach uses field representing types (FRTs), which represent a single field of a struct with no indirection. When writing something like <code>@x.y.z</code>, we perform the place operation twice, first using the FRT <code>field_of!(X, y)</code> and then again with <code>field_of!(T, z)</code> where <code>T</code> is the resulting type of the first projection.</p>
<p>The second approach called one-shot projections instead extends FRTs with <em>projections</em>, these are compositions of FRTs, can be empty and dynamic. Using these we desugar <code>@x.y.z</code> to a single place operation.</p>
<p>Field-by-field projections have the advantage that they simplify the implementation for users of the feature, the compiler implementation and the mental model that people will have to keep in mind when interacting with field projections. However, they also have pretty big downsides, which either are fundamental to their design or would require significant complification of the feature:</p>
<ul>
<li>They have less expressiveness than one-shot projections. For example, when moving out a subsubfield of <code>x: &amp;own Struct</code> by doing <code>let a = @x.field.a</code>, we have to move out <code>field</code>, which prevents us from later writing <code>let b = @x.field.b</code>. One-shot projections allow us to track individual subsubfields with the borrow checker.</li>
<li>Field-by-field projections also make it difficult to define type-changing projections in an inference friendly way. Projecting through multiple fields could result in several changes of types in between, so we would have to require only canonical projections in certain places. However, this requires certain intermediate types for which defining their safety invariants is very complex.</li>
</ul>
<p>We additionally note that the single function call desugaring is also a simplification that also lends itself much better when explaining what the <code>@</code> syntax does.</p>
<p>All of this points in the direction of proceeding with one-shot projections and we will most likely do that. However, we must note that the field-by-field approach might yield easier trait definitions that make implementing the various place operations more manageable. There are several open issues on how to design the field-by-field API in the place variation (the previous proposal did have this mapped out clearly, but it does not translate very well to places), which would require significant effort to solve. So at this point we cannot really give a fair comparison. Our initial scouting of the solutions revealed that they all have some sort of limitation (as we explained above for intermediate projection types for example), which make field-by-field projections less desirable. So for the moment, we are set on one-shot projections, but when the time comes to write the RFC we need to revisit the idea of field-by-field projections.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3690808729" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-12-25:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h2>Wiki Project</h2>
<p>We started a wiki project at https://rust-lang.github.io/beyond-refs to map out the solution space. We intend to grow it into the single source of truth for the current state of the field projection proposal as well as unfinished and obsolete ideas and connections between them. Additionally, we will aim to add the same kind of information for the in-place initialization effort, since it has overlap with field projections and, more importantly, has a similarly large solution space.</p>
<p>In the beginning you might find many stub pages in the wiki, which we will work on making more complete. We will also mark pages that contain old or abandoned ideas as such as well as mark the current proposal.</p>
<p>This issue will continue to receive regular detailed updates, which are designed for those keeping reasonably up-to-date with the feature. For anyone out of the loop, the wiki project will be a much better place when it contains more content.</p>

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
<a href="https://github.com/rust-lang/rust-project-goals/issues/399#issuecomment-3665883649" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @aapoalas posted on 2025-12-17:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h2>Purpose</h2>
<p>A refresher on what we want to achieve here: the most basic form of reborrowing we want to enable is this:</p>


<pre><code class="language-rust">// Note: not Clone or Copy
#[derive(Reborrow)]
struct MyMutMarker&lt;'a&gt;(...);

// ...

let marker: MyMarkerMut = MyMutMarker::new();
some_call(marker);
some_call(marker);
</code></pre>
<p>ie. make it possible for an owned value to be passed into a call twice and have Rust inject a reborrow at each call site to produce a new bitwise copy of the original value for the passing purposes, and mark the original value as disabled for reads and writes for the duration of the borrow.</p>
<p>A notable complication appears with implementing such reborrowing in userland using explicit cals when dealing with returned values:</p>


<pre><code class="language-rust">return some_call(marker.reborrow());
</code></pre>
<p>If the borrowed lifetime escapes through the return value, then this will not compile as the borrowed lifetime is based on a value local to this function. Alongside convenience, this is the major reason for the Reborrow traits work.</p>
<p><code>CoerceShared</code> is a secondary trait that enables equivalent reborrowing that only disables the original value for writes, ie. matching the <code>&amp;mut T</code> to <code>&amp;T</code> coercion.</p>
<h2>Update</h2>
<p>We have the <code>Reborrow</code> trait working, albeit currently with a bug in which the <code>marker</code> must be bound as <code>let mut</code>. We are working towards a working <code>CoerceShared</code> trait in the following form:</p>


<pre><code class="language-rust">trait CoerceShared&lt;Target: Copy&gt; {}
</code></pre>
<p>Originally the trait had a <code>type Target</code> ADT but this turned out to be unnecessary, as there is no reason to particularly disallow multiple coercion targets. The original reason for using an ADT to disallow multiple coercion targets was based on the trait also having an unsafe method, at which point unscrupulous users could use the trait as a generic coercion trait. Because the trait method was found to be unnecessary, the fear is also unnecessary.</p>
<p>This means that the trait has better chances of working with multiple coercing lifetimes (think a collection of <code>&amp;mut</code>s all coercing to <code>&amp;</code>s, or only some of them). However, we are currently avoiding any support of multiple lifetimes as we want to avoid dealing with rmeta before we have the basic functionality working.</p>

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
<a href="https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-3655862823" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @davidtwco posted on 2025-12-15:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>rust-lang/rfcs#3873 is waiting on one checkbox before entering the final comment period. We had our sync meeting on the 11th and decided that we would enter FCP on rust-lang/rfcs#3874 and rust-lang/rfcs#3875 after rust-lang/rfcs#3873 is accepted. We've responded to almost all of the feedback on the next two RFCs and expect the FCP to act as a forcing-function so that the relevant teams take a look, they can always register concerns if there are things we need to address, and if we need to make any major changes then we'll restart the FCP.</p>

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
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/397#issuecomment-3597627406" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @folkertdev posted on 2025-12-01:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>We did not receive the funding we needed to work on this goal, so no progress has been made.</p>
<p>Overall I think the improvements we felt comfortable promising are on the low side. Overall the amount of time spent in codegen for realistic changes to real code bases was smaller than expected, meaning that the improvements that cranelift can deliver for the end-user experience are smaller.</p>
<p>We still believe larger gains can be made with more effort, but did not feel confident in promising hard numbers.</p>
<p>So for now, let's close this.</p>

</div>
</div>
</div>
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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
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
<a href="https://github.com/rust-lang/rust-project-goals/issues/119#issuecomment-3656328086" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @epage posted on 2025-12-15:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Key developments</p>
<ul>
<li>A fence length limit was added in response to T-lang feedback (https://github.com/rust-lang/rust/pull/149358)</li>
<li>Whether to disallow or lint for CR inside of a frontmatter is under discussion (https://github.com/rust-lang/rust/pull/149823)</li>
</ul>
<p>Blockers</p>
<ul>
<li>https://github.com/rust-lang/rust/pull/146377</li>
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
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/393#issuecomment-3666126619" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @cramertj posted on 2025-12-17:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Current status:</p>
<ul>
<li>The <a href="https://github.com/rust-lang/rfcs/pull/3851">RFC for <code>auto impl</code> supertraits</a> has been updated to address SemVer compatibility issues.</li>
<li>There is a <a href="https://github.com/rust-lang/rust/pull/149335">parsing PR</a> kicking off an experimental implementation. The tracking issue for this experimental implementation is <a href="https://github.com/rust-lang/rust/issues/149556">here</a>.</li>
</ul>

</div>
</div>
</div>
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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
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
<a href="https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-3655904366" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @lcnr posted on 2025-12-15:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>We've continued to fix a bunch of smaller issues over the last month. <a href="https://github.com/theemathas">Tim (Theemathas Chirananthavat)</a> helped uncover a new potential issue due to non-fatal overflow which we'll have to consider before stabilizing the new solver: https://github.com/rust-lang/trait-system-refactor-initiative/issues/258.</p>
<p>I fixed two issues myself in https://github.com/rust-lang/rust/pull/148823 and https://github.com/rust-lang/rust/pull/148865.</p>
<p><a href="https://github.com/tiif">tiif</a> with help by <a href="https://github.com/BoxyUwU">Boxy</a> fixed query cycles when evaluating constants in where-clauses: https://github.com/rust-lang/rust/pull/148698.</p>
<p>@adwinwhite fixed a subtle issues involving coroutine witnesses in https://github.com/rust-lang/rust/pull/149167 after having diagnosed the underlying issue there last month. They've also fixed a smaller diagnostics issue in https://github.com/rust-lang/rust/pull/149299. Finally, they've also fixed an edge case of impl well-formedness checking in https://github.com/rust-lang/rust/pull/149345.</p>
<p><a href="https://github.com/ShoyuVanilla">Shoyu Vanilla</a> fixed a broken interaction of aliases and fudging in https://github.com/rust-lang/rust/pull/149320. Looking into fudging and HIR typeck <code>Expectation</code> handling also uncovered a bunch of broken edge-cases and I've openedhttps://github.com/rust-lang/rust/issues/149379 to track these separately.</p>
<p>I have recently spent some time thinking about the remaining necessary work and posted a write-up on my personal blog: https://lcnr.de/blog/2025/12/01/next-solver-update.html. I am currently trying to get a clearer perspective on our cycle handling while slowly working towards an RFC for the changes there. This is challenging as we don't have a good theoretical foundation here yet.</p>

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
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-3699751576" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @lqd posted on 2025-12-30:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>This month's key developments were:</p>
<ul>
<li>borrowck support in <code>a-mir-formality</code> has been progressing steadily — it has its own dedicated updates in https://github.com/rust-lang/rust-project-goals/issues/122 for more details</li>
<li>we were also able to find a suitable project for the master's student project on a-mir-formality (and they accepted and should start around February) and which will help expand our testing coverage for the polonius alpha as well.</li>
<li>tiif has kept making progress on fixing opaque type soundness issue https://github.com/rust-lang/trait-system-refactor-initiative/issues/159. It is the one remaining blocker for passing all tests. By itself it will not immediately fix the two remaining (soundness) issues with opaque type region liveness, but we'll able to use the same supporting code to ensure the regions are indeed live where they need to be.</li>
<li>I quickly cleaned up some <a href="https://github.com/rust-lang/rust/pull/149639">inefficiencies in constraint conversion</a>, it hasn't landed yet but it maybe won't need to because of the next item</li>
<li>but most of the time this month was spent on this final item: we have the first interesting results from the rewriting effort. After a handful of wrong starts, I have a branch almost ready to switch the constraint graph to be lazy and computed during traversal. It removes the need to index the numerous list of constraints, or to convert liveness data to a different shape. It thus greatly reduces the current alpha overhead (some rare cases look faster than NLLs but I don't yet know why, maybe due to being able to better use the sparseness, low connectivity of the constraint graph, and a small number of loans). The overhead wasn't entirely removed of course: the worst offending benchmark has a +5% wall-time regression, but icounts are worse looking (+13%). This was also only benchmarking the algorithm itself, without the improvements to the rest of borrowck mentioned in previous updates. I should be able to open a PR in the next couple days, once I figure out how to best convert the polonius mermaid graph dump to the new lazy localized constraint generation.</li>
<li>and finally, happy holidays everyone!</li>
</ul>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-3702561875" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @lqd posted on 2025-12-31:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<blockquote>
<ul>
<li>I should be able to open a PR in the next couple days</li>
</ul>
</blockquote>
<p>done in https://github.com/rust-lang/rust/pull/150551</p>

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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
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
<span style="font-weight: bold;"><p>4 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/122#issuecomment-3604453619" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-12-03:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>PR https://github.com/rust-lang/a-mir-formality/pull/206 contains a &quot;first draft&quot; for the NLL rules. It checks for loan violations (e.g., mutating borrowed data) as well as some notion of outlives requirements. It does not check for move errors and there aren't a lot of tests yet.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/122#issuecomment-3604455122" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-12-03:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>The PR also includes two big improvements to the a-mir-formality framework:</p>
<ul>
<li>support for <code>(for_all)</code> rules that can handle &quot;iteration&quot;</li>
<li>tracking proof trees, making it <em>much</em> easier to tell why something is accepted that should not be</li>
</ul>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/122#issuecomment-3637518738" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-12-10:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update: opened https://github.com/rust-lang/a-mir-formality/pull/207 which contains support for <code>&amp;mut</code>, wrote some new tests (including one FIXME), and added a test for NLL Problem Case #3 (which behaved as expected).</p>
<p>One interesting thing (cc <a href="https://github.com/RalfJung">Ralf Jung</a>) is that we have diverged from <a href="https://github.com/minirust/minirust/">MiniRust</a> in a few minor ways:</p>
<ul>
<li>We do not support embedding value expressions in place expressions.</li>
<li>Where MiniRust has a <code>AddrOf</code> operator that uses the <code>PtrType</code> to decide what kind of operation it is, we have added a <code>Ref</code> MIR operation. This is in part because we need information that is not present in MiniRust, specifically a lifetime.</li>
<li>We have also opted to extend <code>goto</code> with the ability to take multiple successors, so that <code>goto b1, b2</code> can be seen as &quot;goto either b1 or b2 non-deterministically&quot; (the actual opsem would probably be to always go to b1, making this a way to add &quot;fake edges&quot;, but the analysis should not assume that).</li>
</ul>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/122#issuecomment-3666177635" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-12-17:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update: opened https://github.com/rust-lang/a-mir-formality/pull/210 with today's work. We are discussing how to move the checker to support polonius-alpha. To that end, we introduced feature gates (so that a-mir-formality can model nightly features) and did some refactoring of the type checker aiming at allowing outlives to become flow-sensitive.</p>

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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
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
<td style="padding: 8px 16px;"><progress value="2" max="7"></progress>
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
<span style="font-weight: bold;"><p>3 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3698333757" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BoxyUwU posted on 2025-12-30:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Since the last update both of my PRs I mentioned have landed, allowing for constructing ADTs in const arguments while making use of generic parameters. This makes MGCA effectively a &quot;full&quot; prototype where it can now fully demonstrate the core concept of the feature. There's still a lot of work left to do but now we're at the point of finishing out the feature :)</p>
<p>Once again huge thanks to camelid for sticking with me throughout this. Also thanks to errs, oli and lcnr for reviewing some of the work and chatting with me about possible impl decisions.</p>
<p>Some examples of what is possible with MGCA as of the end of this goal cycle:</p>


<pre><code class="language-rust">#![feature(const_default, const_trait_impl, min_generic_const_args)]

trait Trait {
    #[type_const]
    const ASSOC: usize;
}

fn mk_array&lt;T: const Default + Trait&gt;() -&gt; [T; T::ASSOC] {
    [const { T::default() }; _]
}
</code></pre>


<pre><code class="language-rust">#![feature(adt_const_params, min_generic_const_args)]

fn foo&lt;const N: Option&lt;u32&gt;&gt;() {}

trait Trait {
    #[type_const]
    const ASSOC: usize;
}

fn bar&lt;T: Trait, const N: u32&gt;() {
    // the initializer of `_0` is a `N` which is a legal const argument
    // so this is ok.
    foo::&lt;{ Some::&lt;u32&gt; { 0: N } }&gt;();

    // this is allowed as mgca supports uses of assoc consts in the
    // type system. ie `&lt;T as Trait&gt;::ASSOC` is a legal const argument
    foo::&lt;{ Some::&lt;u32&gt; { 0: &lt;T as Trait&gt;::ASSOC } }&gt;();

    // this on the other hand is not allowed as `N + 1` is not a legal
    // const argument
    foo::&lt;{ Some::&lt;u32&gt; { 0: N + 1 } }&gt;(); // ERROR
}
</code></pre>
<p>As for <code>adt_const_params</code> we now have a zulip stream specifically for discussion of the upcoming RFC and the drafting of the RFC: <a href="https://rust-lang.zulipchat.com/#narrow/channel/551659-project-const-generics.2Fadt_const_params-rfc">#project-const-generics/adt_const_params-rfc</a>. I've gotten part of the way through actually writing the RFC itself though it's gone slower than I had originally hoped as I've also been spending more time thinking through the implications of allowing private data in const generics.</p>
<p>I've debugged the remaining two ICEs making <code>adt_const_params</code> not fully ready for stabilization and written some brief instructions on how to resolve them. One ICE has been incidentally fixed (though more <em>masked</em>) by some work that <a href="https://github.com/Kivooeo">Kivooeo</a> has been doing on MGCA. The other has been picked up by someone I'm not sure the github handle of so that will also be getting fixed soon.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3698351163" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BoxyUwU posted on 2025-12-30:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Ah I forgot to mention, even though MGCA has a tonne of work left to do I expect it should be somewhat approachable for people to help out with. So if people are interested in getting involved now is a good time :)</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3700029422" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BoxyUwU posted on 2025-12-30:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Ah another thing I forgot to mention. <a href="https://github.com/davidtwco">David Wood</a> spent some time looking into the name mangling scheme for <code>adt_const_params</code> stuff to make sure it would be fine to stabilize and it seems it is so that's another step closer to <code>adt_const_params</code> being stabilizable</p>

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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
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
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/391#issuecomment-3662260597" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @PLeVasseur posted on 2025-12-16:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Meeting notes here: <a href="https://github.com/rust-lang/fls-team/blob/main/meetings/2025-12-12.md">FLS team meeting 2025-12-12</a></p>
<p><strong>Key developments</strong>: We're close to completing the FLS release for 1.91.0, 1.91.1. We've started to operate as a team, merging a <a href="https://github.com/rust-lang/fls/pull/621">PR</a> with the changelog entries, then opening up issues for each change required: ✅ <a href="https://github.com/rust-lang/rust/issues/624">#624</a>(https://github.com/rust-lang/fls/issues/624), ✅ <a href="https://github.com/rust-lang/rust/issues/625">#625</a>(https://github.com/rust-lang/fls/issues/625), ✅ <a href="https://github.com/rust-lang/rust/issues/626">#626</a>(https://github.com/rust-lang/fls/issues/626), ⚠️ <a href="https://github.com/rust-lang/rust/issues/623">#623</a>(https://github.com/rust-lang/fls/issues/623). <a href="https://github.com/rust-lang/rust/issues/623">#623</a>(https://github.com/rust-lang/fls/issues/623) is still pending, as it requires a bit of alignment with the Reference on definitions and creation of a new example.
<strong>Blockers</strong>: None currently
<strong>Help wanted</strong>: We'd love more folks from the safety-critical community to contribute to picking up <a href="https://github.com/rust-lang/fls/issues">issues</a> or opening an issue if you notice something is missing.</p>

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
<a href="https://github.com/rust-lang/rust-project-goals/issues/392#issuecomment-3661541321" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @icmccorm posted on 2025-12-16:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Here's our December status update!</p>
<ul>
<li>
<p>We have revised <a href="https://github.com/BorrowSanitizer/rust/commit/c0f33b5503f7d972f0359ba0cefa5d7f819deac2">our prototype</a> of the pre-RFC based on <a href="https://github.com/RalfJung">Ralf Jung</a>'s feedback. Now, instead of having two different retag functions for operands and places, we emit a single <code>__rust_retag</code> intrinsic in every situation. We also track interior mutability precisely. At this point, the implementation is mostly stable and seems to be ready for an MCP.</p>
</li>
<li>
<p>There's been some discussion <a href="https://github.com/rust-lang/unsafe-code-guidelines/issues/371">here</a> and in the <a href="https://internals.rust-lang.org/t/pre-rfc-emit-retags-in-codegen/23706">pre-RFC</a> about whether or not Rust will still have explicit MIR retag statements. We plan on revising our implementation so that we no longer rely on MIR retags to determine where to insert our lower-level retag calls. This should be a relatively straightforward change to the current prototype. If anything, it should make these changes easier to merge upstream, since they will no longer affect Miri.</p>
</li>
<li>
<p>BorrowSanitizer continues to gain new features, and we've started testing it on our first real crate (<a href="https://docs.rs/lru/latest/lru/">lru</a>) (which has uncovered a few new bugs in our implementation). The two core Tree Borrows features that we have left to support are error reporting and <a href="https://github.com/rust-lang/miri/pull/2479">garbage collection</a>. Once these are finished, we will be able to expand our testing to more real-world libraries and confirm that we are passing each of Miri's test cases (and likely find more bugs lurking in our implementation). Our instrumentation pass ignores global and thread-local state for now, and it does not support atomic memory accesses outside of atomic <code>load</code> and <code>store</code> instructions. These operations should be relatively straightforward to add once we've finished higher-priority items.</p>
</li>
<li>
<p>Performance is slow. We do not know exactly how slow yet, since we've been focusing on feature support over benchmarking and optimization. This is at least partially due to the lack of garbage collection, based on what we're seeing from profiling. We will have a better sense of what our performance is like once we can compare against Miri on more real-world test cases.</p>
</li>
</ul>
<p>As for what's next, we plan on posting an MCP soon, now that it's clear that we will be able to do without MIR retags. You can expect a more detailed status update on BorrowSanitizer by the end of January. This will discuss our implementation and plans for 2026. We will post that here and on our <a href="https://borrowsanitizer.com/">project website</a>.</p>

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
<a href="https://github.com/rust-lang/rust-project-goals/issues/394#issuecomment-3666356167" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @joshtriplett posted on 2025-12-17:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>In addition to further ongoing work on reference material (some of which is on track to be merged), we've had some extensive discussions about reference processes, maintenance, and stability markers. <a href="https://github.com/nikomatsakis">Niko Matsakis</a> is putting together a summary and proposal for next steps.</p>

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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
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
<td style="padding: 8px 16px;"><progress value="5" max="8"></progress>
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
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-3603917236" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @ZuseZ4 posted on 2025-12-02:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>It's only been two weeks, but we got a good number of updates, so I already wanted to share them.</p>
<h2>autodiff</h2>
<ol>
<li>On the autodiff side, we landed the support for rlib and better docs. This means that our autodiff frontend is &quot;almost&quot; complete, since there are almost no cases left where you can't apply autodiff. There are a few features like custom-derivatives or support for <code>dyn</code> arguments that I'd like to add, but they are currently waiting for better docs on the Enzyme side. There is also a long-term goal off replacing the fat-lto requirement with the less invasive embed-bc requirement, but this proved to be tricky in the past and only affects compile times.</li>
<li>@sgasho picked up my old PR to dlopen enzyme, and found the culprit of it failing after my last rebase. A proper fix might take a bit longer, but it might be worth waiting for. As a reminder, using dlopen in the future allows us to ship autodiff on nightly without increasing the size of rustc and therefore without making our infra team sad.</li>
</ol>
<p>All in all, we have landed most of the hard work here, so that's a very comfortable position to be in before enabling it on nightly.</p>
<h2>offload</h2>
<ol>
<li>We have landed the intrinsic implementation of <a href="https://github.com/Sa4dUs">Marcelo Domínguez</a>, so now you can offload functions with almost arbitrary arguments. In my first prototype, I had limited it to pointers to 256 f64 values. The updated usage example continues to live <a href="https://rustc-dev-guide.rust-lang.org/offload/usage.html#usage">here</a> in our docs. As you can see, we still require <code>#[cfg(target_os=X)]</code> annotations. Under the hood, the LLVM-IR which we generate is also still a bit convoluted. In his next PRs, he'll clean up the generated IR, and introduce an offload macro that users shall call instead of the internal offload intrinsic.</li>
<li>I spend more time on <a href="https://rustc-dev-guide.rust-lang.org/offload/usage.html#compile-instructions">enabling offload in our CI</a>, to enable <code>std::offload</code> in nightly. After multiple iterations and support from LLVM offload devs, we found a cmake config that does not run into bugs, should not increase Rust CI time too much, and works with both in-tree llvm/clang builds, as well as external clang's (the current case in our Rust CI).</li>
<li>I spend more time on simplifying the usage instructions in <a href="https://rustc-dev-guide.rust-lang.org/offload/usage.html#compile-instructions">the dev guide</a>. We started with two cargo calls, one rustc call, two clang calls, and two clang-helper binary calls. I was able to remove the rustc and one of the clang-offload-packager calls, by directly calling the underlying LLVM APIs. I also have an unmerged PR which removes the two clang calls. Once I cleaned it up and landed it, we would be down to only two cargo calls and one binary call to <code>clang-linker-wrapper</code>. Once I automated this last wrapper (and enabled offload in CI), nightly users should be able to experiment with <code>std::offload</code>.</li>
</ol>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-3692957114" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @ZuseZ4 posted on 2025-12-26:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Time for the next round of updates. Again, most of the updates were on the GPU side, but with some notable autodiff improvements too.</p>
<h3>autodiff:</h3>
<ol>
<li>
<p>@sgasho finished his work on using dlopen to load enzyme and the pr landed. This allowed <a href="https://github.com/kobzol">Jakub Beránek</a> and me to start working on distributing Enzyme via a standalone component.</p>
</li>
<li>
<p>As a first step, I added a <a href="https://github.com/rust-lang/rust/pull/150060">nicer error</a> if we fail to find or dlopen our Enzyme backend. I also removed most of our autodiff fallbacks, we now unconditionally enable our macro frontend on nightly: https://github.com/rust-lang/rust/pull/150133
<strong>You may notice that <code>cargo expand</code> now works on autodiff code.</strong> This also allowed the first bug reports about ICE (internal compiler error) in our macro parser logic.</p>
</li>
<li>
<p>Kobzol opened a PR to build Enzyme in CI. In theory, I should have been able to download that artifact, put it into my sysroot, and use the latest nightly to automatically load it. If that had worked, we could have just merged his PR, and everyone could have started using AD on nightly. Of course, things are never that easy. Even though both Enzyme, LLVM, and rustc were built in CI, the LLVM version shipped along with rustc does not seem compatible with the LLVM version Enzyme was built against. We assume some slight cmake mismatch during our CI builds, which we will have to debug.</p>
</li>
</ol>
<h3>offload:</h3>
<ol>
<li>
<p>On the gpu side, <a href="https://github.com/sa4dus">Marcelo Domínguez</a> finished his cleanup <a href="https://github.com/rust-lang/rust/pull/149788">PR</a>, and along the way also fixed using multiple kernels within a single codebase. When developing the offload MVP I had taken a lot of inspiration from the LLVM-IR generated by clang - and it looks like I had gotten one of the (way too many) LLVM attributes wrong. That caused some metadata to be fused when multiple kernels are present, confusing our offload backend. We started to find more bugs when working on benchmarks, more about the fixes for those in the next update.</p>
</li>
<li>
<p>I finished cleaning up my offload build PR, and <a href="https://github.com/oli-obk">Oliver Scherer</a> reviewed and approved it. Once the dev-guide gets synced, you should see much simpler usage instructions. Now it's just up to me to automate the <a href="https://github.com/rust-lang/rust/pull/149827">last part</a>, then you can compile offload code purely with cargo or rustc.
I also improved how we <a href="https://github.com/rust-lang/rust/pull/150108">build offload</a>, which allows us to build it both in CI and locally. CI had some very specific requirements to not increase build times, since our x86-64-dist runner is already quite slow.</p>
</li>
<li>
<p>Our first benchmarks directly linked against NVIDIA and AMD intrinsics on llvm-ir level. However, we already had an nvptx Rust module for a while, and since recently also an <a href="https://github.com/rust-lang/stdarch/pull/1976">amdgpu</a> module which nicely wraps those intrinsics. I just synced the stdarch repository into rustc a few minutes ago, so from now on, we can replace both with the corresponding Rust functions. In the near future we should get a higher level GPU module, which abstracts away naming differences between vendors.</p>
</li>
<li>
<p>Most of my past rustc contributions were related to LLVM projects or plugins (Offload and Enzyme), and I increasingly encountered myself asking other people for updates or backports of our LLVM submodule, since upstream LLVM has fixes which were not yet merged into our LLVM submodule. Our llvm working group is quite small and I didn't want to burden them too much with my requests, so I recently asked them to join it, which also got approved. In the future I intend to help a little with the maintenance here.</p>
</li>
</ol>

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
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/407#issuecomment-3616309374" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-12-05:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update from the 2025-12-03 meeting:</p>
<h2><a href="https://github.com/rust-lang/rust/pull/136597"><code>-Zharden-sls</code></a></h2>
<p>Wesley reviewed it again, provided a qualification, more changes requested.</p>

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
<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3617147228" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-12-05:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update from the 2025-12-03 meeting.</p>
<h2><code>Deref</code> / <code>Receiver</code></h2>
<p>Ding keeps working on the Reference draft. The idea is still not well-proliferated and people are not convinced this is a good way to go. We hope the method-probing section in Reference PR could clear thins up.</p>
<p>We're keeping the supertrait auto-impl experiment as an alternative.</p>
<h2><a href="https://github.com/rust-lang/rfcs/pull/3851">RFC #3851: Supertrait Auto-impl</a></h2>
<p>Ding addressed Predrag's requests on SemVer compatibility. He's also opened an implementation PR: https://github.com/rust-lang/rust/pull/149335. Here's the tracking issue: https://github.com/rust-lang/rust/issues/149556.</p>
<h2><code>derive(CoercePointee)</code></h2>
<p>Ding opened a PR to require additional checks for DispatchFromDyn: https://github.com/rust-lang/rust/pull/149068</p>
<h2>In-place initialization</h2>
<p>Ding will prepare material for a discussion at the LPC (Linux Plumbers Conference). We're looking to hear feedback on the end-user syntax for it.</p>
<p>The feature is going quite large, Ding will check with Tyler on the whether this might need a series of RFCs.</p>
<p>The various proposals on the table continue being discussed and there are signs (albeit slow) of convergence. The placing function and guaranteed return ones are superseded by outpointer. The more ergonomic ideas can be built on top. The guaranteed value placement one would be valuable in the compiler regardless and we're waiting for Olivier to refine it.</p>
<p>The feeling is that we've now clarified the constraints that the proposals must operate under.</p>
<h2>Field projections</h2>
<p>Nadri's Custom places proposal is looking good at least for the user-facing bits, but the whole thing is growing into a large undertaking. Benno's been focused on academic work that's getting wrapped up soon. The two will sync afterwards.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3671167146" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-12-18:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Quick bit of great news: Rust in the Linux kernel is no longer treated as an experiment, it's here to stay 🎉</p>
<p>https://lwn.net/SubscriberLink/1050174/63aa7da43214c3ce/</p>

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
<span style="font-weight: bold;"><p>3 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/256#issuecomment-3605302867" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @sladyn98 posted on 2025-12-03:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p><a href="https://github.com/epage">Ed Page</a>  hey i would like to contribute to this I reached out on zulip. Bumping up the post in case it might have gone under the radar</p>
<p>CC <a href="https://github.com/nikomatsakis">Niko Matsakis</a></p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/256#issuecomment-3608827740" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @epage posted on 2025-12-03:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>The work is more on the compiler side atm, so <a href="https://github.com/eholk">Eric Holk</a> and <a href="https://github.com/b-naber">b-naber</a> could speak more to where they could use help.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/256#issuecomment-3619077917" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @eholk posted on 2025-12-06:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Hi @sladyn98 - feel free to ping me on Zulip about this.</p>

</div>
</div>
</div>
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
<a href="https://github.com/rust-lang/rust-project-goals/issues/396#issuecomment-3667545376" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @Amanieu posted on 2025-12-17:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>The RFC draft was reviewed in detail and <a href="https://github.com/RalfJung">Ralf Jung</a> pointed out that the proposed semantics introduce issues because they rely on &quot;no-behavior&quot; (NB) with regards to choosing an address for a local. This can lead to surprising &quot;time-traveling&quot; behavior where the set of possible addresses that a local may have (and whether 2 locals can have the same address) depends on information from the future. For example:</p>


<pre><code class="language-rust">// This program has DB
let x = String::new();
let xaddr = &amp;raw const x;
let y = x; // Move out of x and de-initialize it.
let yaddr = &amp;raw const y;
x = String::new(); // assuming this does not change the address of x
// x and y are both live here. Therefore, they can't have the same address.
assume(xaddr != yaddr);
drop(x);
drop(y);
</code></pre>


<pre><code class="language-rust">// This program has UB
let x = String::new();
let xaddr = &amp;raw const x;
let y = x; // Move out of x and de-initialize it.
let yaddr = &amp;raw const y;
// So far, there has been no constraint that would force the addresses to be different.
// Therefore we can demonically choose them to be the same. Therefore, this is UB.
assume(xaddr != yaddr);
// If the addresses are the same, this next line triggers NB. But actually this next
// line is unreachable in that case because we already got UB above...
x = String::new();
// x and y are both live here.
drop(x);
drop(y);
</code></pre>
<hr />
<p>With that said, there is still a possibility of achieving the optimization, but the scope will need to be scaled down a bit. Specifically, we would need to:</p>
<ul>
<li>no longer perform a &quot;partial free&quot;/&quot;partial allocation&quot; when initializing or moving out of a single field of a struct. The lifetime of a local starts when any part of it is initialized and ends when it is fully moved out.</li>
<li>allow a local's address to change when it is re-initialized after having been fully moved out, which eliminates the need for NB.</li>
</ul>
<p>This reduces the optimization opportunities since we can't merge arbitrary sub-field moves, but it still allows for eliminating moves when constructing a struct from multiple values.</p>
<p>The next step is for me to rework the RFC draft to reflect this.</p>

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
<td style="padding: 8px 16px;"><progress value="2" max="4"></progress>
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
<a href="https://github.com/rust-lang/rust-project-goals/issues/398#issuecomment-3649579282" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @weihanglo posted on 2025-12-13:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p><strong>Key developments</strong>: HTML replay logic has merge. Once it gets into nightly <code>cargo report timings</code> can open the timing report you have previously logged.</p>
<ul>
<li>https://github.com/rust-lang/cargo/pull/16377</li>
<li>https://github.com/rust-lang/cargo/pull/16378</li>
<li>https://github.com/rust-lang/cargo/pull/16382</li>
</ul>
<p><strong>Blockers</strong>: No, except my own availability</p>
<p><strong>Help wanted</strong>: Same as <a href="https://github.com/rust-lang/rust-project-goals/issues/398#issuecomment-3571897575">https://github.com/rust-lang/rust-project-goals/issues/398#issuecomment-3571897575</a></p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/398#issuecomment-3691943709" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @weihanglo posted on 2025-12-26:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p><strong>Key developments</strong>:</p>
<p>Headline: You should always enable build analysis locally, if you are using nightly and want the timing info data always available.</p>


<pre><code class="language-toml">[unstable]
build-analysis = true

[build.analysis]
enabled = true
</code></pre>
<ul>
<li>More log events are emitted: https://github.com/rust-lang/cargo/pull/16390
<ul>
<li>dependency resolution time</li>
<li>unit-graph construction</li>
<li>unit-registration (which contain unit metadata)</li>
</ul>
</li>
<li>Timing replay from <code>cargo report timings</code> now has almost the same feature parity as <code>cargo build --timings</code>, except CPU usage: https://github.com/rust-lang/cargo/pull/16414</li>
<li>Rename <code>rebuild</code> event to <code>unit-fingerprint</code>, and is emitted also for fresh unit: <a href="https://github.com/rust-lang/cargo/pull/16408">https://github.com/rust-lang/cargo/pull/16408</a>.</li>
<li>Proposed a new <code>cargo report sessions</code> command so that people can retrieve previous sessions IDs not use the latest one: https://github.com/rust-lang/cargo/pull/16428</li>
<li>Proposed to remove <code>--timings=json</code> which timing info in log files should be a great replacement: https://github.com/rust-lang/cargo/pull/16420</li>
<li>Documenting efforts for having man pages for nested commands `cargo report <!-- raw HTML omitted -->: <a href="https://github.com/rust-lang/cargo/pull/16430">https://github.com/rust-lang/cargo/pull/16430</a> and <a href="https://github.com/rust-lang/cargo/pull/16432">https://github.com/rust-lang/cargo/pull/16432</a></li>
</ul>
<p>Besides implementations, we also discussed about:</p>
<ul>
<li>The interaction of <code>--message-format</code> and structured logging system, as well as log event schemas and formats: https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/build.20analysis.20log.20format/with/558294271</li>
<li>A better name for <code>RunId</code>. We may lean towards <code>SessionId</code> which is a common name for logging/tracing ecosystem.</li>
<li>Nested Cargo calls to have a sticky session ID. At least a way to show they were invoked from the same top-level Cargo call.</li>
</ul>
<p><strong>Blockers</strong>: No, except my own availability</p>
<p><strong>Help wanted</strong>: Same as <a href="https://github.com/rust-lang/rust-project-goals/issues/398#issuecomment-3571897575">https://github.com/rust-lang/rust-project-goals/issues/398#issuecomment-3571897575</a></p>

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
<a href="https://github.com/rust-lang/rust-project-goals/issues/406#issuecomment-3656778925" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @oli-obk posted on 2025-12-15:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3>Updates</h3>
<ul>
<li>https://github.com/rust-lang/rust/pull/148820 adds a way to mark functions and intrinsics as only callable during CTFE</li>
<li>https://github.com/rust-lang/rust/pull/144363 has been unblocked and just needs some minor cosmetic work</li>
</ul>
<h3>Blockers</h3>
<ul>
<li>https://github.com/rust-lang/rust/pull/146923 (reflection MVP) has not been reviewed yet</li>
</ul>

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
<a href="https://github.com/rust-lang/rust-project-goals/issues/401#issuecomment-3687108254" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @ranger-ross posted on 2025-12-23:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h2>Status update December 23, 2025</h2>
<p>The majority of December was spent iterating on https://github.com/rust-lang/cargo/pull/16155 .
As mentioned in the previous update, the original locking design was not correct and we have been working through other solutions.</p>
<p>As locking is tricky to get right and there are many scenarios Cargo needs to support, we are trying to descope the initial implementation to an MVP, even if that means we lose some of the concurrency.
Once we have an MVP on nightly, we can start gathering feedback on the scenarios that need improvement and iterate.</p>
<p>I'm hopeful that we get an unstable <code>-Zfine-grain-locking</code> on nightly in January for folks to try out in their workflows.</p>
<hr />
<p>Also we are considering adding an opt-in for the new build-dir layout using an env var (<code>CARGO_BUILD_DIR_LAYOUT_V2=true</code>) to allow tool authors to begin migrating to the new layout. https://github.com/rust-lang/cargo/pull/16336</p>
<p>Before stabilizing this, we are doing crater run to test the impact of the changes and proactively reaching out to projects to minimize breakage as much as possible. https://github.com/rust-lang/rust/pull/149852</p>

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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
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
<a href="https://github.com/rust-lang/rust-project-goals/issues/403#issuecomment-3656362563" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @jakos-sec posted on 2025-12-15:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Based on the gathered feedback I opened a new MCP for the proposed new Tier 2 targets with sanitizers enabled. (https://github.com/rust-lang/compiler-team/issues/951)</p>

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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
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
<a href="https://github.com/rust-lang/rust-project-goals/issues/275#issuecomment-3655799898" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @Kobzol posted on 2025-12-15:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>We have enabled the second x64 machine, so we now have benchmarks running in <a href="https://perf.rust-lang.org/status.html">parallel</a> 🎉 There are some smaller things to improve, but next year we can move onto running benchmarks on Arm collectors.</p>

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
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/404#issuecomment-3667410003" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @GuillaumeGomez posted on 2025-12-17:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Opened <a href="https://github.com/rust-lang/rust/pull/150055">stabilization PR</a> but we have blockers I didn't hear of, so stabilization will be postponed until then.</p>

</div>
</div>
</div>
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
<span style="font-weight: bold;"><p>3 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3655868247" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @davidtwco posted on 2025-12-15:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>I haven't made any progress on <code>Deref::Target</code> yet, but I have been focusing on landing rust-lang/rust#143924 which has went through two rounds of review and will hopefully be approved soon.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3671055194" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-12-18:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update: <a href="https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals/topic/SVE.20and.20SME.20on.20AArch64.20.28goals.23270.29/near/564463529">David and I chatted on Zulip</a>. Key points:</p>
<p>David has made &quot;progress on the non-Sized Hierarchy part of the goal, the infrastructure for defining scalable vector types has been merged (with them being Sized in the interim) and that'll make it easier to iterate on those and find issues that need solving&quot;.</p>
<p>On the Sized hierarchy part of the goal, no progress. We discussed options for migrating. There seem to be three big options:</p>
<p>(A) The <strong>conservative-but-obvious route</strong> where the <code>T: Deref</code>in the old edition is expanded to <code>T: Deref&lt;Target: SizeOfVal&gt;</code> (but in the new edition it means <code>T: Deref&lt;Target: Pointee&gt;</code>, i.e., no additional bounds). The main <em>downside</em> is that new Edition code using <code>T: Deref</code> can't call old Edition code using <code>T: Deref</code> as the old edition code has stronger bounds. Therefore new edition code must either use stronger bounds than it needs <em>or</em> wait until that old edition code has been updated.</p>
<p>(B) You do something smart with Edition.Old code where you figure out if the bound can be loose or strict by bottom-up computation. So <code>T: Deref</code> in the old could mean either <code>T: Deref&lt;Target: Pointee&gt;</code> or <code>T: Deref&lt;Target: SizeOfVal&gt;</code>, depending on what the function actually does.</p>
<p>(C) You make Edition.Old code always mean <code>T: Deref&lt;Target: Pointee&gt;</code> and you still allow calls to <code>size_of_val</code> but have them cause post-monomorphization errors if used inappropriately. In Edition.New you use stricter checking.</p>
<p>Options (B) and (C) have the downside that changes to the function body (adding a call to <code>size_of_val</code>, specifically) in the old edition can stop callers from compiling. In the case of Option (B), that breakage is at type-check time, because it can change the where-clauses. In Option (C), the breakage is post-monomorphization.</p>
<p>Option (A) has the disadvantage that it takes longer for the new bounds to roll out.</p>
<p>Given this, (A) seems the preferred path. We discussed options for how to encourage that roll-out. We discussed the idea of a lint that would warn Edition.Old code that its bounds are stronger than needed and suggest rewriting to <code>T: Deref&lt;Target: Pointee&gt;</code> to explicitly disable the stronger Edition.Old default. This lint could be implemented in one of two ways</p>
<ul>
<li>at type-check time, by tracking what parts of the environment are used by the trait solver. This may be feasible in the new trait solver, someone from @rust-lang/types would have to say.</li>
<li>at post-mono time, by tracking which functions <em>actually call</em> <code>size_of_val</code> and propagating that information back to callers. You could then compare against the generic bounds declared on the caller.</li>
</ul>
<p>The former is more useful (knowing what parts of the environment are necessary could be useful for more things, e.g., better caching); the latter may be easier or more precise.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3676042942" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-12-19:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update to the previous post.</p>
<p><a href="https://github.com/tmandry">Tyler Mandry</a> pointed me at <a href="https://rust-lang.zulipchat.com/#narrow/channel/144729-t-types/topic/adding.20implicit.20auto.20traits.2C.20trait.20bound.20error.20to.20FCW/with/564751985">this thread</a>, where <a href="https://github.com/lcnr">lcnr</a> posted <a href="https://lcnr.de/blog/2025/11/28/implicit-auto-traits-assoc-types.html">this nice blog post that he wrote</a> detailing more about (C).</p>
<p>Key insights:</p>
<ul>
<li>Because the use of <code>size_of_val</code> would still cause post-mono errors when invoked on types that are not <code>SizeOfVal</code>, you know that adding <code>SizeOfVal</code> into the function's where-clause bounds is not a breaking change, even though adding a where clause is a breaking change more generally.</li>
<li>But, to <a href="https://github.com/davidtwco">David Wood</a>'s point, it <em>does</em> mean that there is a change to Rust's semver rules: adding <code>size_of_val</code> would become a breaking change, where it is not today.</li>
</ul>
<p>This may well be the best option though, particularly as it allows us to make changes to the defaults across-the-board. A change to Rust's <em>semver rules</em> is not a breaking change in the usual sense. It <em>is</em> a notable shift.</p>

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
<td style="padding: 8px 16px;"><progress value="2" max="4"></progress>
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
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--body-background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/405#issuecomment-3698337880" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BoxyUwU posted on 2025-12-30:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>This month I've written some documentation for how Const Generics is implemented in the compiler. This mostly covers the implementation of the stable functionality as the unstable features are quite in flux right now. These docs can be found here: https://rustc-dev-guide.rust-lang.org/const-generics.html</p>

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
