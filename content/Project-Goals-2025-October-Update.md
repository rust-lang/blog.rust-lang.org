+++
path = "9999/12/31/project-goals-update-october-2025"
title = "Project goals update — October 2025"
authors = ["Tomas Sedovic"]

[extra]
team = "the Goals team"
team_url = "https://rust-lang.org/governance/teams/launching-pad/#team-goals"
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/389#issuecomment-3430872050" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @frank-king posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Status update:</p>
<p>Regarding the TODO list in <a href="https://rust-lang.github.io/rust-project-goals/2025h2/pin-ergonomics.html#the-next-6-months">the next 6 months</a>, here is the current status:</p>
<h2>Introduce <code>&amp;pin mut|const place</code> borrowing syntax</h2>
<ul>
<li>[x] parsing: <a href="https://github.com/rust-lang/rust/issues/135731">#135731</a>, merged.</li>
<li>[ ] lowering and borrowck: not started yet.</li>
</ul>
<p>I've got some primitive ideas about borrowck, and I probably need to confirm with someone who is familiar with MIR/borrowck before starting to implement.</p>
<p>A pinned borrow consists two MIR statements:</p>
<ol>
<li>a borrow statement that creates the mutable reference,</li>
<li>and an adt aggregate statement that put the mutable reference into the <code>Pin</code> struct.</li>
</ol>
<p>I may have to add a new borrow kind so that pinned borrows can be recognized. Then traverse the dataflow graph to make sure that pinned places cannot been moved.</p>
<h2>Pattern matching of <code>&amp;pin mut|const T</code> types</h2>
<p>In the past few months, I have struggled with the <code>!Unpin</code> stuffs (the original design sketch <em>Alternative A</em>), trying implementing it, refactoring, discussing on zulips, and was constantly confused; luckily, we have finally reached a new agreement of the <em>Alternative B</em> version.</p>
<ul>
<li>[ ] <a href="https://github.com/rust-lang/rust/issues/139751">#139751</a> under review (reimplemented regarding <em>Alternative B</em>).</li>
</ul>
<h2>Support <code>drop(&amp;pin mut self)</code> for structurally pinned types</h2>
<ul>
<li>[ ] adding a new <code>Drop::pin_drop(&amp;pin mut self)</code> method: draft PR <a href="https://github.com/rust-lang/rust/issues/144537">#144537</a></li>
</ul>
<p>Supporting both <code>Drop::drop(&amp;mut self)</code> and <code>Drop::drop(&amp;pin mut self)</code> seems to introduce method-overloading to Rust, which I think might need some more general ways to handle (maybe by a rustc attribute?). So instead, I'd like to implemenent this via a new method <code>Drop::pin_drop(&amp;pin mut self)</code> first.</p>
<h2>Introduce <code>&amp;pin pat</code> pattern syntax</h2>
<p>Not started yet (I'd prefer doing that when pattern matching of <code>&amp;pin mut|const T</code> types is ready).</p>
<h2>Support <code>&amp;pin mut|const T</code> -&gt; <code>&amp;|&amp;mut T</code> coercion (requires <code>T: Unpin</code> of <code>&amp;pin mut T</code> -&gt; <code>&amp;mut T</code>)</h2>
<p>Not started yet. (It's quite independent, probably someone else can help with it)</p>
<h2>Support auto borrowing of <code>&amp;pin mut|const</code> place in method calls with <code>&amp;pin mut|const self</code> receivers</h2>
<p>Seems to be handled by <a href="https://rust-lang.github.io/rust-project-goals/2025h2/autoreborrow-traits.html">Autoreborrow traits</a>?</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<strong>TL;DR.</strong> <p>There have been lots of internal developments since the last update:</p>
<ul>
<li><a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3438476150">field representing types and chained projections</a> have received a fundamental overhaul: disallowing field paths and requiring projections to decompose. Additionally we explored how const generics could emulate FRTs.</li>
<li><a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3438481368">we discussed a potential solution to having only a single project operator &amp; trait</a> through a decay operation with special borrow checker treatment.</li>
<li><a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3438552573">we were able to further simplify the project trait</a> moving the generic argument of the represented field to the project function. We also discovered the possibility that FRTs are not fundamentally necessary for field projections -- however, they are still very useful in other applications and my gut feeling is that they are also right for field projections. So we will continue our experiment with them.</li>
<li><a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3438584812">we talked about making <code>Project::project</code> be a safe function</a> by introducing a new kind of type.</li>
</ul>
<p><strong>Next Steps</strong>:</p>
<ul>
<li>we're still planning to merge https://github.com/rust-lang/rust/pull/146307, after I have updated it with the new FRT logic and it has been reviewed</li>
<li>once that PR lands, I plan to update the <a href="https://github.com/Rust-for-Linux/field-projection/tree/new">library experiment</a> to use the experimental FRTs</li>
<li>then the testing using that library can begin in the Linux kernel and other projects (this is where anyone interested in trying field projections can help out!)</li>
</ul>

</div>

<!-- Help Wanted Section -->

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>4 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3438476150" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-10-23:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3>Decomposing Projections</h3>
<p>A chained projection operation should naturally decompose, so <code>foo.[Ber Clausen][].[Baz Shkara][]</code> should be the same as writing <code>(foo.[Ber Clausen][]).[Baz Shkara][]</code>. Until now, the different parenthesizing would have allowed different outcomes. This behavior is confusing and also makes many implementation details more complicated than they need to be.</p>
<h3>Field Representing Types</h3>
<p>Since projections now decompose, we have no need from a design perspective for multi-level FRTs. So <code>field_of!(Foo, bar.baz)</code> is no longer required to work. Thus we have decided to restrict FRTs to only a single field and get rid of the path. This simplifies the implementation in the compiler and also avoids certain difficult questions such as the locality of FRTs (if we had a path, we would have to walk the path and it is local, if all structs included in the path are local). Now with only a single field, the FRT is local if the struct is.</p>
<p>We also discovered that it is a good idea to make FRTs inhabited (they still are ZSTs), since then it allows the following pattern to work:</p>
<pre><code class="language-rust">fn project_free_standing&lt;F: Field&gt;(_: Field, r: &amp;F::Base) -&gt; &amp;F::Type { ... }

// can now call the function without turbofish:
let my_field = project_free_standing(field_of!(MyStruct, my_field), &amp;my_struct);
</code></pre>
<h4>FRTs via <code>const</code> Generics</h4>
<p>We also spent some time thinking about const generics and FRTs on zulip:</p>
<ul>
<li><a href="https://rust-lang.zulipchat.com/#narrow/channel/144729-t-types/topic/const.20generics.3A.20implementing.20field.20representing.20types/with/544617587">https://rust-lang.zulipchat.com/#narrow/channel/144729-t-types/topic/const.20generics.3A.20implementing.20field.20representing.20types/with/544617587</a></li>
<li><a href="https://rust-lang.zulipchat.com/#narrow/channel/144729-t-types/topic/field.20representing.20values.20.26.20.60Field.3Cconst.20F.3A.20.3F.3F.3F.3E.60.20trait/with/542855620">https://rust-lang.zulipchat.com/#narrow/channel/144729-t-types/topic/field.20representing.20values.20.26.20.60Field.3Cconst.20F.3A.20.3F.3F.3F.3E.60.20trait/with/542855620</a></li>
</ul>
<p>In short, this won't be happening any time soon. However, it could be a future implementation of the <code>field_of!</code> macro depending on how reflection through const generics evolves (but also only in the far-ish future).</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3438481368" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-10-23:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3>Single Project Operator &amp; Trait via <strong>Exclusive Decay</strong></h3>
<p>It would be great if we only had to add a single operator and trait and could obtain the same features as we have with two. The current reason for having two operators is to allow both shared and exclusive projections. If we could have another operation that <em>decays</em> an exclusive reference (or custom, exclusive smart-pointer type) into a shared reference (or the custom, shared version of the smart pointer). This decay operation would need borrow checker support in order to have simultaneous projections of one field exclusively and another field shared (and possibly multiple times).</p>
<p>This goes into a similar direction as the reborrowing project goal https://github.com/rust-lang/rust-project-goals/issues/399, however, it needs extra borrow checker support.</p>
<pre><code class="language-rust">fn add(x: cell::RefMut&lt;'_, i32&gt;, step: i32) {
    *x = *x + step;
}

struct Point {
    x: i32,
    y: i32,
}

fn example(p: cell::RefMut&lt;'_, Point&gt;) {
    let y: cell::Ref&lt;'_, i32&gt; = coerce_shared!(p.[@y][]);
    let y2 = coerce_shared!(p.[@y][]); // can project twice if both are coerced
    add(p.[Devon Peticolas][], *y);
    add(p.[Devon Peticolas][], *y2);
    assert_eq!(*y, *y2); // can still use them afterwards
}
</code></pre>
<p>Problems:</p>
<ul>
<li>explicit syntax is annoying for these &quot;coercions&quot;, but</li>
<li>we cannot make this implicit:
<ul>
<li>if this were an implicit operation, only the borrow checker would know when one had to coerce,</li>
<li>this operation is allowed to change the type,</li>
<li>this results in borrow check backfeeding into typecheck, which is not possible or at least extremely difficult</li>
</ul>
</li>
</ul>
<h3>Syntax</h3>
<p>Not much movement here, it depends on the question discussed in the previous section, since if we only have one operator, we could choose <code>.@</code>, <code>-&gt;</code> or <code>~</code>; if we have to have two, then we need additional syntax to differentiate them.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3438552573" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-10-23:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3>Simplifying the <code>Project</code> trait</h3>
<p>There have been some developments in pin ergonomics <a href="https://github.com/rust-lang/rust/issues/130494">https://github.com/rust-lang/rust/issues/130494</a>: &quot;alternative B&quot; is now the main approach which means that <code>Pin&lt;&amp;mut T&gt;</code> has <em>linear</em> projections, which means that it doesn't change its output type depending on the concrete field (really depending on the <strong>field</strong>, not only its type). So it falls into the general projection pattern <code>Pin&lt;&amp;mut Struct&gt;</code> -&gt; <code>Pin&lt;&amp;mut Field&gt;</code> which means that <code>Pin</code> doesn't need any <code>where</code> clauses when implementing <code>Project</code>.</p>
<p>Additionally we have found out that RCU also doesn't need <code>where</code> clauses, as we can also make its projections linear by introducing a <code>MutexRef&lt;'_, T&gt;</code> smart pointer that always allows projections and only has special behavior for <code>T = Rcu&lt;U&gt;</code>. Discussed on zulip after <a href="https://rust-lang.zulipchat.com/#narrow/channel/144729-t-types/topic/field.20representing.20values.20.26.20.60Field.3Cconst.20F.3A.20.3F.3F.3F.3E.60.20trait/near/541874520">this message</a>.</p>
<p>For this reason we can get rid of the generic argument to <code>Project</code> and mandate that all types that support projections support them for <em>all</em> fields. So the new <code>Project</code> trait looks like this:</p>
<pre><code class="language-rust">// still need a common super trait for `Project` &amp; `ProjectMut`
pub trait Projectable {
    type Target: ?Sized;
}

pub unsafe trait Project: Projectable {
    type Output&lt;F: Field&lt;Base = Self::Target&gt;&gt;;

    unsafe fn project&lt;F: Field&lt;Base = Self::Target&gt;&gt;(
        this: *const Self,
    ) -&gt; Self::Output&lt;F&gt;;
}
</code></pre>
<h4>Are FRTs even necessary?</h4>
<p>With this change we can also think about getting rid of FRTs entirely. For example we could have the following <code>Project</code> trait:</p>
<pre><code class="language-rust">pub unsafe trait Project: Projectable {
    type Output&lt;F&gt;;

    unsafe fn project&lt;const OFFSET: usize, F&gt;(
        this: *const Self,
    ) -&gt; Self::Output&lt;F&gt;;
}
</code></pre>
<p>There are other applications for FRTs that are very useful for Rust-for-Linux. For example, storing field information for intrusive data structures directly in that structure as a generic.</p>
<!-- raw HTML omitted -->
<p>More concretely, in the kernel there are workqueues that allow you to run code in parallel to the currently running thread. In order to insert an item into a workqueue, an intrusive linked list is used. However, we need to be able to insert the same item into multiple lists. This is done by storing multiple instances of the <code>Work</code> struct. Its definition is:</p>
<pre><code class="language-rust">pub struct Work&lt;T, const ID: u64&gt; { ... }
</code></pre>
<p>Where the <code>ID</code> generic must be unique inside of the struct.</p>
<pre><code class="language-rust">struct MyDriver {
    data: Arc&lt;MyData&gt;,
    main_work: Work&lt;Self, 0&gt;,
    aux_work: Work&lt;Self, 1&gt;,
    // more fields ...
}

// Then you call a macro to implement the unsafe `HasWork` trait safely.
// It asserts that there is a field of type `Work&lt;MyDriver, 0&gt;` at the given field
// (and also exposes its offset).
impl_has_work!(impl HasWork&lt;MyDriver, 0&gt; for MyDriver { self.main_work });
impl_has_work!(impl HasWork&lt;MyDriver, 1&gt; for MyDriver { self.aux_work });

// Then you implement `WorkItem` twice:

impl WorkItem&lt;0&gt; for MyDriver {
    type Pointer = Arc&lt;Self&gt;;
    
    fn run(this: Self::Pointer) {
        println!(&quot;doing the main work here&quot;);
    }
}

impl WorkItem&lt;1&gt; for MyDriver {
    type Pointer = Arc&lt;Self&gt;;
    
    fn run(this: Self::Pointer) {
        println!(&quot;doing the aux work here&quot;);
    }
}

// And finally you can call `enqueue` on a `Queue`:

let my_driver = Arc::new(MyDriver::new());
let queue: &amp;'static Queue = kernel::workqueue::system_highpri();
queue.enqueue::&lt;_, 0&gt;(my_driver.clone()).expect(&quot;my_driver is not yet enqueued for id 0&quot;);

// there are different queues
let queue = kernel::workqueue::system_long();
queue.enqueue::&lt;_, 1&gt;(my_driver.clone()).expect(&quot;my_driver is not yet enqueued for id 1&quot;);

// cannot insert multiple times:
assert!(queue.enqueue::&lt;_, 1&gt;(my_driver.clone()).is_err());
</code></pre>
<p>FRTs could be used instead of this id, making the definition be <code>Work&lt;F: Field&gt;</code> (also merging the <code>T</code> parameter).</p>
<pre><code class="language-rust">struct MyDriver {
    data: Arc&lt;MyData&gt;,
    main_work: Work&lt;field_of!(Self, main_work)&gt;,
    aux_work: Work&lt;field_of!(Self, aux_work)&gt;,
    // more fields ...
}

impl WorkItem&lt;field_of!(MyDriver, main_work)&gt; for MyDriver {
    type Pointer = Arc&lt;Self&gt;;
    
    fn run(this: Self::Pointer) {
        println!(&quot;doing the main work here&quot;);
    }
}

impl WorkItem&lt;field_of!(MyDriver, aux_work)&gt; for MyDriver {
    type Pointer = Arc&lt;Self&gt;;
    
    fn run(this: Self::Pointer) {
        println!(&quot;doing the aux work here&quot;);
    }
}

let my_driver = Arc::new(MyDriver::new());
let queue: &amp;'static Queue = kernel::workqueue::system_highpri();
queue
    .enqueue(my_driver.clone(), field_of!(MyDriver, main_work))
    //                          ^ using Gary's idea to avoid turbofish
    .expect(&quot;my_driver is not yet enqueued for main_work&quot;);

let queue = kernel::workqueue::system_long();
queue
    .enqueue(my_driver.clone(), field_of!(MyDriver, aux_work))
    .expect(&quot;my_driver is not yet enqueued for aux_work&quot;);

assert!(queue.enqueue(my_driver.clone(), field_of!(MyDriver, aux_work)).is_err());
</code></pre>
<p>This makes it overall a lot more readable (by providing sensible names instead of magic numbers), and maintainable (we can add a new variant without worrying about which IDs are unused). It also avoids the <code>unsafe</code> <code>HasWork</code> trait and the need to write the <code>impl_has_work!</code> macro for each <code>Work</code> field.</p>
<!-- raw HTML omitted -->
<p>I still think that having FRTs is going to be the right call for field projections as well, so I'm going to keep their experiment going. However, we should fully explore their necessity and rationale for a future RFC.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3438584812" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-10-23:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3>Making <code>Project::project</code> safe</h3>
<p>In the current proposal the <code>Project::project</code> function is <code>unsafe</code>, because it takes a raw pointer as an argument. This is pretty unusual for an operator trait (it would be the first). <a href="https://github.com/tmandry">Tyler Mandry</a> thought about a way of making it safe by introducing &quot;partial struct types&quot;. This new type is spelled <code>Struct.F</code> where <code>F</code> is an FRT of that struct. It's like <code>Struct</code>, but with the restriction that only the field represented by <code>F</code> can be accessed. So for example <code>&amp;Struct.F</code> would point to <code>Struct</code>, but only allow one to read that single field. This way we could design the <code>Project</code> trait in a safe manner:</p>
<pre><code class="language-rust">// governs conversion of `Self` to `Narrowed&lt;F&gt;` &amp; replaces Projectable
pub unsafe trait NarrowPointee {
    type Target;

    type Narrowed&lt;F: Field&lt;Base = Self::Target&gt;&gt;;
}

pub trait Project: NarrowPointee {
    type Output&lt;F: Field&lt;Base = Self::Type&gt;&gt;;

    fn project(narrowed: Self::Narrowed&lt;F&gt;) -&gt; Self::Output&lt;F&gt;;
}
</code></pre>
<p>The <code>NarrowPointee</code> trait allows a type to declare that it supports conversions of its <code>Target</code> type to <code>Target.F</code>. For example, we would implement it for <code>RefMut</code> like this:</p>
<pre><code class="language-rust">unsafe impl&lt;'a, T&gt; NarrowPointee for RefMut&lt;'a, T&gt; {
    type Target = T;
    type Narrowed&lt;F: Field&lt;Base = T&gt;&gt; = RefMut&lt;'a, T.F&gt;;
}
</code></pre>
<p>Then we can make the narrowing a builtin operation in the compiler that gets prepended on the actual coercion operation.</p>
<p>However, this &quot;partial struct type&quot; has a fatal flaw that <a href="https://github.com/oli-obk">Oliver Scherer</a> found (edit by oli: it was actually boxy who found it): it conflicts with <code>mem::swap</code>, if <code>Struct.F</code> has the same layout as <code>Struct</code>, then writing to such a variable will overwrite all bytes, thus also overwriting field that aren't <code>F</code>. Even if we make an exception for these types and moves/copies, this wouldn't work, as a user today can rely on the fact that they write <code>size_of::&lt;T&gt;()</code> bytes to a <code>*mut T</code> and thus have a valid value of that type at that location. <a href="https://github.com/tmandry">Tyler Mandry</a> suggested we make it <code>!Sized</code> and even <code>!MetaSized</code> to prevent overwriting values of that type (maybe the <code>Overwrite</code> trait could come in handy here as well). But this might make &quot;partial struct types&quot; too weak to be truly useful. Additionally this poses many more questions that we haven't yet tackled.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/399#issuecomment-3432955570" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @aapoalas posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Initial implementation of a Reborrow trait for types with only lifetimes with exclusive reference semantics is working but not yet upstreamed not in review. CoerceShared implementation is not yet started.</p>
<p>Proper composable implementation will likely require a different tactic than the current one. Safety and validity checks are currently absent as well and will require more work.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-3472806299" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @davidtwco posted on 2025-10-31:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>We've now opened our first batch of RFCs: rust-lang/rfcs#3873, rust-lang/rfcs#3874 and rust-lang/rfcs#3875</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="5"></progress>
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="8"></progress>
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
<td style="padding: 8px 16px;"><p><a href="https://github.com/dropbear32">Ally Sommers</a>, <a href="https://github.com/osiewicz">Piotr Osiewicz</a></p>
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>3 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/107#issuecomment-3377116274" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-10-07:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>I posted <a href="https://smallcultfollowing.com/babysteps/blog/2025/10/07/the-handle-trait/">this blog post</a> that proposes that we ought to name the trait <code>Handle</code> and define it as a trait where clone produces an &quot;entangled&quot; value -- i.e., a second handle to the same underlying value.</p>
<p>Before that, there's been a LOT of conversation that hasn't made its way onto this tracking issue. Trying to fix that! Here is a brief summary, in any case:</p>
<ul>
<li>It began with the first Rust Project Goals program in 2024H2, where <a href="https://github.com/jkelleyrtp">Jonathan Kelley</a> from Dioxus wrote a <a href="https://dioxus.notion.site/Dioxus-Labs-High-level-Rust-5fe1f1c9c8334815ad488410d948f05e">thoughtful blog post about a path to high-level Rust</a> that eventually became a 2024H2 <a href="https://rust-lang.github.io/rust-project-goals/2024h2/ergonomic-rc.html">project goal towards ergonomic ref-counting</a>.</li>
<li>I wrote a <a href="https://smallcultfollowing.com/babysteps/series/claim/">series of blog posts about a trait I called <code>Claim</code></a>.</li>
<li><a href="https://github.com/joshtriplett">Josh Triplett</a> and I talked and <a href="https://github.com/joshtriplett">Josh Triplett</a> opened <a href="https://github.com/rust-lang/rfcs/pull/3680">RFC #3680</a>[], which proposed a <code>use</code> keyword and <code>use ||</code> closures. Reception, I would say, was mixed; yes, this is tackling a real problem, but there were lots of concerns on the approach. <a href="https://github.com/rust-lang/rfcs/pull/3680#issuecomment-2625526944">I summarized the key points here</a>.</li>
<li><a href="https://github.com/spastorino">Santiago Pastorino</a> implemented experimental support for (a variant of) <a href="https://github.com/rust-lang/rfcs/pull/3680">RFC #3680</a>[] as part of the <a href="https://rust-lang.github.io/rust-project-goals/2025h1/ergonomic-rc.html">2025H1 project goal</a>.</li>
<li>I authored a <a href="https://rust-lang.github.io/rust-project-goals/2025h2/ergonomic-rc.html">2025H2 project goal proposing that we create an alternative RFC focused on higher-level use-cases</a> which prompted <a href="https://github.com/joshtriplett">Josh Triplett</a> and I have to have a long and fruitful conversation in which he convinced me that this was not the right approach.</li>
<li>We had a lang-team design meeting on 2025-08-27 in which I presented this <a href="https://hackmd.io/%5B@rust-lang-team%5D%5B%5D/B12TpGhKle">survey and summary of the work done thus far</a>.</li>
<li>And then at the <a href="https://2025.rustweek.org/unconf/">RustConf 2025 Unconf</a> we had a big group discussion on the topic that I found very fruitful, as well as various follow-up conversations with smaller groups. The name <code>Handle</code> arose from this and I plan to be posting further thoughts as a result.</li>
</ul>
<p><a href="https://github.com/rust-lang/rfcs/pull/3680">RFC #3680</a>: https://github.com/rust-lang/rfcs/pull/3680</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/107#issuecomment-3385854917" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-10-09:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>I wrote up a brief summary of my <a href="https://rust-lang.zulipchat.com/#narrow/channel/410673-t-lang.2Fmeetings/topic/Design.20meeting.202025-08-27.3A.20Ergonomic.20RC/near/543956015">current thoughts on Zulip</a>; I plan to move this content into a series of blog posts, but I figured it was worth laying it out here too for those watching this space:</p>
<blockquote>
<p><a href="https://rust-lang.zulipchat.com/#narrow/channel/410673-t-lang.2Fmeetings/topic/Design.20meeting.202025-08-27.3A.20Ergonomic.20RC/near/543956370"> 09:11</a>
(1) I don't think clones/handles are categorically different when it comes to how much you want to see them made explicit; some applications want them both to be explicit, some want them automatic, some will want a mix -- and possibly other kinds of categorizations.</p>
<p><a href="https://rust-lang.zulipchat.com/#narrow/channel/410673-t-lang.2Fmeetings/topic/Design.20meeting.202025-08-27.3A.20Ergonomic.20RC/near/543956421"> 09:11</a>
(2) But I do think that if you are making everything explicit, it's useful to see the difference between a general purpose clone and a handle.</p>
<p><a href="https://rust-lang.zulipchat.com/#narrow/channel/410673-t-lang.2Fmeetings/topic/Design.20meeting.202025-08-27.3A.20Ergonomic.20RC/near/543956624"> 09:12</a>
(3) I also think there are many classes of software where there is value in having everything explicit -- and that those classes are often the ones most in Rust's &quot;sweet spot&quot;. So we should make sure that it's possible to have everything be explicit ergonomically.</p>
<p><a href="https://rust-lang.zulipchat.com/#narrow/channel/410673-t-lang.2Fmeetings/topic/Design.20meeting.202025-08-27.3A.20Ergonomic.20RC/near/543956749"> 09:12</a>
(4) This does not imply that we can't make automatic clones/handles possible too -- it is just that we should treat both use cases (explicit and automatic) as first-class in importance.</p>
<p><a href="https://rust-lang.zulipchat.com/#narrow/channel/410673-t-lang.2Fmeetings/topic/Design.20meeting.202025-08-27.3A.20Ergonomic.20RC/near/543956952"> 09:13</a>
(5) Right now I'm focused on the explicit case. I think this is what the use-use-everywhere was about, though I prefer a different proposal now -- basically just making handle and clone methods understood and specially handled by the compiler for optimization and desugaring purposes. There are pros and cons to that, obviously, and that's what I plan to write-up in more detail.</p>
<p><a href="https://rust-lang.zulipchat.com/#narrow/channel/410673-t-lang.2Fmeetings/topic/Design.20meeting.202025-08-27.3A.20Ergonomic.20RC/near/543957159"> 09:14</a>
(6) On a related note, I think we also need explicit closure captures, which is a whole interesting design space. I don't personally find it &quot;sufficient&quot; for the &quot;fully explicit&quot; case but I could understand why others might think it is, and it's probably a good step to take.</p>
<p><a href="https://rust-lang.zulipchat.com/#narrow/channel/410673-t-lang.2Fmeetings/topic/Design.20meeting.202025-08-27.3A.20Ergonomic.20RC/near/543957494"> 09:15</a>
(7) I go back and forth on <a href="https://smallcultfollowing.com/babysteps/blog/2023/09/30/profiles/">profiles</a> -- basically a fancy name for lint-groups based on application domain -- and whether I think we should go that direction, but I think that if we were going to go automatic, that's the way I would do it: i.e., the compiler will automatically insert calls to clone and handle, but it will lint when it does so; the lint can by deny-by-default at first but applications could opt into allow for either or both.</p>
<p>I previously wanted allow-by-default but I've decided this is a silly hill to die on, and it's probably better to move in smaller increments.</p>
</blockquote>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/107#issuecomment-3432669824" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update:</p>
<p>There has been more discussion about the Handle trait on Zulip and elsewhere. Some of the notable comments:</p>
<ul>
<li>Downsides of the current name: it's a noun, which doesn't follow Rust naming convention, and the verb <code>handle</code> is very generic and could mean many things.</li>
<li>Alternative names proposed: <code>Entangle/entangle</code> or <code>entangled</code>, <code>Share</code>/<code>share</code>, <code>Alias/alias</code>, or <code>Retain/retain</code>. if we want to seriously hardcore on the science names -- <code>Mitose/mitose</code> or <code>Fission/fission</code>.</li>
<li>There has been some criticism pointing out that focusing on handles means that other types which <em>might</em> be &quot;cheaply cloneable&quot; don't qualify.</li>
</ul>
<p>For now I will go on using the term Handle, but I agree with the critique that it should be a verb, and currently prefer Alias/alias as an alternative.</p>
<hr />
<p>I'm continuing to work my way through the backlog of blog posts about the conversations from Rustconf. The purposes of these blog posts is not just to socialize the ideas more broadly but also to help myself think through them. Here is the latest post:</p>
<p>https://smallcultfollowing.com/babysteps/blog/2025/10/13/ergonomic-explicit-handles/</p>
<p>The point of this post is to argue that, whatever else we do, Rust should have a way to create handles/clones (and closures that work with them) which is at once explicit <em>and</em> ergonomic.</p>
<p>To give a preview of my current thinking, I am working now on the next post which will discuss how we should add an explicit capture clause syntax. This is somewhat orthogonal but not really, in that an explicit syntax would make closures that clone more ergonomic (but only mildly). I don't have a proposal I fully like for this syntax though and there are a lot of interesting questions to work out. As a strawperson, though, you might imagine [this older proposal I wrote up](https://hackmd.io/<a href="https://github.com/nikomatsakis">Niko Matsakis</a>/SyI0eMFXO?type=view), which would mean something like this:</p>
<pre><code class="language-rust">let actor1 = async move(reply_tx.handle()) {
    reply_tx.send(...);
};
let actor2 = async move(reply_tx.handle()) {
    reply_tx.send(...);
};
</code></pre>
<p>This is an improvement on</p>
<pre><code class="language-rust">let actor1 = {
    let reply_tx = reply_tx.handle();
    async move(reply_tx.handle()) {
        reply_tx.send(...);
    }
};
</code></pre>
<p>but only mildly.</p>
<p>The next post I intend to write would be a variant on &quot;use, use everywhere&quot; that recommends <em>method call syntax</em> and permitting the compiler to elide handle/clone calls, so that the example becomes</p>
<pre><code class="language-rust">let actor1 = async move {
    reply_tx.handle().send(...);
    //       -------- due to optimizations, this would capture the handle creation to happen only when future is *created*
};
</code></pre>
<p>This would mean that cloning of strings and things might benefit from the same behavior:</p>
<pre><code class="language-rust">let actor1 = async move {
    reply_tx.handle().send(some_id.clone());
    //                     -------- the `some_id.clone()` would occur at future creation time
};
</code></pre>
<p>The rationable that got me here is (a) minimizing perceived complexity and focusing on muscle memory (just add <code>.clone()</code> or <code>.handle()</code> to fix use-after-move errors, no matter when/where they occur). The cost of course is that (a) Handle/Clone become very special; and (b) it blurs the lines on when code execution occurs. Despite the <code>.handle()</code> occurring inside the future (resp. closure) body, it actually executes when the future (resp. closure) is created in this case (in other cases, such as a closure that implements <code>Fn</code> or <code>FnMut</code> and hence executes more than once, it might occur during each execution as well).</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>


### "Unblocking dormant traits"

<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Evolving trait hierarchies <a href='https://github.com/rust-lang/rust-project-goals/issues/393' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#393)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/395#issuecomment-3433019959" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @Darksonn posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>This is our first update we’re posting for the in-place init work. Overall things are progressing well, with lively discussion happening on the newly minted <code>t-lang/in-place-init</code> Zulip channel. Here are the highlights since the lang team design meeting at the end of July:</p>
<ul>
<li><strong>Zulip</strong>: we now have a dedicated zulip channel that includes all topics surrounding in-place initialization: <a href="https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init">#t-lang/in-place-init</a>.</li>
<li><strong>Guaranteed value emplacement</strong>: Olivier FAURE shared a new version of C++ inspired emplacement in <a href="https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init/topic/RFC.20Draft.3A.20Guaranteed.20Value.20Emplacement/with/545850965">#t-lang/in-place-init &gt; RFC Draft: Guaranteed Value Emplacement</a> inspired by C++’s emplacement system.</li>
<li><strong>Rosetta code sample</strong>: to help guide the comparison of the various proposals, we’ve started collecting examples to compare against each other. The first one was contributed by <a href="https://github.com/darksonn">Alice Ryhl</a> and is: <em>“How can we construct a <code>Box&lt;Mutex&lt;MyType&gt;&gt;</code> in-place inside the <code>Box</code>”</em>. For more see <a href="https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init/topic/Shared.20example.3A.20emplacing.20into.20.60Box.3CMutex.3CMyStruct.3E.3E.60/with/546076697">#t-lang/in-place-init &gt; Shared example: emplacing into `Box</a>.</li>
<li><strong>Evolution of the outptr proposal</strong>: <a href="https://github.com/cramertj">Taylor Cramer</a>’s original outptr-based emplacement proposal used concrete types as part of her proposal. Since then there has been significant discussion about alternative ways to represent out-pointers, including: <a href="https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init/topic/out-pointer.20type.20and.20MIR.20semantics.20consideration/with/543431067">#t-lang/in-place-init &gt; out-pointer type and MIR semantics consideration</a>.</li>
<li><strong>Placing functions as a high-level notation</strong>: <a href="https://github.com/yoshuawuyts">Yoshua Wuyts</a> has begun reworking the “placing functions” proposal as a high-level sugar on top of one of the other proposals, instead of directly desugaring to <code>MaybeUninit</code>. For more see: <a href="https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init/topic/Placing.20functions.20as.20sugar.20for.20low-level.20emplacement/with/546201858">#t-lang/in-place-init &gt; Placing functions as sugar for low-level emplacement</a>.</li>
<li><strong>Generic fallibility for the <code>Init</code> proposal</strong>: following feedback from the lang team meeting, <a href="https://github.com/darksonn">Alice Ryhl</a> posted an update showing how the <code>Init</code> trait could be made generic over <em>all</em> <code>Try</code> types instead of being limited to just <code>Result</code>. For more see: <a href="https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init/topic/Making.20.60impl.20Init.60.20generic.20over.20.60Result.60.2F.60Option.60.2Finfallible/with/543195128">#t-lang/in-place-init &gt; Making <code>impl Init</code> generic over <code>Result</code>/<code>Option</code>/infallible</a>.</li>
<li><strong>Interactions between emplacement and effects</strong>: <a href="https://github.com/yoshuawuyts">Yoshua Wuyts</a> has begun documenting the expected interactions between placing functions and other function-transforming effects (e.g. <code>async</code>, <code>try</code>, <code>gen</code>). For more see: <a href="https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init/topic/placing.20functions.20and.20interactions.20with.20effects/with/546220108">#t-lang/in-place-init &gt; placing functions and interactions with effects</a>.</li>
</ul>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-3436473908" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @lcnr posted on 2025-10-23:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Since the last update we've fixed the hang in rayon in https://github.com/rust-lang/rust/pull/144991 and https://github.com/rust-lang/rust/pull/144732 which relied on https://github.com/rust-lang/rust/pull/143054 https://github.com/rust-lang/rust/pull/144955 https://github.com/rust-lang/rust/pull/144405 https://github.com/rust-lang/rust/pull/145706. This introduced some search graph bugs which we fixed in https://github.com/rust-lang/rust/pull/147061 https://github.com/rust-lang/rust/pull/147266.</p>
<p>We're mostly done with the opaque type support now. Doing so required a lot of quite involved changes:</p>
<ul>
<li>https://github.com/rust-lang/rust/pull/145244 non-defining uses in borrowck</li>
<li>https://github.com/rust-lang/rust/pull/145925 non-defining uses in borrowck closure support</li>
<li>https://github.com/rust-lang/rust/pull/145711 non-defining uses in hir typeck</li>
<li>https://github.com/rust-lang/rust/pull/140375 eagerly compute sub_unification_table again</li>
<li>https://github.com/rust-lang/rust/pull/146329 item bounds</li>
<li>https://github.com/rust-lang/rust/pull/145993 function calls</li>
<li>https://github.com/rust-lang/rust/pull/146885 method selection</li>
<li>https://github.com/rust-lang/rust/pull/147249 fallback</li>
</ul>
<p>We also fixed some additional self-contained issues and perf improvements: https://github.com/rust-lang/rust/pull/146725 https://github.com/rust-lang/rust/pull/147138 https://github.com/rust-lang/rust/pull/147152 https://github.com/rust-lang/rust/pull/145713 https://github.com/rust-lang/rust/pull/145951</p>
<p>We have also migrated rust-analyzer to entirely use the new solver instead of <code>chalk</code>. This required a large effort mainly by <a href="https://github.com/jackh726">Jack Huey</a> <a href="https://github.com/ChayimFriedman2">Chayim Refael Friedman</a> and <a href="https://github.com/ShoyuVanilla">Shoyu Vanilla</a>. That's some really impressive work on their end 🎉 See <a href="https://github.com/rust-lang/rust-analyzer/pulls?q=is%3Apr+solver+merged%3A%3E2025-08-12+-merged%3A%3E2025-10-23+">this list of merged PRs</a> for an overview of what this required on the r-a side. <a href="https://github.com/ChayimFriedman2">Chayim Refael Friedman</a> also landed some changes to the trait solver itself to simplify the integration: https://github.com/rust-lang/rust/pull/145377 https://github.com/rust-lang/rust/pull/146111 https://github.com/rust-lang/rust/pull/147723 https://github.com/rust-lang/rust/pull/146182.</p>
<p>We're still tracking the remaining issues in https://github.com/orgs/rust-lang/projects/61/views/1. Most of these issues are comparatively simple and I expect us to fix most of them over the next few months, getting us close to stabilization. We're currently doing another crater triage which may surface a few more issues.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-3433251687" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @lqd posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Here's another summary of the most interesting developments since the last update:</p>
<ul>
<li>reviews and updates have been done on the polonius alpha, and it has since landed</li>
<li>the last 2 trivial diagnostics failures were fixed</li>
<li>we've done perf runs, crater runs, completed gathering stats on crates.io for avg and outliers in CFG sizes, locals, loan and region counts, dataflow framework behavior on unexpected graph shapes and bitset invalidations</li>
<li>I worked on dataflow for borrowck: single pass analyses on acyclic CFGs, dataflow analyses on SCCs for cyclic CFGs</li>
<li>some more pieces of amanda's SCC rework have landed, with lcnr's help</li>
<li>lcnr's opaque type rework, borrowcking of nested items, and so on, also fixed some issues we mentioned in previous updates with member constraints for computing when loans are going out of scope</li>
<li>we also studied recent papers in flow-sensitive pointer analysis</li>
<li>I also started the loans-in-scope algorithm rework, and also have reachability acceleration with the CFG SCCs</li>
<li>the last 2 actual failures in the UI tests are soundness issues, related to liveness of captured regions for opaque types: some regions that should be live are not, which were done to help with precise capture and limit the impact of capturing unused regions that cannot be actually used in the hidden type. The unsoundness should not be observable with NLLs, but polonius alpha relies on liveness to propagate loans throughout the CFG: these dead regions prevent detecting some error-causing loan invalidations. The easiest fix would cause breakage in code that's now accepted. niko, jack and I have another possible solution and I'm trying to implement it now</li>
</ul>

</div>
</div>
</div>
</details>

</div>


## Goals looking for help


## Other goal updates

<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Add a team charter for rustdoc team <a href='https://github.com/rust-lang/rust-project-goals/issues/387' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#387)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="1"></progress>
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
C++/Rust Interop Problem Space Mapping <a href='https://github.com/rust-lang/rust-project-goals/issues/388' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#388)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="7"></progress>
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3432925171" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>We had a design meeting on 2025-09-10, <a href="https://hackmd.io/a4Ds5Ec0QlaOzPeyoPQE8w">minutes available here</a>, aiming at these questions:</p>
<blockquote>
<p>There are a few concrete things I would like to get out of this meeting, listed sequentially in order of most to least important:</p>
<ol>
<li>Would you be comfortable stabilizing the initial ADTs-only extensions?
<ul>
<li>This would be properly RFC'd before stabilization, this ask is just a &quot;vibe check&quot;.</li>
</ul>
</li>
<li>Are you interested in seeing Per-Value Rejection for enums with undesirable variants?</li>
<li>How do you feel about the idea of Lossy Conversion as an approach in general, what about specifically for the References and Raw Pointers extensions?</li>
<li>How do you feel about the idea of dropping the One Equality ideal in general, what about specifically for <code>-0.0</code> vs <code>+0.0</code>, what about specifically for <code>NaN</code> values?</li>
</ol>
</blockquote>
<p>The vibe checks on the first one were as follows:</p>
<blockquote>
<h2>Vibe check</h2>
<p>The main ask:</p>
<blockquote>
<p>Would you be comfortable stabilizing the initial ADTs-only extensions?</p>
</blockquote>
<p>(plus the other ones)</p>
<h3>nikomatsakis</h3>
<p>I am +1 on working incrementally and focusing first on ADTs. I am supportive of stabilization overall but I don't feel like we've &quot;nailed&quot; the way to talk or think about these things. So I guess my &quot;vibe&quot; is +1 but if this doc were turned into an RFC kind of &quot;as is&quot; I would probably wind up -1 on the RFC, I think more work is needed (in some sense, the question is, &quot;what is the name of the opt-in trait and why is it named that&quot;). This space is complex and I think we have to do better at helping people understand the fine-grained distinctions between runtime values, const-eval values, and type-safe values.</p>
<p>Niko: if we add some sort of derive of a trait name, how much value are we getting from the derive, what should the trait be named?</p>
<h3>tmandry</h3>
<p>I think we'll learn the most by stabilizing ADTs in a forward compatible way (including an opt-in) now. So +1 from me on the proposed design.</p>
<p>It's worth noting that this is a feature that interacts with many other features, and we will be considering extensions to the MVP for the foreseeable future. To some extent the lang team has committed to this already but we should know what we're signing ourselves up for.</p>
<h3>scottmcm</h3>
<p>scottmcm: concern over the private fields restriction (see question below), but otherwise for the top ask, yes happy to just do &quot;simple&quot; types (no floats, no cells, no references, etc).</p>
<h3>TC</h3>
<p>As Niko said, +1 on working incrementally, and I too am supportive overall.</p>
<p>As a vibe, per-value rejection seems fairly OK to me in that we decided to do value-based reasoning for other const checks.  It occurs to me there's some parallel with that.</p>
<p>https://github.com/rust-lang/rust/pull/119044</p>
<p>As for the opt-in on types, I see the logic.  I do have reservations about adding too many opt-ins to the language, and so I'm curious about whether this can be safely removed.</p>
<p>Regarding floats, I see the question on these as related to our decision about how to handle padding in structs.  If it makes sense to normalize or otherwise treat <code>-0.0</code> and <code>+0.0</code> as the same, then it'd also make sense in my view to normalize or otherwise treat two structs with the same values but different padding (or where only one has initialized padding) as the same.</p>
</blockquote>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>2 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/391#issuecomment-3432696119" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>After much discussion, we have decided to charter this team as a t-spec subteam. <a href="https://github.com/PLeVasseur">Pete LeVasseur</a> and I are working to make that happen now.</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/391#issuecomment-3432855906" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>PR with charters:</p>
<p>https://github.com/rust-lang/team/pull/2028</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/392#issuecomment-3446674680" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @icmccorm posted on 2025-10-25:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Here's our first status update!</p>
<ul>
<li>
<p>We've been experimenting with a few different ways of emitting retags in codegen, as well as a few different forms that retags should take at this level. We think we've settled on <a href="https://github.com/BorrowSanitizer/rust/commit/d623b8445b625b1d88ccd8be514800e571217f2e">a set of changes</a> that's worth sending out to the community for feedback, likely as a pre-RFC. You can expect more engagement from us on this level in the next couple of weeks.</p>
</li>
<li>
<p>We've used these changes to create an initial working prototype for <a href="https://github.com/borrowSanitizer/bsan">BorrowSanitizer</a> that supports finding Tree Borrows violations in tiny, single-threaded Rust programs. We're working on getting Miri's test suite ported over to confirm that everything is working correctly and that we've quashed any false positives or false negatives.</p>
</li>
<li>
<p>This coming Monday, I'll be presenting on BorrowSanitizer and this project goal at the <a href="https://discourse.llvm.org/t/cfp-2025-workshop-on-supporting-memory-safety-in-llvm/88184">Workshop on Supporting Memory Safety in LLVM</a>. Please reach out if you're attending and would like to chat more in person!</p>
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
Expand the Rust Reference to specify more aspects of the Rust language <a href='https://github.com/rust-lang/rust-project-goals/issues/394' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#394)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/394#issuecomment-3431110401" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @joshtriplett posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>The work on this goal has led to many ongoing discussions on the current status of the Reference. Those discussions are still in progress.</p>
<p>Meanwhile, many people working on this goal have successfully written outlines or draft chapters, at various stages of completeness. There's a broken-out status report at https://github.com/rust-lang/project-goal-reference-expansion/issues/11 .</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="3" max="8"></progress>
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-3433494189" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @ZuseZ4 posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>A longer update of the changes over the fall. We had two gsoc contributors and a lot of smaller improvements for std::autodiff. The first two improvements were already mentioned as draft PRs in the previous update, but got merged since. I also upstreamed more std::offload changes.</p>
<ol>
<li><a href="https://github.com/Sa4dUs">Marcelo Domínguez</a> refactored the autodiff frontend to be a proper rustc intrinsic, rather than just hackend into the frontend like I first implemented it. This already solved multiple open issues, reduced the code size, and made it generally easier to maintain going forward.</li>
<li><a href="https://github.com/KMJ-007">Karan Janthe</a> upstreamed a first implementation of &quot;TypeTrees&quot;, which lowers rust type and layout information to Enzyme, our autodiff backend. This makes it more likely that you won't see compilation failures with the error message &quot;Can not deduce type of <!-- raw HTML omitted -->&quot;. We might refine in the future what information exactly we lower.</li>
<li><a href="https://github.com/KMJ-007">Karan Janthe</a> made sure that std::autodiff has support for f16 and and f128 types.</li>
<li>One more of my offload PRs landed. I also figured out why the LLVM-IR generated by the std::offload code needed some manual adjustments in the past. We were inconsistent when communicating with LLVM's offload module, about whether we'd want a magic, extra, <em>dyn_ptr</em> argument, that enables kernels to use some extra features. We don't use these features yet, but for consistency we now always generate and expect the extra pointer. The bugfix is currently under review, once it lands upstream, rustc is able to run code on GPUs (still with a little help of clang).</li>
<li><a href="https://github.com/Sa4dUs">Marcelo Domínguez</a> refactored my offload frontend, again introducing a proper rustc intrinsic. That code will still need to go through review, but once it lands it will get us a lot closer to a usable frontend. He also started to generate type information for our offload backend to know how many bytes to copy to and from the devices. This is a very simplified version of our autodiff typetrees.</li>
<li>At RustChinaConf, I was lucky to run into the wild linker author <a href="https://github.com/davidlattimore">David Lattimore</a>, which helped me to create a <a href="https://github.com/rust-lang/rust/pull/146623">draft PR</a> that can dlopen Enzyme at runtime. This means we could ship it via rustup for people interested in std::autodiff, and don't have to link it in at build time, which would increase binary size even for those users that are not interested in it. There are some open issues, so please reach out if you have time to get the PR ready!</li>
<li><a href="https://github.com/sgasho">@sgasho</a> spend a lot of time trying to get Rust into the Enzyme CI. Unfortunately that is a tricky process due to Enzyme's CI requirements, so it's not merged yet.</li>
<li>I tried to simplify building std::autodiff by marking it as compatible with download-llvm-ci. Building LLVM from source was previously the by far slowest part of building rustc with autodiff, so this has a large potential. Unfortunately the CI experiments revealed some issues around this setting. We think we know why Enzyme's Cmake causes issues here and are working on a fix to make it more reliable.</li>
<li><a href="https://github.com/osamakader">Osama Abdelkader</a> and <a href="https://github.com/bjorn3">bjorn3</a> looked into automatically enabling fat-lto when autodiff is enabled. In the past, forgetting to enable fat-lto resulted in incorrect (zero) derivatives. The first approach unfortunately wasn't able to cover all cases, so we need to see whether we can handle it nicely. If that turns out to be too complicated, we will revert it and instead &quot;just&quot; provide a nice error message, rather than returning incorrect derivatives.</li>
</ol>
<p>All-in-all I spend a lot more time on infra (dlopen, cmake, download-llvm-ci, ...) then I'd like, but on the happy side there are only so many features left that I want to support here so there is an end in sight.
I am also about to give a tech-talk at the upcoming LLVM dev meeting about safe GPU programming in Rust.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>3 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/407#issuecomment-3385530348" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-10-09:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>I've updated the top-level description to show everything we're tracking here (please let me know if anything's missing or incorrect!).</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/407#issuecomment-3390674922" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-10-10:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<ul>
<li>[merged] Sanitizers target modificators / https://github.com/rust-lang/rust/pull/138736</li>
<li>[merged] Add assembly test for -Zreg-struct-return option / https://github.com/rust-lang/rust/pull/145382</li>
<li>[merged] CI: rfl: move job forward to Linux v6.17-rc5 to remove temporary commits / https://github.com/rust-lang/rust/pull/146368</li>
<li><code>-Zharden-sls</code> / https://github.com/rust-lang/rust/pull/136597
<ul>
<li>Waiting on review</li>
</ul>
</li>
<li><code>#![register_tool]</code> / https://github.com/rust-lang/rust/issues/66079
<ul>
<li>Waiting on https://github.com/rust-lang/rfcs/pull/3808</li>
</ul>
</li>
<li><code>-Zno-jump-tables</code> / https://github.com/rust-lang/rust/pull/145974
<ul>
<li>Active FCP, waiting on 2 check boxes</li>
</ul>
</li>
</ul>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/407#issuecomment-3442005789" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-10-24:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3><code>-Cunsigned-char</code></h3>
<p>We've discussed adding an option analogous to <code>-funsigned-char</code> in GCC and Clang, that would allow you to set whether <code>std::ffi::c_char</code> is represented by <code>i8</code> or <code>u8</code>. Right now, this is platform-specific and should map onto whatever <code>char</code> is in C on the same platform. However, Linux explicitly sets <code>char</code> to be unsigned and then our Rust code conflicts with that. And isn this case the sign is significant.</p>
<p>Rust for Linux works around this this with their <a href="https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/rust/ffi.rs">rust::ffi module</a>, but now that they've switched to the standard library's <code>CStr</code> type, they're running into it again with the <a href="https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.as_ptr"><code>as_ptr</code> method</a>.</p>
<p>Tyler mentioned https://docs.rs/ffi_11/latest/ffi_11/ which preserves the <code>char</code> / signed <code>char</code> / unsigned <code>char</code> distinction.</p>
<h3>Grouping target modifier flags</h3>
<p>The proposed <code>unsigned-char</code> option is essentially a target modifier. We have several more of these (e.g. <code>llvm-args</code>, <code>no-redzone</code>) in the Rust compiler and Josh suggested we distinguish them somehow. E.g. by giving them the same prefix or possibly creating a new config option (right now we have <code>-C</code> and <code>-Z</code>, maybe we could add <code>-T</code> for target modifiers) so they're distinct from the e.g. the codegen options.</p>
<p>Josh started a Zulip thread here: https://rust-lang.zulipchat.com/#narrow/channel/131828-t-compiler/topic/Grouping.20target.20modifier.20options.3F/with/546524232</p>
<h3><code>#![register_tool]</code> / <a href="https://github.com/rust-lang/rust/issues/66079">rust#66079</a> / <a href="https://github.com/rust-lang/rfcs/pull/3808">RFC#3808</a></h3>
<p>Tyler looked at the RFC. The Crubit team started using <code>register_tool</code> but then moved to using an attribute instead. <a href="https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/namespaced.20tool.20attrs/with/544748021">He proposed we could do something similar here</a>, although it would require a new feature and RFC.</p>
<p>The team was open to seeing how it would work.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>3 detailed updates available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3385528657" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-10-09:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>I've updated the top-level description to show everything we're tracking here (please let me know if anything's missing or incorrect!).</p>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3390645417" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-10-10:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3><code>Deref</code>/<code>Receiver</code></h3>
<ul>
<li><a href="https://github.com/dingxiangfei2009">Ding Xiang Fei</a>  keeps updating the PR: https://github.com/rust-lang/rust/pull/146095</li>
<li>They're also working on a document to explain the consequences of this split</li>
</ul>
<h3>Arbitrary Self Types</h3>
<ul>
<li>https://github.com/rust-lang/rust/issues/44874</li>
<li>Waiting on the <code>Deref</code>/<code>Receiver</code> work, no updates</li>
</ul>
<h3><code>derive(CoercePointee)</code></h3>
<ul>
<li>https://github.com/rust-lang/rust/pull/133820</li>
<li>Waiting on Arbitrary self types</li>
</ul>
<h3>Pass pointers to <code>const</code> in <code>asm!</code> blocks</h3>
<ul>
<li>RFC: https://github.com/rust-lang/rfcs/pull/3848</li>
<li>The Lang team went through the RFC with <a href="https://github.com/Darksonn">Alice Ryhl</a> on 2025-10-08 and it's in FCP now</li>
</ul>
<h3>Field projections</h3>
<ul>
<li><a href="https://github.com/BennoLossin">Benno Lossin</a> opened a PR here: https://github.com/rust-lang/rust/pull/146307</li>
<li>Being reviewed by the compiler folks</li>
</ul>
<h3>Providing <code>\0</code> terminated file names with <code>#[track_caller]</code></h3>
<ul>
<li>The feature has been implemented and stabilized with <code>file_as_c_str</code> as the method name: https://github.com/rust-lang/rust/pull/145664</li>
</ul>
<h3>Supertrait auto impl RFC</h3>
<ul>
<li><a href="https://github.com/dingxiangfei2009">Ding Xiang Fei</a> opened the RFC and works with the reviewers: https://github.com/rust-lang/rfcs/pull/3851</li>
</ul>
<h3>Other</h3>
<ul>
<li><a href="https://github.com/ojeda">Miguel Ojeda</a> spoke to Linus about rustfmt and they came to agreement.</li>
</ul>

</div>
</div>
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3442107142" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @tomassedovic posted on 2025-10-24:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3>Layout of <a href="https://doc.rust-lang.org/stable/core/any/struct.TypeId.html"><code>core::any::TypeId</code></a></h3>
<p>Danilo asked about the layout of <code>TypeId</code> -- specifically its size and whether they can rely on it because they want to store it in a C <code>struct</code>. The struct's size is currently 16 bytes, but that's an implementation detail.</p>
<p>As a vibe check, <a href="https://github.com/joshtriplett">Josh Triplett</a> and <a href="https://github.com/tmandry">Tyler Mandry</a> were open to guaranteeing that it's going to be at most 16 bytes, but they wanted to reserve the option to reduce the size at some point. The next step is to have the full Lang and Libs teams discuss the proposal.</p>
<p>Danilo will open a PR to get that discussion started.</p>
<h3>rustfmt</h3>
<p>Miguel brought up the &quot;trailing empty comment&quot; workaround for the formatting issue that made the rounds on the Linux kernel a few weeks ago. The kernel style places each import on a single line:</p>
<pre><code class="language-rust">    use crate::{
        fmt,
        page::AsPageIter,
    };
</code></pre>
<p>rustfmt compresses this to:</p>
<pre><code class="language-rust">    use crate::{fmt, page::AsPageIter};
</code></pre>
<p>The workaround is to put an empty trailing comment at the end</p>
<pre><code class="language-rust">    use crate::{
        fmt,
        page::AsPageIter, //
    };
</code></pre>
<p>This was deemed acceptable (for the time being) and merged into the mainline kernel: https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=4a9cb2eecc78fa9d388481762dd798fa770e1971</p>
<p>Miguel is in contact with rustfmt to support this behaviour without a workaround.</p>
<h3><code>// PANIC: ...</code> comments / <a href="https://github.com/rust-lang/rust-clippy/issues/15895">clippy#15895</a></h3>
<p>This is a proposal to add a lint that would require a <code>PANIC</code> comment (modeled after the <code>SAFETY</code> comment) to explain the circumstances during which the code will or won't panic.</p>
<p>Alejandra González was open to the suggestion and Henry Barker stepped up to implement it.</p>
<h3><code>Deref</code>/<code>Receiver</code></h3>
<p>During the experimentation work, Ding ran into an issue with overlapping <code>impls</code> (that was present even with <code>#[unstable_feature_bound(..)]</code>). We ran out of time but we'll discuss this offline and return to it at the next meeting.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Prototype a new set of Cargo &quot;plumbing&quot; commands <a href='https://github.com/rust-lang/rust-project-goals/issues/264' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#264)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/398#issuecomment-3368323977" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @weihanglo posted on 2025-10-04:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Cargo tracking issue: <a href="https://github.com/rust-lang/cargo/issues/15844">https://github.com/rust-lang/cargo/issues/15844</a>.
The first implementation was <a href="https://github.com/rust-lang/cargo/pull/15845">https://github.com/rust-lang/cargo/pull/15845</a> in August that added <code>build.analysis.enabled = true</code> to unconditionally generate timing HTML. Further implementations tasks is listed in <a href="https://github.com/rust-lang/cargo/issues/15844#issuecomment-3192779748">https://github.com/rust-lang/cargo/issues/15844#issuecomment-3192779748</a>.</p>
<p>Haven't yet got any progress in September.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/406#issuecomment-3431174687" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @oli-obk posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>I implemented an initial MVP supporting only tuples and primitives (tho those are just opaque things you can't interact with further), and getting offsets for the tuple fields as well as the size of the tuple: https://github.com/rust-lang/rust/pull/146923</p>
<p>There are two designs of how to expose this from a libs perspective, but after a sync meeting with scottmcm yesterday we came to the conclusion that neither is objectively better at this stage so we're just going to go with the nice end-user UX version for now. For details see the PR description.</p>
<p>Once the MVP lands, I will mentor various interested contributors who will keep adding fields to the Type struct and variants the TypeKind enum.</p>
<p>The next major step is restricting what information you can get from structs outside of the current module or crate. We want to honor visibility, so an initial step would be to just never show private fields, but we want to explore allowing private fields to be shown either just within the current module or via some opt-in marker trait</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/401#issuecomment-3371889062" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @ranger-ross posted on 2025-10-06:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h3>Status update October 6, 2025</h3>
<p>The <code>build-dir</code> was split out of <code>target-dir</code> as part of https://github.com/rust-lang/cargo/issues/14125 and <a href="https://github.com/rust-lang/cargo/pull/15833">scheduled for stabilization</a> in Rust <code>1.91.0</code>. 🎉</p>
<p>Before re-organizing the <code>build-dir</code> layout we wanted to improve the existing layout tests to make sure we do not make any unexpected changes. This testing harness improvement was merged in https://github.com/rust-lang/cargo/pull/15874.</p>
<p>The initial <code>build-dir</code> layout reorganization PR has been posted https://github.com/rust-lang/cargo/pull/15947 and discussion/reviews are under way.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="3"></progress>
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Rust Vision Document <a href='https://github.com/rust-lang/rust-project-goals/issues/269' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#269)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/269#issuecomment-3432798929" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @jackh726 posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Update:</p>
<p>Niko and I gave a talk at RustConf 2025 (and I represented that talk at RustChinaConf 2025) where we gave an update on this (and some intermediate insights).</p>
<p>We have started to seriously plan the shape of the final doc. We have some &quot;blind spots&quot; that we'd like to cover before finishing up, but overall we're feeling close to the finish line on interviews.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="14"></progress>
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/275#issuecomment-3427936137" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @Kobzol posted on 2025-10-21:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>We moved forward with the implementation, and the new job queue system is now being tested in production on a single test pull request. Most things seem to be working, but there are a few things to iron out and some profiling to be done. I expect that within a few weeks we could be ready to switch to the new system fully in production.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: var(--blockquote-bg-color); cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: var(--background-color);">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3432823824" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @nikomatsakis posted on 2025-10-22:</a>
<div style="margin-top: 8px; padding: 12px; background: var(--blockquote-bg-color); border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h1>Sized hierarchy</h1>
<p>The focus right now is on the &quot;non-const&quot; parts of the proposal, as the &quot;const&quot; parts are blocked on the new trait solver (https://github.com/rust-lang/rust-project-goals/issues/113). Now that the types team FCP https://github.com/rust-lang/rust/pull/144064 has completed, work can proceed to land the implementation PRs. <a href="https://github.com/davidtwco">David Wood</a> plans to split the RFC to separate out the &quot;non-const&quot; parts of the proposal so it can move independently, which will enable extern types.</p>
<p>To that end, there are three interesting T-lang design questions to be considered.</p>
<h2>Naming of the traits</h2>
<p>The RFC currently proposes the following names</p>
<ul>
<li><code>Sized</code></li>
<li><code>MetaSized</code></li>
<li><code>PointeeSized</code></li>
</ul>
<p>However, these names do not follow the &quot;best practice&quot; of naming the trait after the capability that it provides. As champion Niko is recommending we shift to the following names:</p>
<ul>
<li><code>Sized</code> -- should righly be called <code>SizeOf</code>, but oh well, not worth changing.</li>
<li><code>SizeOfVal</code> -- named after the method <code>size_of_val</code> that you get access to.</li>
<li><code>Pointee</code> -- the only thing you can do is point at it.</li>
</ul>
<p>The last trait name is already used by the (unstable) <a href="https://doc.rust-lang.org/std/ptr/trait.Pointee.html"><code>std::ptr::Pointee</code></a> trait. We do not want to have these literally be the same trait because that trait adds a <code>Metadata</code> associated type which would be backwards incompatible; if existing code uses <code>T::Metadata</code> to mean <code>&lt;T as SomeOtherTrait&gt;::Metadata</code>, it could introduce ambiguity if now <code>T: Pointee</code> due to defaults. My proposal is to rename <code>std::ptr::Pointee</code> to <code>std::ptr::PointeeMetadata</code> for now, since that trait is unstable and the design remains under some discussion. The two traits could either be merged eventually or remain separate.</p>
<p>Note that <code>PointeeMetadata</code> <em>would</em> be implemented automatically by the compiler for anything that implements <code>Pointee</code>.</p>
<h2>Syntax opt-in</h2>
<p>The RFC proposes that an explicit bound like <code>T: MetaSized</code> disabled the default <code>T: Sized </code> bound. However, this gives no signal that this trait bound is &quot;special&quot; or different than any other trait bound. Naming conventions can help here, signalling to users that these are special traits, but that leads to constraints on naming and may not scale as we consider using this mechanism to relax other defaults as proposed in <a href="https://smallcultfollowing.com/babysteps/blog/2025/10/21/move-destruct-leak/">my recent blog post</a>. One idea is to use some form of syntax, so that <code>T: MetaSized</code> is just a regular bound, but (for example) <code>T: =MetaSized</code> indicates that this bound &quot;disables&quot; the default <code>Sized</code> bound. This gives users some signal that something special is going on. This <code>=</code> syntax is borrowing from semver constraints, although it's not a precise match (it does not mean that <code>T: Sized</code> doesn't hold, after all). Other proposals would be some other sigil (<code>T: ?MetaSized</code>, but it means &quot;opt out from the traits above you&quot;; <code>T: #MetaSized</code>, ...) or a keyword (no idea).</p>
<p>To help us get a feel for it, I'll use <code>T: =Foo</code> throughout this post.</p>
<h2>Implicit trait supertrait bounds, edition interaction</h2>
<p>In Rust 2024, a trait is implicitly <code>?Sized</code> which gets mapped to <code>=SizeOfVal</code>:</p>
<pre><code class="language-rust">trait Marker {} // cannot be implemented by extern types
</code></pre>
<p>This is not desirable but changing it would be backwards incompatible if traits have default methods that take advantage of this bound:</p>
<pre><code class="language-rust">trait NotQuiteMarker {
    fn dummy(&amp;self) {
        let s = size_of_val(self);
    }
}
</code></pre>
<p>We need to decide how to handle this. Options are</p>
<ul>
<li>Just change it, breakage will be small (have to test that).</li>
<li>Default to <code>=SizeOfVal</code> but let users explicitly write <code>=Pointee</code> if they want that. Bad because all traits will be incompatible with extern types.</li>
<li>Default to <code>=SizeOfVal</code> only if defaulted methods are present. Bad because it's a backwards incompatible change to add a defaulted method now.</li>
<li>Default to <code>=Pointee</code> but add <code>where Self: =SizeOfVal</code> implicitly to defaulted methods. Now it's not backwards incompatible to add a new defaulted method, but it is backwards incompatible to change an existing method to have a default.</li>
</ul>
<p>If we go with one of the latter options, Niko proposes that we should relax this in the next Edition (Rust 2026?) so that the default becomes <code>Pointee</code> (or maybe not even that, if we can).</p>
<h2>Relaxing associated type bounds</h2>
<p>Under the RFC, existing <code>?Sized</code> bounds would be equivalent to <code>=SizeOfVal</code>. This is mostly fine but will cause problems in (at least) two specific cases: closure bounds and the <code>Deref</code> trait. For closures, we can adjust the bound since the associated type is unstable and due to the peculiarities of our <code>Fn() -&gt; T</code> syntax. Failure to adjust the Deref bound in particular would prohibit the use of <code>Rc&lt;E&gt;</code> where <code>E</code> is an extern type, etc.</p>
<p>For deref bounds, <a href="https://github.com/davidtwco">David Wood</a> is preparing a PR that simply changes the bound in a backwards incompatible way to assess breakage on crater. There is some chance the breakage will be small.</p>
<p>If the breakage proves problematic, or if we find other traits that need to be relaxed in a similar fashion, we do have the option of:</p>
<ul>
<li>In Rust 2024, <code>T: Deref</code> becomes equivalent to <code>T: Deref&lt;Target: SizeOfVal&gt;</code> unless written like <code>T: Deref&lt;Target: =Pointee&gt;</code>. We add that annotation throughout stdlib.</li>
<li>In Rust 202X, we change the default, so that <code>T: Deref</code> does not add any special bounds, and existing Rust 2024 <code>T: Deref</code> is rewritten to <code>T: Deref&lt;Target: SizeOfVal&gt;</code> as needed.</li>
</ul>
<h2>Other notes</h2>
<p>One topic that came up in discussion is that we may eventually wish to add a level &quot;below&quot; <code>Pointee</code>, perhaps <code>Value</code>, that signifies webassembly external values which cannot be pointed at. That is not currently under consideration but should be backwards compatible.</p>

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

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="4"></progress>
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
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Unsafe Fields <a href='https://github.com/rust-lang/rust-project-goals/issues/273' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#273)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: var(--background-color);">
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
