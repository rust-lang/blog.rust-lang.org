+++
layout = "post"
title = "Rust Compiler Midyear Report for 2022"
author = "Felix Klock, Wesley Wiser"
description = "The compiler team's midyear report on its ambitions for 2022."
team = "The Compiler Team <https://www.rust-lang.org/governance/teams/compiler>"
+++

# Rust Compiler Midyear Report for 2022

Back in February, the compiler team [posted a collection][ambitions-post] of
concrete initiatives and hopeful aspirations for this year. This post is a
midyear report summarizing the progress so far on all of those items.

[ambitions-post]: /inside-rust/2022/02/22/compiler-team-ambitions-2022.html


As a reminder, there were three main themes we identified in the work items that
the team wanted to tackle: Fulfilling Rust's Promise (🦀), Developer Delight
(👩‍💻) and Contributor Workflow (🛠️). Within those themes, we identified a total
of fourteen work items. Six were concrete initiatives: work items with
developers committed to contributing their time and making forward progress.
Eight were "aspirations": work items that, while of interest to the team, had no
commitment of development resources.

Each work item, regardless of whether it had resources committed or not, had an
associated owner.
To construct this progress report, pnkfelix sent each
owner a survey (you can see what it looks like [here][survey]).
In fact, some work items had multiple subprojects, each with its own owner.
This meant there were actually a total of 22 projects for which we received an update.
This blog post is the compilation (ha ha) of those survey results.

[survey]: https://hackmd.io/Nfs-UmfpQwCiIRHFR-YQfQ?view

## Work Items

Category  | [Concrete Initiatives] |  [Aspirations]
----------|---------------------|-----------
I-unsound (🦀) | [Initiatives][I-unsound Issues]      |
Async Rust (🦀, 👩‍💻)| [Initiatives][Async Initiatives]     |
Debugging (🦀, 👩‍💻)| [Initiatives][Debugging Initiatives] | [Aspirations][Debugging Aspirations]
Faster Builds (👩‍💻, 🛠️) | [Initiatives][Faster Builds Initiatives] | [Aspirations][Faster Builds Aspirations]
Expressiveness (👩‍💻, 🦀) | [Initiatives][Expressiveness Initiatives] | [Aspirations][Expressiveness Aspirations]
Librarification (🛠️) | [Initiatives][Librarification Initiatives]                 | [Aspirations][Librarification Aspirations]
P-high Backlog (🦀) |                             | [Aspirations][P-high Aspirations]
Team Operations (🛠️) |                             | [Aspirations][Team Operations]
Backend (🛠️, 👩‍💻) |                             | [Aspirations][Backend Aspirations]
Diagnostics  (👩‍💻) |                             | [Aspirations][Diagnostics Aspirations]

[Concrete Initiatives]: #concrete-initiatives
[I-unsound Issues]: #i-unsound-issues-
[Async Initiatives]: #async-rust-initiatives--
[Debugging Initiatives]: #debugging-initiatives-
[Faster Builds Initiatives]: #faster-builds-initiatives--%EF%B8%8F
[Expressiveness Initiatives]: #expressiveness-initiatives--
[Librarification Initiatives]: #librarification-initiatives-%EF%B8%8F

[Aspirations]: #aspirations
[P-high Aspirations]: #p-high-aspirations-
[Debugging Aspirations]: #debugging-aspirations-
[Faster Builds Aspirations]: #faster-builds-aspirations--%EF%B8%8F
[Expressiveness Aspirations]: #expressiveness-aspirations--
[Librarification Aspirations]: #librarification-aspirations-%EF%B8%8F
[Team Operations]: #compiler-team-operations-aspirations-%EF%B8%8F
[Backend Aspirations]: #compiler-backend-aspirations-%EF%B8%8F-
[Diagnostics Aspirations]: #diagnostics-aspirations-

## Overall Survey Results

The survey itself had two parts: first, a set of "structured" multiple-choice questions, and second, a set of "unstructured" questions that allowed for free-form answers.

The main reasons I provided the structured questions were two-fold: to make it easy for people to respond (e.g. I expected some ambitions to not require filling out any free-form text at all), and to prime the mindset of the respondant before they entered any free-form answers. I hadn't actually anticipated trying to do formal analysis of the responses.

But, since we *do* have those multiple choice answers available, I took a stab at making a table summarizing them.[^1] That table is below. (You will probably need to zoom out in your web browser to take it all in.)

[^1]: Source code for generating the table rows is at this [gist](https://gist.github.com/pnkfelix/dc4b6875dd31fbf4e0864d8b7dba8dc6)

<table style="width: 100vw; position: relative; left: 50%; right: 50%; margin-left: -49vw; margin-right: -50vw;">

<tr style="position: sticky; top: 0; background: #EBEBEB;"><th></th><th><a href="#async-traits">async traits</a></th><th><a href="#diagnostics-aspirations-">diagnostics improvements</a></th><th><a href="#safe-transmute">safe transmute</a></th><th><a href="#chalk">chalk</a></th><th><a href="#generic-associated-types">Generic Associated Types</a></th><th><a href="#performance-dashboard">Performance Dashboard</a></th><th><a href="#ease-writing-new-backends-via-intrinsic-mir-fallbacks">intrinsic MIR fallbacks</a></th><th><a href="#p-high-backlog-processing-aspirations-">P-high backlog processing</a></th><th><a href="#better-integration-with-trace-based-debuggers">better integration with trace-based debuggers</a></th><th><a href="#mcve-reduction-tooling">MCVE reduction tooling</a></th><th><a href="#incremental-compilation-aspirations">Incremental Compilation</a></th><th><a href="#wg-debugging">wg-debugging</a></th><th><a href="#debugging-aspirations-">Debugging Aspirations</a></th><th><a href="#improving-debuginfo-quality">improving Rust's debuginfo quality</a></th><th><a href="#gcc-backend">GCC backend</a></th><th><a href="#i-unsound-issues-">I-unsound issues</a></th><th><a href="#const-generics-and-const-eval">const-generics and const-eval</a></th><th><a href="#async-crashdump-dissection">async crashdump dissection</a></th><th><a href="#faster-builds-initiatives--%EF%B8%8F">Faster Builds</a></th><th><a href="#mir-tooling-stable-mir-and-ghost-code">MIR tooling</a></th><th><a href="#cranelift">Cranelift</a></th><th><a href="#supporting-split-debuginfo">supporting split debuginfo</a></th></tr>
<tr><th style=color:darkgreen rowspan=2>any-progress?</th><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen>y</td><td style=color:darkgreen> </td></tr>
<tr>                                                      <td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen>n</td><td style=color:darkgreen>n</td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen> </td><td style=color:darkgreen>n</td></tr>
<tr><th style=color:darkorange rowspan=6>problem-size?</th></tr>
<tr>                                                       <td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange><=6mo     </td><td style=color:darkorange>          </td><td style=color:darkorange><=6mo     </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange><=6mo     </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange><=6mo     </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange><=6mo     </td></tr>
<tr>                                                       <td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>>6mo      </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>>6mo      </td><td style=color:darkorange>          </td><td style=color:darkorange>>6mo      </td><td style=color:darkorange>          </td><td style=color:darkorange>>6mo      </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td></tr>
<tr>                                                       <td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>ad-hoc    </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>ad-hoc    </td><td style=color:darkorange>ad-hoc    </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>ad-hoc    </td><td style=color:darkorange>          </td><td style=color:darkorange>ad-hoc    </td><td style=color:darkorange>ad-hoc    </td><td style=color:darkorange>ad-hoc    </td><td style=color:darkorange>          </td><td style=color:darkorange>ad-hoc    </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>ad-hoc    </td><td style=color:darkorange>          </td></tr>
<tr>                                                       <td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>who-knows?</td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td></tr>
<tr>                                                       <td style=color:darkorange>other     </td><td style=color:darkorange>other     </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>other     </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td><td style=color:darkorange>          </td></tr>
<tr><th style=color:maroon rowspan=6>resolved-when?</th></tr>
<tr>                                                    <td style=color:maroon><=6mo  </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon><=6mo  </td><td style=color:maroon><=6mo  </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon><=6mo  </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon><=6mo  </td></tr>
<tr>                                                    <td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon><=2yr  </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon><=2yr  </td><td style=color:maroon><=2yr  </td><td style=color:maroon><=2yr  </td><td style=color:maroon><=2yr  </td><td style=color:maroon><=2yr  </td><td style=color:maroon><=2yr  </td><td style=color:maroon><=2yr  </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon><=2yr  </td><td style=color:maroon>       </td><td style=color:maroon><=2yr  </td><td style=color:maroon>       </td><td style=color:maroon>       </td></tr>
<tr>                                                    <td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>>2yr   </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>>2yr   </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td></tr>
<tr>                                                    <td style=color:maroon>       </td><td style=color:maroon>&infin;</td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>&infin;</td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td></tr>
<tr>                                                    <td style=color:maroon>other  </td><td style=color:maroon>other  </td><td style=color:maroon>other  </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>other  </td><td style=color:maroon>       </td><td style=color:maroon>other  </td><td style=color:maroon>other  </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>       </td><td style=color:maroon>other  </td><td style=color:maroon>       </td></tr>
<tr><th style=color:purple rowspan=7>how-it-started?</th></tr>
<tr>                                                     <td style=color:purple>                        </td><td style=color:purple>had-no-goals            </td><td style=color:purple>                        </td><td style=color:purple>had-no-goals            </td><td style=color:purple>                        </td><td style=color:purple>had-no-goals            </td><td style=color:purple>                        </td><td style=color:purple>had-no-goals            </td><td style=color:purple>                        </td><td style=color:purple>had-no-goals            </td><td style=color:purple>had-no-goals            </td><td style=color:purple>had-no-goals            </td><td style=color:purple>                        </td><td style=color:purple>had-no-goals            </td><td style=color:purple>                        </td><td style=color:purple>had-no-goals            </td><td style=color:purple>had-no-goals            </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>had-no-goals            </td><td style=color:purple>                        </td><td style=color:purple>                        </td></tr>
<tr>                                                     <td style=color:purple>had-no-plan             </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>had-no-plan             </td><td style=color:purple>                        </td><td style=color:purple>had-no-plan             </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>had-no-plan             </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>had-no-plan             </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td></tr>
<tr>                                                     <td style=color:purple>                        </td><td style=color:purple>milestones-unestablished</td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>milestones-unestablished</td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td></tr>
<tr>                                                     <td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>some-accomplishments    </td><td style=color:purple>                        </td><td style=color:purple>some-accomplishments    </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>some-accomplishments    </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>some-accomplishments    </td><td style=color:purple>                        </td></tr>
<tr>                                                     <td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>wrong-plan              </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td></tr>
<tr>                                                     <td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>                        </td><td style=color:purple>just-needed-polish      </td></tr>
<tr><th style=color:darkblue rowspan=10>hows-it-going?</th></tr>
<tr>                                                       <td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>whats-next?           </td><td style=color:darkblue>whats-next?           </td><td style=color:darkblue>                      </td><td style=color:darkblue>whats-next?           </td><td style=color:darkblue>whats-next?           </td><td style=color:darkblue>whats-next?           </td><td style=color:darkblue>whats-next?           </td><td style=color:darkblue>                      </td><td style=color:darkblue>whats-next?           </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td></tr>
<tr>                                                       <td style=color:darkblue>                      </td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>improved-understanding</td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td></tr>
<tr>                                                       <td style=color:darkblue>now-have-plan         </td><td style=color:darkblue>now-have-plan         </td><td style=color:darkblue>now-have-plan         </td><td style=color:darkblue>now-have-plan         </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>now-have-plan         </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>now-have-plan         </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>now-have-plan         </td><td style=color:darkblue>                      </td><td style=color:darkblue>now-have-plan         </td><td style=color:darkblue>                      </td></tr>
<tr>                                                       <td style=color:darkblue>now-have-milestones   </td><td style=color:darkblue>now-have-milestones   </td><td style=color:darkblue>now-have-milestones   </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>now-have-milestones   </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>now-have-milestones   </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>now-have-milestones   </td><td style=color:darkblue>now-have-milestones   </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td></tr>
<tr>                                                       <td style=color:darkblue>                      </td><td style=color:darkblue>have-contributors     </td><td style=color:darkblue>have-contributors     </td><td style=color:darkblue>have-contributors     </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>have-contributors     </td><td style=color:darkblue>have-contributors     </td><td style=color:darkblue>                      </td><td style=color:darkblue>have-contributors     </td><td style=color:darkblue>                      </td><td style=color:darkblue>have-contributors     </td><td style=color:darkblue>have-contributors     </td><td style=color:darkblue>                      </td><td style=color:darkblue>have-contributors     </td><td style=color:darkblue>have-contributors     </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td></tr>
<tr>                                                       <td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>                      </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>                      </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>                      </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>                      </td><td style=color:darkblue>completed-milestones  </td><td style=color:darkblue>                      </td></tr>
<tr>                                                       <td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>implemented-solution  </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>implemented-solution  </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>implemented-solution  </td></tr>
<tr>                                                       <td style=color:darkblue>                      </td><td style=color:darkblue>need-user-feedback    </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>need-user-feedback    </td><td style=color:darkblue>                      </td><td style=color:darkblue>need-user-feedback    </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>need-user-feedback    </td><td style=color:darkblue>need-user-feedback    </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td></tr>
<tr>                                                       <td style=color:darkblue>                      </td><td style=color:darkblue>getting-feedback      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>getting-feedback      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>getting-feedback      </td><td style=color:darkblue>                      </td><td style=color:darkblue>getting-feedback      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td><td style=color:darkblue>                      </td></tr>

</table>

Here are some trends I noted from looking at the table:

The overwhelming majority of our ambitions, 19 out of 22, reported *some*
amount of progress. That's great, especially given that several of those
ambitions were mere aspirations that explicitly had no resources attached to
them at the start of the year.

Less than a quarter of the ambitions, 6 out of 22, said that their overall
problem would be resolved, or the bulk of the problem addressed, within the next
six months. Ten others said they predicted the important components of their
solution would be available within the next two years, so that's a total of 16
out of 22 that expect some kind of solution within two years.

  * Note: One respondent noted that the two year threshold embedded in the
    survey was an "odd timeframe." I do not disagree with that remark. The
    reality is that I was trying to find a happy medium between two extremes:
    short timeframes don't allow enough time for significant work to be
    accomplished, while excessively long timeframes (e.g. greater than five
    years) are very difficult to predict realistically. But the two year
    time frame was drawn somewhat out of a hat.

Of the nine concrete initiatives (i.e. ambitions with development resources attached) that reported some progress so far this year, four said they had no specific goal at the start of the year, and two said they had no plan to achieve their goal.
Similarly, of the ten aspirations (i.e. ambitions without development resources attached) that reported progress, six had no specific goals, and two said they had no plan to achieve their goals.

&nbsp; | No Specific Goals | No Plan | Other
--------|-------------------|--------|--------
Concrete Initiative | 4 | 2 | 3
Aspiration | 6 | 2 | 2
 
 Four of those six concrete initiatives that lacked either specific goals or a plan say they now
have a better understanding of the problem; two of them say they
now have a plan.

Five of the eight aspirations that lacked either specific goals or a plan say they now better understand the problem, and two now have a plan.

I spell this out in this level of detail because I was *expecting* to see some strong correlation between having development resources and having a plan, at least compared to what the aspirations reported. I think the numbers above show that expectation was incorrect. I'm not yet sure what lesson to draw from this, apart from being happy that so many ambitions report that they have a better understanding of their respective problem.

  * Here is a question I want to discuss with the [compiler team and  contributors](https://www.rust-lang.org/governance/teams/compiler) in some future [compiler steering meeting](https://rust-lang.github.io/compiler-team/about/steering-meeting/): Do we, as a
    project, need to put more energy into upfront planning? Or, given our
    volunteer nature, will that just distract people from "the fun stuff" and
    potentially burn out contributors? In other words: is this 4/14 "ambitions
    that spent time making a concrete plan", is that a value we
    should work on growing? Or should we just monitor it, and not actively try to
    change it?

  * Note: It is possible that my survey was itself somewhat flawed, so we need
    to be careful in how we interpret this data. My intention was to try to
    capture how well structured a given ambition's plan was by asking if it had
    established milestones and/or a schedule. However, the survey results
    include cases where the respondent did *not* indicate that they had made a
    plan, but they *did* say that this year they did establish milestones.
    Should I have interpreted that as implicitly saying that there *was* a plan
    established? Or is the respondent saying that something significant was
    accomplished despite the absence of a plan with concrete milestones?
    Maybe we need to restructure future surveys to make these
    "implied bounds" clearer (ha ha, that's a little Rust type system joke).

One more thing I want to point out: five of the ambitions checked the box in the
survey that said "some of our work has reached Rust programmers, but we do not
know if it has improved Rust for them." (This is listed as "need-user-feedback"
in the table above.) We should figure out ways to provide such feedback. I know
that in one case, the lead for diagnostic improvements scours twitter to see if
people are complaining about Rust's error messages (or celebrating them!).
That's one approach, but I think we have to figure out something better. Perhaps
we could provide an opt-in plugin that provide telemetry to the project about the
language and compiler features people use. Or maybe you, gentle reader, have a
better idea to share with us.

Having said that, here are the concrete survey results, edited slightly for
presentation here.

## Concrete Initiatives

### I-unsound issues (🦀)

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#i-unsound-issues-)

<!-- https://hackmd.io/@rust-compiler-team/r1Abdj7uq -->

**Progress:** The Rust project has made progress on this ambition since the start of the year.

**Goals:** we do not think this year’s planned goals for this ambition will be achieved in the next six months, but we do think the most important parts of a solution will be available in the next six months.

**How it started:** at start of 2022, we knew a problem existed, but we did not yet have a specific goal in mind for solving the problem.

**How it's going:** we have milestones that represent individual units of work for the near term that each make progress towards a solution, we have Rust contributors who have agreed to help with the units of work that we have identified, and we have completed some of our milestones.

**Details:** We have fixed a few (some long standing) soundness bugs. Some of them were easy to fix, which was surprising considering how long they were open.

Working on soundness bugs has a high entry cost, as most work requires either a deeper understanding of the type system or it requires spending a significant amount of time on major refactorings. The work is now being done by existing contributors.

An existing Rust contributor who works on fuzzing for ICEs started fuzzing for soundness bugs. Mentoring capabilities are sufficient, considering the lack of mentees.

**Regarding prioritization and focus:** We're mostly addressing technical debt or miri work, and letting soundness bug fixes fall out of that work instead of targeting soundness directly.

Some soundness bugs are notoriously hard to fix, or even impossible in the current compiler.

It is not effective to fix bugs that users will never encounter in practice when they are encountering soundness bugs due to unsafe code much more frequently. So we’ve expanded the focus on miri usability, too, allowing users to find soundness bugs in library code.



### Async Rust Initiatives (🦀, 👩‍💻)

### async traits

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#async-traits)

<!-- https://hackmd.io/@rust-compiler-team/H1eW9oXdc -->

**Progress:** The Rust project has made progress on this ambition since the start of the year.

**Goals:** We had a loosely defined goal of shipping async fn in traits this year. tmandry thinks we will at least have an implementation landed in nightly of static, and possibly dyn, async fn in traits by the end of the year. Stabilization timeline is unclear. We think the most important parts of a solution will be available in the next six months. Major design questions should be resolved or in “resolved to experiment” mode by the end of the year. Nightly-only implementation in the next six months, with part of it possibly moving toward stabilization. Should all be stable in two years or less.

**How it started:** we had a solution or goal in mind, but did not have a plan for how to achieve it.

**How it's going:** we have a high-level end-to-end plan to solve the problem, we have milestones that represent individual units of work for the near term that each make progress towards a solution,  we have Rust contributors who have agreed to help with the units of work that we have identified, and we have completed some of our milestones.

**Details:**

* [Refined impls RFC](https://github.com/rust-lang/rfcs/pull/3245) is in final comment period
* Refactoring work to make async fn easier is ongoing
* We understand the problem of async fn in `dyn` better, and are working to better define the possible design options and potential sticking points with stakeholders on the lang team.

**Regarding new contributors:** For async trait specifically we don't have much for new contributors to do. For wg-async we could probably do a better job of finding new contributors and mentoring them on polish issues.

**Regarding prioritization and focus:** Within wg-async we had one area lead who shifted to different work for reasons related to their job. Overall it seems like everyone is struggling a bit with prioritizing async work over other work their company gives them.

async drop and async closures are the main thing we are not doing since we decided to focus on async traits first. It seemed like the most fundamental and least controversial. On the `dyn` side at least it has proven to be a bit more controversial, but I think it's important to have a firm grasp of how that's going to work before designing other async features.

We may want to focus on shipping static async fn in trait with an experimental proposal for `dyn` being worked on, then shift our attention to async drop next.

If we reached the point where almost no one is blocked by having to use `#[async_trait]`, but many people are blocked by the lack of async drop,
then that would lead us to change our focus.

We have an async stakeholders group that we use for this kind of input, but haven't convened with them in quite some time. We should make an effort to update them and find out where the pain points are again.

### async crashdump dissection

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#async-crashdump-dissection)

<!-- https://hackmd.io/@rust-compiler-team/SJYL9iQ_9 -->

**Progress:** The Rust project has made progress on this ambition since the start of the year, but we do not think this year’s planned goals for this ambition will be achieved in the next six months. We think a solution will be available in the next two years, but not sooner than six months.

**How it started:** At start of 2022, we had a plan for how to achieve a specific goal, but we did not have intermediate progress points (aka “milestones”) established.

**How it's going:** We understand the problem better than we did at the start of the year, and we have completed some of our milestones. Some of our work has reached Rust programmers, but we do not know if it has improved Rust for them

**Details:** @mw thinks we made a lot of progress on the compiler side. `rustc` now encodes most of the information we need for implementing logical stack traces. Only the information about file/line of await points in not readily available.

We also implemented a fairly good testing framework for debugger plugins at:
https://github.com/rust-lang/rust-dbg-ext/tree/main/test-framework/dbt

The two main obstacles are:
- there are some many different debuggers, all with different extension mechanisms of varying capabilities. It's not quite clear how to make use of the rich information we have available in debuginfo.
- A large part of the logic for generating logical stack traces is executor-framework dependent (and there even different versions can need different logic). It's unclear how to best deal with that.

### Debugging Initiatives (🦀)

### wg-debugging

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#debugging-initiatives-)

<!-- https://hackmd.io/@rust-compiler-team/HJOtiiQOq -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** We think this year’s planned goals for this ambition will be achieved in the next six months. (This is based on the "immediate goals" listed in the ambitions blog post.)
We think a solution will be available in the next two years, but not sooner than six months. In general, there is always more we can do here but wesleywiser thinks we’ll have made significant, noticeable progress within the next two years and probably even within this year.

**How it started:** At start of 2022, we knew a problem existed, but we did not yet have a specific goal in mind for solving the problem.

**How it's going:** We would like help deciding what to do next.
We understand the problem better than we did at the start of the year,  we have Rust contributors who have agreed to help with the units of work that we have identified,
and we have completed some of our milestones. Furthermore, (some of) our work has reached Rust programmers. In some cases, we do not know if it has improved Rust for them; in others, what we learn of their usage is informing our plans going forward.

**Details:** wg-debugging has been spun up. We now are conducting regular status/design and triage meetings. We are working through the backlog of A-debuginfo issues. There is active participation from a number of contributors.

### improving debuginfo quality

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#debugging-initiatives-)

<!-- https://hackmd.io/@rust-compiler-team/HJQ25sXOq -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** We had no goals for this ambition planned for this year, but we made ad-hoc progress on the problem itself.
There’s nearly an unbounded amount of effort that could be spent improving debuginfo quality but @wesleywiser thinks we are making significant improvement both over the last 6 months and in the final 6 months of this year as well.

**How it started:** At start of 2022, we knew a problem existed, but we did not yet have a specific goal in mind for solving the problem.

**How it's going:** We would like help deciding what to do next.
We understand the problem better than we did at the start of the year,  we have Rust contributors who have agreed to help with the units of work that we have identified,
Furthermore, (some of) our work has reached Rust programmers. In some cases, we do not know if it has improved Rust for them; in others, what we learn of their usage is informing our plans going forward.
Much of the work @wesleywiser is aware of has landed in 1.60 or 1.61 but there are a few small pieces landing in 1.62 (current beta).

**Details:**
We've made concrete improvements/fixes to debuginfo generation.

Specifically:
- Change char type in debuginfo to DW_ATE_UTF [#89887](https://github.com/rust-lang/rust/pull/89887)
- Fix debuginfo for pointers/references to unsized types [#93006](https://github.com/rust-lang/rust/pull/93006)
- debuginfo: Support fat pointers to unsized tuples. [#94050](https://github.com/rust-lang/rust/pull/94050)
- debuginfo: Fix bug in type name generation for dyn types with associated types but no other generic arguments. [#94810](https://github.com/rust-lang/rust/pull/94810)
- async: Give predictable name to binding generated from .await expressions. [#95011](https://github.com/rust-lang/rust/pull/95011)
- debuginfo: Fix debuginfo for Box&lt;T&gt; where T is unsized. [#95270](https://github.com/rust-lang/rust/pull/95270)
- debuginfo: Emit ZST struct debuginfo for unit type when CPP-like debuginfo is enabled [#96316](https://github.com/rust-lang/rust/pull/96316)

Surprises:
Debuginfo just doesn't have enough test coverage, but that isn't particularly surprising.

**Regarding prioritization and focus:** debugging in general is a top priority for @mw & @wesleywiser’s
team.

### supporting split debuginfo

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#debugging-initiatives-)

<!-- https://hackmd.io/@rust-compiler-team/ByXfjiXu5 -->

**Progress:** The Rust project has not made any progress on this ambition since the start of the year (@pnkfelix: but the free form answer somewhat contradicts this)

**Goals:** As of today, we think this year’s planned goals for this ambition will be achieved in the next six months, and we think the most important parts of a solution will be available in the next six months.

**Where we started:** At the start of 2022, we had completed most of our plan; our main focus was polish and getting the work into the hands of Rust users.

**How it's going:** As of today, we think we have implemented a solution to the problem, but most of our work has not yet landed in hands of Rust users

**Details:** There hasn't been a lot of progress on split debuginfo in 2022 because most of the big pending work that was in-progress landed in late 2021 (namely integration of [thorin](https://github.com/rust-lang/thorin) into rustc to support cross-crate Split DWARF). Since then, Split DWARF has been basically complete and just simmering on nightly; an FCP for stabilization on Linux has completed. Split debuginfo on non-DWARF platforms is already complete and stable.

Progress this year has primarily been some benchmarking of Split DWARF and some work to make split debuginfo an option when bootstrapping rustc (i.e. in `config.toml`), but not a lot else.

Future work is basically just stabilization of `-Csplit-debuginfo` on Linux (Split DWARF); and of the currently-default options for the other platforms (for example, `-Csplit-debuginfo=packed` on Windows requires `-Zunstable-options` despite being effectively the default if you don't specify any flags).

The owner of this work, @davidtwco, intends to stick with the theme of debugging and contribute to the wg-debugging working group, but has also shifted attention to diagnostic translation they see that as an interesting area where they can have impact (and because the remaining implementation tasks for split debuginfo were completed as noted above).

### better integration with trace-based debuggers

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#debugging-initiatives-)

<!-- https://hackmd.io/@rust-compiler-team/BkJSsi7d5 -->

**Progress:** The Rust project has not made any progress on this ambition since the start of the year.

**Goals:** We do not think this year’s planned goals for this ambition will be achieved in the next six months;
we think a solution will be available in the next two years, but not sooner than six months.

**How it started:** At start of 2022, we had a solution or goal in mind, but did not have a plan for how to achieve it.

**How it's going:** We would like help deciding what to do next.

**Details:** @pnkfelix spent a significant portion of 2021 learning about `rr` and
`pernos.co`. They had hoped to spend some of 2022 trying to improve the
experience when using those tools with Rust, but so far @pnkfelix has failed to
allocate sufficient time to make headway here.

One thing that @pnkfelix thinks would be great to deliver would be recreating
`pernos.co`'s click-on-terminal behavior, which jumps to the point in the
control flow where that specific character was emitted to stdout/stderr.

### Faster Builds Initiatives (👩‍💻, 🛠️)

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#faster-builds-initiatives--%EF%B8%8F)

<!-- https://hackmd.io/@rust-compiler-team/B1O2siXd9 -->

**Progress:** The Rust project has made progress on this ambition since the start of the year, and we think this year’s planned goals for this ambition will be achieved in the next six months. (Note that also we think our chosen problem is never-ending and will never be “resolved”.)

**How it started:** at start of 2022, we had a solution or goal in mind, but did not have a plan for how to achieve it.

**How it's going:** we understand the problem better than we did at the start of the year, we have a high-level end-to-end plan to solve the problem, we have milestones that represent individual units of work for the near term that each make progress towards a solution, and we have Rust contributors who have agreed to help with the units of work that we have identified. Furthermore, we have completed some of our milestones, and some of our work has reached Rust programmers, but we do not know if it has improved Rust for them.

**Details:** The [roadmap](https://hackmd.io/YJQSj_nLSZWl2sbI84R1qA) has 22 items on it. Currently progress is currently:
- Completed: 9
- Some progress: 9
- No progress: 4

**Regarding new contributors:** We have had four people show interest.
- @Kobzol has become a highly effective contributor, doing lots of PRs on rustc-perf and rustc, and regularly meeting with @nnethercote and @lqd.
- @martingms has done a few small improvements to rustc, but hasn't had much time available recently.
- @miwig has made a few improvements to rustc-perf's data presentation.
- One other person made initial contact but has done nothing since, even after one follow-up.

**Regarding prioritization and focus:** If finding compiler performance wins gets too difficult, then the owners of this project may shift their focus elsewhere. Its worth noting, however, that one of the owners, @nnethercote, *was* harboring those sorts of feelings before @lqd did their analysis identifying new opportunities for big wins. Thus, better data and analysis was sufficient to enable a new round of progress this time, but that may be harder to repeat in the future.

### Expressiveness Initiatives (👩‍💻, 🦀)

### Generic Associated Types

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#expressiveness-initiatives--)

<!-- https://hackmd.io/@rust-compiler-team/BkM83iX_9 -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** We think this year’s planned goals for this ambition will be achieved in the next six months
and that the most important parts of a solution will likewise be available in the next six months.

**How it started:** At start of 2022, we had a plan, some of which had been completed, but more work remained to be done.

**How it's going:** We have completed some of our milestones. Some of our work has reached Rust users and what we learn of their usage is informing our plans going forward

**Details:** We moved the GATs implementation from “close to ready to stabilize” to “ready to stabilize” - at least in our eyes. This included added a “self outlives lint”, changing the recommended location of GAT where clauses, patching future-compatibility traps, and fixing smaller papercut bugs. After opening a stabilization PR, there was a non-insignificant amount of pushback. In the time sense, we’ve considered how to better message the current state of the implementation - specifically how we see stabilization as a stepping stone in the overall adoption of GATs, with obvious ergonomic and technical (mainly around HRTBs) limitations being future work.

**Regarding new contributors:** There has not been much community involvement in the implementation, but many people have shown up to express their support (or dissent) in the stabilization PR.

**Regarding prioritization and focus:** Since the opening of the stabilization PR and following pushback, progress has been slow. That has, in part, been to try to incorporate work from other projects (NLL, a-mir-formality) into the “stabilization package” - either through direct improvements (from NLL) or a more clear future (through modeling of GATs in a-mir-formality). However, there are other bits of work (writing docs, triaging new issues) that could be done in parallel that have been somewhat partially neglected.

For @jackh726, switching to getting NLL stabilized was a nice change of pace. In a sense, it was “low-hanging fruit” and was a helpful mental break from pushing so hard on GATs for the past year.

If @jackh726 hadn’t been working on GATs for the past year or so, they would have instead been pushing harder on Chalk and librarifcation. In particular, there are fundamental questions, e.g. associated type normalization, to solve there. Recent work with a-mir-formality has started to help answer those. In the meantime, GATs were at a state that they were “unblocked”, had significant interest, and are a requirement for other language (async fns in traits) and lib (LendingIterator) features.

### safe transmute

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#expressiveness-initiatives--)

<!-- https://hackmd.io/@rust-compiler-team/HJ7Y3s7uq -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** We think this year’s planned goals for this ambition will be achieved in the next six months;
we think the most important parts of the problem are now (or very soon to be) solved, and additional parts of the solution will be available in the next six months.

**How it started:** At start of 2022, we had a plan, some of which had been completed, but more work remained to be done. Furthermore, that plan was not sufficient and was in need of revision before it could actually solve the problem.

**How it's going:** we understand the problem better than we did at the start of the year,
we have a high-level end-to-end plan to solve the problem,
we have milestones that represent individual units of work for the near term that each make progress towards a solution
we have Rust contributors who have agreed to help with the units of work that we have identified.
Further more, we have completed some of our milestones; we think we have implemented a solution to the problem, but most of our work has not yet landed in hands of Rust users

**Details:** At the start of the year, we opened [PR #92268](https://github.com/rust-lang/rust/pull/92268), *Initial Implementation of Transmutability Trait*, which aimed to provide the basic functionality of a trait implemented for any two types transmutable into each other (as defined by [MCP #411](https://github.com/rust-lang/compiler-team/issues/411)). This PR required additional testing and polish before it would be ready to merge, but progress unfortunately stalled in the spring.

With the mentoring provided by @oli-obk and an influx of interest and help from @m1el, progress resumed this summer; notably:
- A significant effort in testing revealed flaws in the initial implementation approach. Fortunately, we quickly [discovered](https://rust-lang.zulipchat.com/#narrow/stream/216762-project-safe-transmute/topic/Implementation/near/288584316) and implemented an alternative (and arguably simpler) implementation strategy!
- The `rustc_transmute` crate now only *optionally* depends on other `rustc_*` dependencies, allowing contributors to edit, build, and test the core implementation using the familiar `cargo` commands, rather than building the entire compiler.

[PR #92268](https://github.com/rust-lang/rust/pull/92268) is now undergoing the final polish required for it to be merged, and [near-future units of follow-up work](https://rust-lang.zulipchat.com/#narrow/stream/216762-project-safe-transmute/topic/Implementation/near/290258987) have been identified.

**Regarding new contributors:** An influx of interest and help from @m1el jolted Project Safe Transmute out of its doldrums. Additionally, @joshlf, an early collaborator on Project Safe Transmute, anticipates he will soon be able to rejoin the implementation effort.

**Regarding prioritization and focus:** Personal and professional obligations sapped the capacity of collaborators to contribute. These obligations have been resolved, and progress is being made once again.

@jswrenn's work on as-casting and enum reform has been slowed by their focus on safe transmute. @jswrenn is increasingly able to devote attention to enum reform, but most of their attention remains on project safe transmute; they believe safe transmute is critically important to writing safe, performant code.


### Librarification Initiatives (🛠️)

### Chalk

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#chalk)

<!-- https://hackmd.io/@rust-compiler-team/HyVh2sQdq -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** We had no goals for this ambition planned for this year, but we made ad-hoc progress on the problem itself.
We think a solution will be available in the next two years, but not sooner than six months.

**How it started:** At start of 2022, we knew a problem existed, but we did not yet have a specific goal in mind for solving the problem.

**How it's going:** We understand the problem better than we did at the start of the year.
We have a high-level end-to-end plan to solve the problem.
We have Rust contributors who have agreed to help with the units of work that we have identified.
We have completed some of our milestones.

**Details:** Relatively little work has been made on Chalk itself. However, somewhat recent progress has been made to move `TyKind` to `rustc_type_ir`, which at first glance was a hard problem (and *was* hard, but more doable than originally thought). This unblocks more piecemeal librarification. Other recent work has been done align the Chalk and rustc `TypeFoldable`-related types. Also, initially work to unify rustc and Chalk handling of early-bound parameters has happened, which also can be iteratively progressed. Finally, related work on a-mir-formality has helped to clarify long-term decisions on the direction of Chalk and an independent trait solver.

**Regarding new contributors:** We have had a few new regular and semi-regular contributors make progress on Chalk and librarification, but not any *new* contributors to the Rust Project.

**Regarding prioritization and focus:** This project has not been the main focus of anyone involved (this year).

## Aspirations

### P-high Backlog Processing Aspirations (🦀)

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#p-high-aspirations-)

<!-- https://hackmd.io/@rust-compiler-team/SkS1psm_c -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** We had no goals for this ambition planned for this year, but we made ad-hoc progress on the problem itself
We think a solution will be available in the next two years, but not sooner than six months.

**How it started:** At start of 2022, we knew a problem existed, but we did not yet have a specific goal in mind for solving the problem.

**How it's going:** we understand the problem better than we did at the start of the year
 we have a high-level end-to-end plan to solve the problem
  we have milestones that represent individual units of work for the near term that each make progress towards a solution
   we have completed some of our milestones


**Details:** The Compiler team has instituted a policy that we will schedule a review of the P-high issues approximately every six weeks. The primary goal of the review is to ensure that every P-high issue has an owner, preferably in the form of an assigned working-group (and preferably a working-group that has a structured issue triage process), but it is also acceptable for the owner to be an individual, as long as there is evidence that the issue is not forgotten.


**Regarding prioritization and focus:** We deliberately chose a low-frequency event to drive the review so that we would not be distracting from other work with this backlog processing. That is, we are giving this task some priority, but certainly not the highest priority, that is deliberate.


### Debugging Aspirations (👩‍💻)

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#debugging-aspirations-)

<!-- https://hackmd.io/@rust-compiler-team/SyM2poXu9 -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** we had no goals for this ambition planned for this year, but we made ad-hoc progress on the problem itself
we think a solution will be available in the next two years, but not sooner than six months.

**How it started:** at start of 2022, we had a solution or goal in mind, but did not have a plan for how to achieve it.

**How it's going:** We understand the problem better than we did at the start of the year
 we have completed some of our milestones
  we think we have implemented a solution to the problem, but most of our work has not yet landed in hands of Rust users.

**Details:** We had three specific aspirations identified in the compiler team blog post:

1. Better integration between Cargo and debuggers, specifically in regards to starting and configuring the debugger. 
   - We have not made progress on this aspiration.
2. Improve expression evaluation support in various debuggers.
   - wg-debugging has had discussions during our status & design meetings on how this could be achieved, but we have not made concrete progress on this aspiration.
3. Revise our debugger extension architecture for rendering Rust data structures.
   - [RFC 3191](https://github.com/rust-lang/rfcs/pull/3191) has been approved and implemented on nightly toolchains but not yet stabilized.

Regarding new contributors:
RFC 3191 was written and implemented by a new contributor (@ridwanabdillahi).

Regarding prioritization and focus:
There isn't much focus happening here currently.

### Faster Builds Aspirations (👩‍💻, 🛠️)

### Incremental Compilation Aspirations

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#incremental-compilation-aspirations)

<!-- https://hackmd.io/@rust-compiler-team/B1D7CiX_c -->

**Progress:**
The Rust project has made progress on this ambition since the start of the year

**Goals**:
we had no goals for this ambition planned for this year, but we made ad-hoc progress on the problem itself.
we think a solution will be available in the next two years, but not sooner than six months.

**How it started**:
at start of 2022, we knew a problem existed, but we did not yet have a specific goal in mind for solving the problem.

**How it's going**:
we would like help deciding what to do next
we have Rust contributors who have agreed to help with the units of work that we have identified

**Details:**



We have refactored the global HIR visitor to reduce the amount of accesses to HIR. Using this, we managed to reduce incremental reuse and avoid some recomputations.

We have laid down the first steps for implementing “salsa jars” in the rustc query system. Those first steps are being implemented by a contributor, but the future plans are not fully designed yet.

We have established the constraints for extending the query system with “salsa entities”. The design is much more subtle than anticipated, and will take longer than planned.

Integration of lowering into the query system is still in review. This blocks progress on making name resolution incremental.


Regarding new contributors:
The progress has been made in part thanks to @kckeiks.


Regarding prioritization and focus:
One of the owners, @cjgillot started a large refactor of lifetime resolution.
This refactor allowed for faster progress in fixing a few old bugs.



### Inter-crate Sharing Aspirations

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#inter-crate-sharing-aspirations)

<!-- https://hackmd.io/zad1BjIoR8S4IfE49z-xYg -->
<!-- (no survey response as of 2022-07-19) -->

No survey response was provided; after following up, we confirmed with the
owners that nothing has happened here, and that progress is unlikely in the near
term because this would almost certainly require changes to Cargo, where changes
are hard to push forward right now due to lack of reviewing capacity.

### Expressiveness Aspirations (🦀, 👩‍💻)

### const generics and const eval

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#expressiveness-aspirations--)

<!-- https://hackmd.io/@rust-compiler-team/HktiComdq -->

**Progress:** The Rust project has made progress on this ambition since the start of the year.

**Goals:** We had no goals for this ambition planned for this year, but we made ad-hoc progress on the problem itself. We do not think a solution will be achieved in the next two years, but we do think it is a solvable problem

**How it started:** at start of 2022, we knew a problem existed, but we did not yet have a specific goal in mind for solving the problem.

**How it's going:** we understand the problem better than we did at the start of the year, and we have Rust contributors who have agreed to help with the units of work that we have identified.

**Details:** We have started to document the design and implementation challenges of const generics [on GitHub](https://github.com/rust-lang/project-const-generics/issues?q=is%3Aissue+is%3Aopen+label%3AC-design-docs). We are generally spending some time to improve our understanding of the overall design.

**Regarding new contributors:** Yes, for const generics we have 1 new major contributor, and we currently have enough mentoring capacity to support them.

**Regarding prioritization and focus:** @lcnr has shifted to looking into general type system cleanups, because they believe that improving both the type system itself, and their understanding of it, will improve the experience of working on const generics and prevent us from making unnecessary mistakes in the future.

### Librarification Aspirations (🛠️)

### MIR tooling (stable-mir and ghost-code)

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#mir-tooling)

<!-- https://hackmd.io/CWcHnbOWReSUyPv0ZDdWaw -->

**Progress:** The Rust project has made progress on this ambition since the start of the year, but we do not think this year’s planned goals for this ambition will be achieved in the next six months.

**Goals:** We think a solution will be available in the next two years, but not sooner than six months.

**How it started:** at start of 2022, we knew a problem existed, but we did not yet have a specific goal in mind for solving the problem.

**How it’s going:** we understand the problem better than we did at the start of the year, we have milestones that represent individual units of work for the near term that each make progress towards a solution, and we have Rust contributors who have agreed to help with the units of work that we have identified.

**Details:** We have started two initiatives related to MIR: ‘Project Stable MIR’ and the ‘Ghost Code Initiative’, which help address the two biggest currently known pain points for external users of MIR.

Project Stable MIR has made initial progress, and has identified key experiments that will enable further work.
This project has also shown that there are many different (and potentially conflicting) needs for a Stable MIR.

The Ghost Code initiative has an initial, high-level plan and volunteer contributors.
Work has not yet started but the we believe that we have a solution.

### Compiler Team Operations Aspirations (🛠️)

#### MCVE reduction tooling

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#mcve-reduction-tooling)

<!-- https://hackmd.io/@rust-compiler-team/r19rJhmu5 -->

**Progress:** The Rust project has not made any progress on this ambition since the start of the year

**Goals:** We had no goals for this ambition planned for this year, and made no progress on the problem it represents
We think a solution will be available in the next two years, but not sooner than six months.

**How it started:** At start of 2022, we knew a problem existed, but we did not yet have a specific goal in mind for solving the problem.

**How it's going:** We would like help deciding what to do next

**Details:** @pnkfelix continues to think that Rust-specific tooling to help contributors with MCVE reduction could be a real boon to the project.
However, they have not dedicated any time to solving the problem.

#### Performance Dashboard

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#performance-dashboard)

<!-- https://hackmd.io/@rust-compiler-team/SyaDJ2X_5 -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** We had no concrete goals for the dashboard, and representing performance is
still a hard problem. @rylev thinks we are unlikely to achieve any significant
milestones in the next 6 months, largely due to lack of clear ideas for solid
improvements or bandwidth to experiment.

We do think the most important parts of a solution will be available in the next six months.
We do not think a complete solution will be achieved in the next two years, but we do think it is a solvable problem
rylev thinks that the real question here is when this becomes something that’s funded and someone wants to focus their work on; that might be never: we’re in an OK state today, but I do think we can do much better (and would likely get some mileage out of it). We’re probably already 75-80% of a ‘perfect’ solution though, I suspect.

**How it started:** at start of 2022, we knew a problem existed, but we did not yet have a specific goal in mind for solving the problem.

**How it's going:** we would like help deciding what to do next
@rylev also wants to emphasize that the largest problem in this domain is room and bandwidth for experimentation.

**Details:** We’re reporting on a few additional metrics in PRs now (cycles, RSS). It’s not really a surprise, but the significant challenge we’ve definitely run into is our audience has such a diverse set of needs that any single representation or comment is likely to be too information dense to be useful; we’re still figuring out how to make the most of the data we have.

**Regarding new contributors:** We’ve had a few folks return to contributing this year (@nnethercote, @lqd), and @rylev believes one new person as well (@Kobzol). There have been a few others with good contributions but not long-duration tenure.

**Regarding prioritization and focus:** Other infrastructure projects have drawn much of @Mark-Simulacrum's attention (crater, triagebot) in the last few months.
We had long-standing debt on Crater and triagebot that needed to be addressed, and performance work was (and is) seeing more investment from other folks so was in less need of direct attention.

@rylev believes that compiler performance remains, in their opinion, the largest and most persistent problem for Rust.

### Compiler Backend Aspirations (🛠️, 👩‍💻)

#### Ease writing new backends via intrinsic MIR fallbacks

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#ease-writing-new-backends)

<!-- https://hackmd.io/@rust-compiler-team/HylMg2m_9 -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** We had no goals for this ambition planned for this year, but we made ad-hoc progress on the problem itself.
We think a solution will be available in the next two years, but not sooner than six months.

**How it started:** At start of 2022, we had a solution or goal in mind, but did not have a plan for how to achieve it.

**How it's going:** We would like help deciding what to do next. Also, it would be wonderful to get
mentoring instructions (even just a sketch of them) so that non-experts could
help make progress on this

**Details:** While we’ve technically made non-zero progress, it’s minimal and unstructured.

**Regarding new contributors:** There are no contributors who have or had committed to making progress on this. A compiler team member did make a step forward on this at one point, but they are unsurprisingly busy with other things.

Mentoring instructions, or even steps with minimal details, might help non-experts pick up work in the area.


#### Cranelift

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#cranelift)

<!-- https://hackmd.io/@rust-compiler-team/BJLre2Xu9 -->

**Progress:** The Rust project has made progress on this ambition since the start of the year.

**Goals:** We had no goals for this ambition planned for this year, but we made ad-hoc progress on the problem itself.
With regards to the overall problem: In terms of getting cg_clif distributed with rustup, @bjorn3 thinks 6 months is feasible. In terms of implementing all missing rust features they think it will take longer.

**How it started:** at start of 2022, we had a plan, some of which had been completed, but more work remained to be done.

**How it's going:** we have a high-level end-to-end plan to solve the problem, and we have completed some of our milestones.

**Regarding new contributors:** There have been a couple minor PR’s. There has also been someone potentially interested in contributing more, but given that most of the low hanging fruit has already been done by @bjorn3, they were not able to help the newcomer find an area to contribute to.

**Regarding prioritization and focus:** @bjorn3 has kept their focus on `cg_clif` and rustc PR’s that are related to `cg_clif`.

If @bjorn3 hadn’t started working on `cg_clif` they probably wouldn’t have gained as much knowledge of the inner workings of rustc as they have now, nor contributed much to rustc. There are some more architectural changes that they want to make to rustc, but they probably wouldn’t get to them in the near future even if they stopped working on `cg_clif`.

With the codegen phase of debug mode builds getting faster with cg_clif in most cases, the bottleneck during debug mode compilation shifts away from codegen to the frontend. @bjorn3 wonders if maybe in the future they could try to see if the performance of the frontend can be improved by the same amount that cranelift improves the backend one way or another?

#### GCC backend

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#gcc-backend)

<!-- https://hackmd.io/T03Nu92NSLeyyYnnW3XFyg -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** we had no goals for this ambition planned for this year, but we made ad-hoc progress on the problem itself
I believe we’ll be able to ship cg_gcc with rustup within 6 months. Implementing the missing Rust features will take longer.

**How it started:** At start of 2022, we had a plan, some of which had been completed, but more work remained to be done.

**How it's going:** We have a high-level end-to-end plan to solve the problem
and  we have completed some of our milestones

**Details:**

* Good support of 128-bit integers on platforms that don't support them.
* Good support of SIMD intrinsics and target-specific builtins.
* The GCC codegen can now bootstrap rustc.
* Added support for packed structs.
* Preparation for a future distribution via rustup with a feature flag allowing to use upstream libgccjit 12.
* Many bug fixes.

**Regarding new contributors:**

* We got around 5 new contributors.
* Around 3 stuck around.

**Regarding prioritization and focus:** @antoyo says some importants things we're not doing because we focus on this project is making the rustc_codegen_ssa API more convenient for the backends other than LLVM. That's something they eventually want to work on.

When the GCC backend is good enough and when the Rust for Linux project is ready to start experimenting with it, it might be worth it to help other areas support new architectures for Rust: adding the arch specs to rustc, adding the support for those architectures in some crates like `libc` and `object`.

### Diagnostics Aspirations (👩‍💻)

[Description from February](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#diagnostics-aspirations-)

<!-- https://hackmd.io/@rust-compiler-team/HJ_cgnXd5 -->

**Progress:** The Rust project has made progress on this ambition since the start of the year

**Goals:** we didn’t have “planned goals” for the year, but we did accomplish lots of incremental improvements and started the translation infrastructure which has been in our wishlist for years.
we think our chosen problem is never-ending and will never be “resolved”;
there is specific feature and integration work that can be tackled in bounded amount of times, but the improvement of diagnostics is a fractal one, and every time we raise the bar, the expectation of our users increases and our increased experience makes us attempt bolder things.

**How it started:** at start of 2022, we knew some problems existed, but we did not yet have a specific goal in mind for solving all such problems.
for other problems, we had a plan for how to achieve a specific goal, but we did not have intermediate progress points (aka “milestones”) established.

**How it's going:** we understand the problem better than we did at the start of the year
we have a high-level end-to-end plan to solve the problem
we have milestones that represent individual units of work for the near term that each make progress towards a solution
we have Rust contributors who have agreed to help with the units of work that we have identified.
Further more, some of our work has reached Rust programmers; in some cases, we do not know if it has improved Rust for them,
and in other cases, what we learn of their usage is informing our plans going forward

**Details:** wg-diagnostics isn't as structured as it could be, which translates to work being taken on piecemeal by individuals at their own pace and leisure. This affects the size of projects that we can tackle, *but* still relatively big projects, like the [translation machinery](https://hackmd.io/@davidtwco/rkXSbLg95) were started and have enough momentum to make progress going forward.

**Regarding new contributors:** @estebank hasn't quantified it, but we've had quite a few new contributors recently. Mentoring capacity isn't depleted, *but* if we wanted to increase the number of people contributing we will need more people. The mentoring needs have been highly variable, depending on the individual (and the complexity of the task they take on).


**Regarding prioritization and focus:** @estebank has seen people come contribute a handful of PRs and disappear, but has not dug deeper into their reasons.

The most common thing is people picking up a project that’s too advanced for them, which demotivates them. We try to both steer them away beforehands and to closely mentor them as they work on things. A cleaner codebase with more machinery for non-standard things (like typechecking opportunistically in the parser, to give an example of something impossible to do today) would allow some of the things people have tried to be done by almost anyone.

@estebank believes that diagnostics are super important and everyone's concern. Efforts like librarification could unblock very powerful heuristics to massively improve our user experience here, but such a project *shouldn't* be started *only* for diagnostics improvements (as there's plenty of things to do already).

What would make @estebank change their focus to something else? Simply put: Seeing no one complain about how hard Rust is to learn and seeing absolutely no new "confusing errors" complaints.

These complaints have dramatically lowered in the last couple of years, but we're not quite there yet. There's also diminishing returns every time we focus on narrower and narrower cases, but making the experience of hitting an extreme corner case a positive one has a knock down effect on how pleasurable the overall experience is: if you're riding a bike in a segregated bike path for miles, but there are 200 yards in the middle where you have to share the road with trucks, the overall experience feels dangerous. Having great E0308 errors is nice for 90% of the cases, but if someone who's trying to push the limits of what can be represented with trait bounds is constantly fighting the compiler (or worse, their users are!), then the experience is soured.


## Conclusion

So, that's the midyear update for T-compiler.

I have one final thought before I sign off on this blog post: my approach this
year for describing our ambitions was very "bottom-up": I surveyed all of the
compiler team and contributors, seeking a full list of everything that was going
on, and everything that we hoped other people might want to help with.

In hindsight, it is a huge list. Furthermore, I did not provide much guidance as
to how I would want these items prioritized. (I had my own pet projects, such as
MCVE reduction, but if anything I *deprioritized* those.)

My reasoning at that time was that by going broad with the presentation, we
would be more likely to attract newcomers who would say "oh, *that* thing sounds
like something I could help with"; especially by given examples of projects that
don't actually require deep knowledge of rustc's implementation. But I do not
think that experiment worked out overall.

There are some things, like Async Traits and Generic Associated Types, that are
hugely important, but nonetheless the implementation effort is falling on the
shoulders of relatively few people. I think in the future, I would like to try
to get the team and contributors to be a little more focused. E.g. maybe try to
convince people to put aside work on ambition A, in order to get more people
working together on ambition B, and then later there will (hopefully) be more
capacity to address A after B has been addressed. But, since this is open
source, it's very much a matter of how much excitement each ambition can build up
around its particular project, and how attractive they can make contribution be
to newcomers.

## FAQ

#### How can I contact an item's owners or sponsor their work on Rust?

This table lists the item owners mentioned above, their [Zulip] username and if they are accepting sponsorships to help them work on Rust:

[Zulip]: https://rust-lang.zulipchat.com/

Owner | Zulip Username | Accepting sponsorships?
-|-|-
[Aaron Hill] | `@Aaron Hill` | No
[antoyo] | `@antoyo` | Yes: [GitHub Sponsors](https://github.com/sponsors/antoyo)
[apiraino] | `@apiraino` | No
[bjorn3] | `@bjorn3` | Yes: [Liberapay](https://liberapay.com/bjorn3)
[cjgillot] | `@cjgillot` | No
[davidtwco] | `@davidtwco` | No: works on Rust at Huawei R&D UK
[estebank] | `@Esteban Küber` | No: works on Rust at Amazon Web Services
[jackh726] | `@Jack Huey` | No
[jswrenn] | `@Jack Wrenn` | No: works on Rust at Amazon Web Services
[lcnr] | `@lcnr` | Yes: [https://lcnr.de/funding/](https://lcnr.de/funding/)
[lqd] | `@lqd` | No: sponsored by the Internet Security Research Group
[Mark-Simulacrum] | `@simulacrum` | No
[michaelwoerister] | `@mw` | No: works on Rust at Microsoft
[nikomatsakis] | `@nikomatsakis` | No: works on Rust at Amazon Web Services
[nnethercote] | `@nnethercote` | No: works on Rust at Futurewei
[oli-obk] | `@oli` | No: works on Rust at Amazon Web Services
[pnkfelix] | `@pnkfelix` | No: works on Rust at Amazon Web Services
[rylev] | `@rylev` | No: works on Rust at Microsoft
[scottmcm] | `@scottmcm` | No
[tmandry] | `@tmandry` | No: works on Rust at Google
[wesleywiser] | `@Wesley Wiser` | No: works on Rust at Microsoft
[xldenis] | `@Xavier Denis` | No

[antoyo]: https://github.com/antoyo
<!-- [antoyo zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/404242-user404242 --> <!-- @**antoyo** -->
<!-- antoyo sponsorship: https://github.com/sponsors/antoyo -->
[Aaron Hill]: https://github.com/Aaron1011
<!-- [Aaron Hill zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/116083-user116083 --> <!-- @**Aaron Hill** -->
<!-- Aaron1011: no affiliation -->
[bjorn3]: https://github.com/bjorn3
<!-- [bjorn3 zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/133247-user133247 --> <!-- @**bjorn3**  -->
<!-- bjorn3 donation page: https://liberapay.com/bjorn3 -->
[cjgillot]: https://github.com/cjgillot
<!-- [cjgillot zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/248906-user248906 --> <!-- @**cjgillot**  -->
<!-- no response from cjgillot re affiliation yet -->
[davidtwco]: https://github.com/davidtwco
<!-- [davidtwco zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/248906-user248906 --> <!-- @**davidtwco**  -->
<!-- davidtwco affiliation: "Huawei R&D UK"-->
[estebank]: https://github.com/estebank
<!-- [estebank zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/119031-user119031 --> <!-- @**Esteban Küber** -->
<!-- estebank affiliation: AWS -->
[lcnr]: https://github.com/lcnr
<!-- [lcnr zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/216206-user216206 --> <!-- @**lcnr** -->
<!-- lcnr sponsorship: https://lcnr.de/funding/ -->
[michaelwoerister]: https://github.com/michaelwoerister
<!-- [michaelwoerister zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/124287-user124287 --> <!-- @**mw** -->
<!-- michaelwoerister affiliation: MS -->
[nikomatsakis]: https://github.com/nikomatsakis
<!-- [nikomatsakis zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/116009-user116009 --> <!-- @**nikomatsakis** -->
<!-- nikomatsakis affiliation: AWS -->
[oli-obk]: https://github.com/oli-obk
<!-- [oli-obk zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/124288-user124288 --> <!-- @**oli** -->
<!-- oli affiliation: AWS -->
[jackh726]: https://github.com/jackh726
<!-- [jackh726 zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/232957-user232957 --> <!-- @**Jack Huey** -->
<!-- jackh726: no affiliation -->
[lqd]: https://github.com/lqd
<!-- [lqd zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/116113-user116113 --> <!-- @**lqd**  -->
<!-- lqd affiliation: ISRG -->
[nnethercote]: https://github.com/nnethercote
<!-- [nnethercote zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/120989-user120989 --> <!-- @**nnethercote**  -->
<!-- nnethercote affiliation: Futurewei -->
[tmandry]: https://github.com/tmandry
<!-- [tmandry zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/116883-user116883 --> <!-- @**tmandry**  -->
<!-- tmandry affiliation: Google (TBD) -->
[scottmcm]: https://github.com/scottmcm
<!-- [scottmcm zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/125270-user125270 --> <!-- @**scottmcm**  -->
<!-- scottmcm: no affiliation -->
[pnkfelix]: https://github.com/pnkfelix
<!-- [pnkfelix zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/116083-user116083  --> <!-- @**pnkfelix**  -->
<!-- pnkfelix affiliation: AWS -->
[wesleywiser]: https://github.com/wesleywiser
<!-- [wesleywiser zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/125250-user125250 --> <!-- @**Wesley Wiser**  -->
<!-- wesleywiser affiliation: MS -->
[jswrenn]: https://github.com/jswrenn
<!-- [jswrenn zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/219211-user219211 --> <!-- @**Jack Wrenn** -->
<!-- jswrenn affiliation: AWS -->
[apiraino]: https://github.com/apiraino
<!-- [apiraino zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/250987-user250987 --> <!-- @**apiraino**  -->
<!-- apiraino: no affiliation -->
[Mark-Simulacrum]: https://github.com/Mark-Simulacrum
<!-- [simulacrum zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/116122-user116122 --> <!-- @**simulacrum**  -->
<!-- simulacrum sponsorship: https://github.com/sponsors/Mark-Simulacrum -->
[rylev]: https://github.com/rylev
<!-- [rylev zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/224872-user224872 --> <!-- @**rylev**  -->
<!-- rylev affiliation: MS -->
[xldenis]: https://github.com/xldenis
<!-- [xldenis zulip PM]: https://rust-lang.zulipchat.com/#narrow/pm-with/312719-user312719 --> <!-- @**Xavier Denis**  -->

