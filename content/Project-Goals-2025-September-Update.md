+++
path = "9999/12/31/project-goals-update--september-2025"
title = "Project goals update — September 2025"
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Design a language feature to solve Field Projections <a href='https://github.com/rust-lang/rust-project-goals/issues/390' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#390)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: white;">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3326839670" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @BennoLossin posted on 2025-09-24:</a>
<div style="margin-top: 8px; padding: 12px; background: #f8f9fa; border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<h2>Key Developments</h2>
<ul>
<li>coordinating with <code>#![feature(pin_ergonomics)]</code> (https://github.com/rust-lang/rust/issues/130494) to ensure compatibility between the two features (allow custom pin projections to be the same as the ones for <code>&amp;pin mut T</code>)</li>
<li>identified connection to auto reborrowing
<ul>
<li>https://github.com/rust-lang/rust-project-goals/issues/399</li>
<li>https://github.com/rust-lang/rust/issues/145612</li>
</ul>
</li>
<li>held a <a href="https://hackmd.io/PgVxFwBDQlGXPGTQrI0i3A">design meeting</a>
<ul>
<li>very positive feedback from the language team</li>
<li>approved lang experiment</li>
<li>got a vibe check on design axioms</li>
</ul>
</li>
<li>created a new Zulip channel <a href="https://rust-lang.zulipchat.com/#narrow/channel/522311-t-lang.2Fcustom-refs">#t-lang/custom-refs</a> for all new features needed to make custom references more similar to <code>&amp;T</code>/<code>&amp;mut T</code> such as field projections, auto reborrowing and more</li>
<li>created the <a href="https://github.com/rust-lang/rust/issues/145383">tracking issue</a> for <code>#![feature(field_projections)]</code></li>
<li>opened https://github.com/rust-lang/rust/pull/146307 to implement field representing types (FRTs) in the compiler</li>
</ul>
<h2>Next Steps</h2>
<ul>
<li>Get https://github.com/rust-lang/rust/pull/146307 reviewed &amp; merged</li>
</ul>
<h2>Help Wanted</h2>
<ul>
<li>When the PR for FRTs lands, try out the feature &amp; provide feedback on FRTs</li>
<li>if possible using the <a href="https://github.com/Rust-for-Linux/field-projection/tree/lang-experiment">field-projection crate</a> and provide feedback on projections</li>
</ul>
<h2>Internal Design Updates</h2>
<h3>Shared &amp; Exclusive Projections</h3>
<p>We want users to be able to have two different types of projections analogous to <code>&amp;T</code> and <code>&amp;mut T</code>. Each field can be projected independently and a single field can only be projected multiple times in a shared way. The current design uses two different traits to model this. The two traits are almost identical, except for their safety documentation.</p>
<p>We were thinking if it is possible to unify them into a single trait and have coercions similar to autoreborrowing that would allow the borrow checker to change the behavior depending on which type is projected.</p>
<h3>Syntax</h3>
<p>There are lots of different possibilities for which syntax we can choose, here are a couple options: <code>[Devon Peticolas][]-&gt;f</code>/<code>[Andrea D'Angelo][] x-&gt;f</code>, <code>[Devon Peticolas][].f</code>/<code>[Andrea D'Angelo][] x.f</code>, <code>x.[Fatih Kadir Akın][]</code>/<code>x.mut[Fatih Kadir Akın][]</code>, <code>x.ref.[Fatih Kadir Akın][]</code>/<code>x.[Fatih Kadir Akın][]</code>. Also many alternatives for the sigils used: <code>x[Fatih Kadir Akın][]</code>, <code>x~f</code>, <code>x.@.f</code>.</p>
<p>We have yet to decide on a direction we want to go in. If we are able to merge the two project traits, we can also settle on a single syntax which would be great.</p>
<h3>Splitting Projections into Containers &amp; Pointers</h3>
<p>There are two categories of projections: Containers and Pointers:</p>
<ul>
<li><strong>Containers</strong> are types like <code>MaybeUninit&lt;T&gt;</code>, <code>Cell&lt;T&gt;</code>, <code>UnsafeCell&lt;T&gt;</code>, <code>ManuallyDrop&lt;T&gt;</code>. They are <code>repr(transparent)</code> and apply themselves to each field, so <code>MaybeUninit&lt;MyStruct&gt;</code> has a field of type <code>MaybeUninit&lt;MyField&gt;</code> (if <code>MyStruct</code> has a field of type <code>MyField</code>).</li>
<li><strong>Pointers</strong> are types like <code>&amp;T</code>, <code>&amp;mut T</code>, <code>cell::Ref[Mut]&lt;'_, T&gt;</code>, <code>*const T</code>/<code>*mut T</code>, <code>NonNull&lt;T&gt;</code>. They support projecting <code>Pointer&lt;'_, Struct&gt;</code> to <code>Pointer&lt;'_, Field&gt;</code>.</li>
</ul>
<p>In the current design, these two classes of projections are unified by just implementing <code>Pointer&lt;'_, Container&lt;Struct&gt;&gt; -&gt; Pointer&lt;'_, Container&lt;Field&gt;&gt;</code> manually for the common use-cases (for example <code>&amp;mut MaybeUninit&lt;Struct&gt; -&gt; &amp;mut MaybeUninit&lt;Field&gt;</code>). However this means that things like <code>&amp;Cell&lt;MaybeUninit&lt;Struct&gt;&gt;</code> doesn't have native projections unless we explicitly implement them.</p>
<p>We could try to go for a design that has two different ways to implement projections -- one for containers and one for pointers. But this has the following issues:</p>
<ul>
<li>there are two ways to implement projections, which means that some people will get confused which one they should use.</li>
<li>making projections through multiple container types work out of the box is great, however this means that when defining a new container type and making it available for projections, one needs to consider all other container types and swear coherence with them. If we instead have an explicit way to opt in to projections through multiple container types, the implementer of that trait only has to reason about the types involved in that operation.
<ul>
<li>so to rephrase, the current design allows more container types that users actually use to be projected whereas the split design allows arbitrary nestings of container types to be projected while disallowing certain types to be considered container types.</li>
</ul>
</li>
<li>The same problem exists for allowing all container types to be projected by pointer types, if I define a new pointer type I again need to reason about all container types and if it's sound to project them.</li>
</ul>
<p>We might be able to come up with a sensible definition of &quot;container type&quot; which then resolves these issues, but further investigation is required.</p>
<h3>Projections for <code>&amp;Custom&lt;U&gt;</code></h3>
<p>We want to be able to have both a blanket <code>impl&lt;T, F: Field&lt;Base = T&gt;&gt; Project&lt;F&gt; for &amp;T</code> as well as allow people to have custom projections on <code>&amp;Custom&lt;U&gt;</code>. The motivating example for custom projections is the Rust-for-Linux <code>Mutex</code> that wants these projections for <a href="https://hackmd.io/PgVxFwBDQlGXPGTQrI0i3A#RCU-Read-Copy-Update">safe RCU abstractions</a>.</p>
<p>During the design meeting, it was suggested we could add a generic to <code>Project</code> that only the compiler is allowed to insert, this would allow disambiguation between the two impls. We have now found an alternative approach that requires less specific compiler magic:</p>
<ul>
<li>Add a new marker trait <code>ProjectableBase</code> that's implemented for all types by default.</li>
<li>People can opt out of implementing it by writing <code>impl !ProjectableBase for MyStruct;</code> (needs negative impls for <em>marker traits</em>).</li>
<li>We add <code>where T: ProjectableBase</code> to the <code>impl Project for &amp;T</code>.</li>
<li>The compiler needs to consider the negative impls in the overlap check for users to be able to write their own <code>impl&lt;U, F&gt; Project&lt;F&gt; for &amp;Custom&lt;U&gt; where ...</code> (needs negative impl overlap reasoning)</li>
</ul>
<p>We probably want negative impls for marker traits as well as improved overlap reasoning for different reasons too, so it is probably fine to depend on them here.</p>
<h3><code>enum</code> support</h3>
<p><code>enum</code> and <code>union</code> shouldn't be available for projections by default, take for example <code>&amp;Cell&lt;Enum&gt;</code>, if we project to a variant, someone else could overwrite the value with a different variant, invalidating our <code>&amp;Cell&lt;Field&gt;</code>. This also needs a new trait, probably <code>AlwaysActiveField</code> (needs more name bikeshedding, but too early for that) that marks fields in structs and tuples.</p>
<p>To properly project an <code>enum</code>, we need:</p>
<ul>
<li>a new <code>CanProjectEnum</code> (TBB) trait that provides a way to read the discriminant that's currently inhabiting the value.
<ul>
<li>it also needs to guarantee that the discriminant doesn't change while fields are being projected (this rules out implementing it for <code>&amp;Cell</code>)</li>
</ul>
</li>
<li>a new <code>match</code> operator that will project all mentioned fields (for <code>&amp;Enum</code> this already is the behavior for <code>match</code>)</li>
</ul>
<h3>Field Representing Types (FRTs)</h3>
<p>While implementing https://github.com/rust-lang/rust/pull/146307 we identified the following problems/design decisions:</p>
<ul>
<li>a FRT is considered local to the orphan check when each container base type involved in the field path is local or a tuple (see the top comment on the PR for more infos)</li>
<li>FRTs cannot implement <code>Drop</code></li>
<li>the <code>Field</code> trait is not user-implementable</li>
<li>types with fields that are dynamically sized don't have a statically known offset, which complicates the <code>UnalignedField</code> trait,</li>
</ul>
<p>I decided to simplify the first implementation of FRTs and restrict them to sized structs and tuples. It also doesn't support packed structs. Future PRs will add support for enums, unions and packed structs as well as dynamically sized types.</p>

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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>


### "Flexible, fast(er) compilation"

<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
build-std <a href='https://github.com/rust-lang/rust-project-goals/issues/274' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#274)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: white;">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-3285625773" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @adamgemmell posted on 2025-09-12:</a>
<div style="margin-top: 8px; padding: 12px; background: #f8f9fa; border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Recently we've been working on feedback on the multi-staged format of the RFC. We've also shared the RFC outside of our sync call group to people from a variety of project teams and potential users too.</p>
<p>We're now receiving feedback that is much more detail-oriented, as opposed to being about the direction and scope of the RFC, which is a good indication that the overall strategy for shipping this RFC seems promising. We're continuing to address feedback to ensure the RFC is clear, consistent and technically feasible. David's feeling is that we've probably got another couple rounds of feedback from currently involved people and then we'll invite more people from various groups before publishing parts of the RFC formally.</p>

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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<div style="padding: 12px 16px; background: #fff3cd; border-bottom: 1px solid #eee; border-left: 4px solid #ffc107;">
<strong>Help wanted:</strong> <p>Help test the deadlock code in the <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3AA-parallel-compiler">issue list</a> and try to reproduce the issue</p>

</div>

<!-- Updates Section -->
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: white;">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/121#issuecomment-3300782390" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @SparrowLii posted on 2025-09-17:</a>
<div style="margin-top: 8px; padding: 12px; background: #f8f9fa; border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<ul>
<li><strong>Key developments:</strong> We have added more tests for deadlock issues. And we can say that deadlock problems are almost resolved. And we are currently addressing issues related to reproducible builds, and some of these have already been resolved.</li>
<li><strong>Blockers:</strong> null</li>
<li><strong>Help wanted:</strong> Help test the deadlock code in the <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3AA-parallel-compiler">issue list</a> and try to reproduce the issue</li>
</ul>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Relink don&#x27;t Rebuild <a href='https://github.com/rust-lang/rust-project-goals/issues/400' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#400)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: white;">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/119#issuecomment-3300180863" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @epage posted on 2025-09-16:</a>
<div style="margin-top: 8px; padding: 12px; background: #f8f9fa; border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Key developments:</p>
<ul>
<li>Overall polish
<ul>
<li>https://github.com/rust-lang/rust/pull/145751</li>
<li>https://github.com/rust-lang/rust/pull/145754</li>
<li>https://github.com/rust-lang/rust/pull/146106</li>
<li>https://github.com/rust-lang/rust/pull/146137</li>
<li>https://github.com/rust-lang/rust/pull/146211</li>
<li>https://github.com/rust-lang/rust/pull/146340</li>
<li>https://github.com/rust-lang/rust/pull/145568</li>
<li>https://github.com/rust-lang/cargo/pull/15878</li>
<li>https://github.com/rust-lang/cargo/pull/15886</li>
<li>https://github.com/rust-lang/cargo/pull/15899</li>
<li>https://github.com/rust-lang/cargo/pull/15914</li>
<li>https://github.com/rust-lang/cargo/pull/15927</li>
<li>https://github.com/rust-lang/cargo/pull/15939</li>
<li>https://github.com/rust-lang/cargo/pull/15952</li>
<li>https://github.com/rust-lang/cargo/pull/15972</li>
<li>https://github.com/rust-lang/cargo/pull/15975</li>
</ul>
</li>
<li>rustfmt work
<ul>
<li>https://github.com/rust-lang/rust/pull/145617</li>
<li>https://github.com/rust-lang/rust/pull/145766</li>
</ul>
</li>
<li>Reference work
<ul>
<li>https://github.com/rust-lang/reference/pull/1974</li>
</ul>
</li>
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: white;">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/393#issuecomment-3354114815" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @cramertj posted on 2025-09-30:</a>
<div style="margin-top: 8px; padding: 12px; background: #f8f9fa; border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Current status: there is <a href="https://github.com/rust-lang/rfcs/pull/3851">an RFC for <code>auto impl</code> supertraits</a> that has received some discussion and updates (thank you, <a href="https://github.com/dingxiangfei2009">Ding Xiang Fei</a>!).</p>
<p>The major open questions currently are:</p>
<h2>Syntax</h2>
<p>The current RFC proposes:</p>
<pre><code class="language-rust">trait Subtrait: Supertrait {
    auto impl Supertrait {
        // Supertrait items defined in terms of Subtrait items, if any
    }
}
</code></pre>
<p>Additionally, there is an open question around the syntax of <code>auto impl</code> for unsafe supertraits. The current proposal is to require <code>unsafe auto impl Supertrait</code>.</p>
<h2>Whether to require impls to opt-out of <code>auto impl</code>s</h2>
<p>The current RFC proposes that</p>
<pre><code class="language-rust">impl Supertrait for MyType {}

impl Subtrait for MyType {
    // Required in order to manually write `Supertrait` for MyType.
    extern impl Supertrait;
}
</code></pre>
<p>This makes it explicit via opt-out whether an <code>auto impl</code> is being applied. However, this is in conflict with the goal of allowing <code>auto impl</code>s to be added to existing trait hierarchies. The RFC proposes to resolve this via a temporary attribute which triggers a warning. See my comment <a href="https://github.com/rust-lang/rfcs/pull/3851#discussion_r2393019242">here</a>.</p>
<p>Note that properly resolving whether or not to apply an <code>auto impl</code> <a href="https://github.com/rust-lang/rfcs/pull/3851#discussion_r2393031851">requires coherence-like analysis</a>.</p>

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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Stabilizable Polonius support on nightly <a href='https://github.com/rust-lang/rust-project-goals/issues/118' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#118)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Continue resolving &#x60;cargo-semver-checks&#x60; blockers for merging into cargo <a href='https://github.com/rust-lang/rust-project-goals/issues/104' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#104)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: white;">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/104#issuecomment-3310128638" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @obi1kenobi posted on 2025-09-19:</a>
<div style="margin-top: 8px; padding: 12px; background: #f8f9fa; border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Just removed the duplicate posts, guessing from a script that had a bad day.</p>

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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Emit Retags in Codegen <a href='https://github.com/rust-lang/rust-project-goals/issues/392' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#392)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Expand the Rust Reference to specify more aspects of the Rust language <a href='https://github.com/rust-lang/rust-project-goals/issues/394' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#394)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Finish the libtest json output experiment <a href='https://github.com/rust-lang/rust-project-goals/issues/255' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#255)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: white;">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/255#issuecomment-3300207747" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @epage posted on 2025-09-16:</a>
<div style="margin-top: 8px; padding: 12px; background: #f8f9fa; border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Key developments:</p>
<ul>
<li>libtest2
<ul>
<li>libtest env variables were deprecated, reducing the API surface for custom test harnesses, https://github.com/rust-lang/rust/pull/145269</li>
<li>libtest2 was updated to reflect deprecations</li>
<li>https://github.com/assert-rs/libtest2/pull/105</li>
<li>libtest2 is now mostly in shape for use</li>
</ul>
</li>
<li>json schema
<ul>
<li>https://github.com/assert-rs/libtest2/pull/107</li>
<li>https://github.com/assert-rs/libtest2/pull/108</li>
<li>https://github.com/assert-rs/libtest2/pull/111</li>
<li>https://github.com/assert-rs/libtest2/pull/120</li>
<li>starting exploration of extension through custom messages, see https://github.com/assert-rs/libtest2/pull/122</li>
</ul>
</li>
</ul>
<p>New areas found for further exploration</p>
<ul>
<li>Failable discovery</li>
<li>Nested discovery</li>
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<details style="border-top: 1px solid #eee;">
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Getting Rust for Linux into stable Rust: compiler features <a href='https://github.com/rust-lang/rust-project-goals/issues/407' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#407)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Getting Rust for Linux into stable Rust: language features <a href='https://github.com/rust-lang/rust-project-goals/issues/116' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#116)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Implement Open API Namespace Support <a href='https://github.com/rust-lang/rust-project-goals/issues/256' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#256)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<details style="border-top: 1px solid #eee;" open>
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: white;">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/264#issuecomment-3300158991" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @epage posted on 2025-09-16:</a>
<div style="margin-top: 8px; padding: 12px; background: #f8f9fa; border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>Key developments:</p>
<ul>
<li>https://github.com/crate-ci/cargo-plumbing/pull/53</li>
<li>https://github.com/crate-ci/cargo-plumbing/pull/62</li>
<li>https://github.com/crate-ci/cargo-plumbing/pull/68</li>
<li>https://github.com/crate-ci/cargo-plumbing/pull/96</li>
<li>Further schema discussions at https://github.com/crate-ci/cargo-plumbing/discussions/18</li>
<li>Writing up https://github.com/crate-ci/cargo-plumbing/issues/82</li>
</ul>
<p>Major obstacles</p>
<ul>
<li>Cargo, being designed for itself, doesn't allow working with arbitrary data, see https://github.com/crate-ci/cargo-plumbing/issues/82</li>
</ul>

</div>
</div>
</div>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Prototype Cargo build analysis <a href='https://github.com/rust-lang/rust-project-goals/issues/398' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#398)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
<tr style="border-bottom: 1px solid #eee;">
<td style="padding: 8px 16px; font-weight: bold; width: 80px; color: #666;">Progress</td>
<td style="padding: 8px 16px;"><progress value="0" max="4"></progress>
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
reflection and comptime <a href='https://github.com/rust-lang/rust-project-goals/issues/406' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#406)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Rework Cargo Build Dir Layout <a href='https://github.com/rust-lang/rust-project-goals/issues/401' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#401)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Run more tests for GCC backend in the Rust&#x27;s CI <a href='https://github.com/rust-lang/rust-project-goals/issues/402' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#402)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;"><p>1 detailed update available.</p>
</span>
</summary>
<div style="padding: 12px 16px; background: white;">
<div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;">
<a href="https://github.com/rust-lang/rust-project-goals/issues/275#issuecomment-3301615085" style="color: #0366d6; font-weight: 500; text-decoration: none;">Comment by @Jamesbarford posted on 2025-09-17:</a>
<div style="margin-top: 8px; padding: 12px; background: #f8f9fa; border-left: 4px solid #e1e4e8; border-radius: 0 6px 6px 0;">
<p>It is possible to now run the system with two different machines on two different architectures however there is work to be done to make this more robust.</p>
<p>We have worked on ironing out the last bits and pieces for dequeuing benchmarks as well as creating a new user interface to reflect multiple collectors doing work. Presently work is mostly on polishing the UI and handing edge cases through manual testing.</p>
<p><strong>Queue Work:</strong></p>
<ul>
<li>https://github.com/rust-lang/rustc-perf/pull/2212</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2214</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2216</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2221</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2226</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2230</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2231</li>
</ul>
<p><strong>Ui:</strong></p>
<ul>
<li>https://github.com/rust-lang/rustc-perf/pull/2217</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2220</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2224</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2227</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2232</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2233</li>
<li>https://github.com/rust-lang/rustc-perf/pull/2236</li>
</ul>

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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
<div class="goal-card" style="border: 1px solid #ddd; border-radius: 8px; margin: 16px 0; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">

<!-- Goal Title Header -->
<div style="background: #4A90E2; color: white; padding: 12px 16px; font-weight: bold; font-size: 1.1em;">
Type System Documentation <a href='https://github.com/rust-lang/rust-project-goals/issues/405' style="color: #cce7ff; text-decoration: underline; font-size: 0.9em;">(rust-lang/rust-project-goals#405)</a>
</div>

<!-- Goal Information Table -->

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
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

<table style="width: 100%; border-collapse: collapse; background: white;">
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
<summary style="padding: 10px 16px; background: #f5f5f5; cursor: pointer; list-style: none; outline: none;">
<span style="font-weight: bold;">No detailed updates available.</span>
</summary>
</details>

</div>
