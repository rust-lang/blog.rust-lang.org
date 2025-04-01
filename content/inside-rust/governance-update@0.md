+++
path = "inside-rust/2022/05/19/governance-update"
title = "Governance Update"
authors = ["Ryan Levick and Mara Bos"]
aliases = ["inside-rust/2022/05/19/governance-update.html"]
+++

Last month, the core team, all the leads of top-level teams, the moderators, and the project representatives on the Rust Foundation board jointly sent out an update to all Rust project members on investigations happening into improvements to the governance of the Rust project. 

As is detailed in the update, this report follows extensive conversations with many inside of the project and will continue as we encourage others in the project to reach out should they have feedback or want to get involved. 

Since proper governance of the Rust project impacts all users of Rust, we thought it appropriate to also make this summary public. While typos and grammatical fixes have been made since the original post, the text is otherwise unaltered. [^1]

As is noted in the summary, the next steps are to take the findings we have so far and begin crafting proposals for how the Rust project will be governed in the future. This will eventually lead to RFCs or similar documents with concrete proposals.

[^1]: See [rust-lang/blog.rust-lang.org#974](https://github.com/rust-lang/blog.rust-lang.org/pull/974) for the full history of changes to the original email.

> From: Ryan Levick\
> To: All members of the Rust project\
> Date: Mon, 11 Apr 2022 18:27:00 UTC\
> Subject: Rust Governance Update
>
> Hey @all,
>
> We’d like to provide you with an update to our email from December about the issue that led to the resignation of the moderation team in November. In that email, we indicated that we had three high-level goals:
> 
> * Improve how we handle complex moderation issues.
> * Bring the specific moderation issue at the center of the disagreement to an adequate resolution.
> * Adapt the governance structure of Rust to better handle the needs of our ever growing project.
> 
> In order to accomplish these goals the following actions were taken place:
> 
> * We have established better communication channels between the core team, the moderation team, project representatives on the Rust Foundation, and all top-level team leads. Concretely, we’ve established a Zulip chat with everyone for those groups so that we can quickly and confidently reach consensus on how to approach sensitive items related to project management and governance. This has led to better communication around project management issues and allowed for healthy, productive, and good faith collaboration among all project leads. While there are many different ways we can and will improve how project management is done in the Rust project, it was important that we reach a healthy and productive steady state, and we believe we have been able to do so.
> * We have begun exploring possible avenues forward for improving how we handle complex moderation cases including the specific moderation issue in question. While we won’t be diving into detail about this topic in this email, there has been some early progress on better understanding the complexity of this issue. We expect the moderation team in conjunction with project leads to bring forward concrete proposals in the future though this work is somewhat blocked on the next point.
> * In pursuit of the ultimate goal of designing and implementing a revised governance structure for Rust, we have been talking with project members to try to deeply understand the requirements of any such governance structure. This process has started with many different conversations with folks from across the project including over 20 formal interviews of project leaders on the needs of Rust project governance. The remainder of this email will be an attempt to outline our initial findings as well as next steps and opportunities for those who are interested to get more involved.
> 
> ### Your opportunity to participate
> 
> First, while we believe we are honing in on a clear understanding of the needs for Rust governance, the conversation is not (and likely will never be) over. If you’d like to participate yourself, you can find the interview questions we have used [here](https://hackmd.io/@ryanlevick/By2RKtMec). Write your answers in whatever form you wish, and send them back to Mara or me so they can be incorporated into our notes. Please note that your answers will not be shared with anyone and will only be reflected in documents that attempt to summarize overall feedback from project members.
> 
> We encourage anyone interested in participating to complete the interview before reading the following summary of our findings so far. While every interview brings us closer to a complete understanding of the project management needs of the Rust project, nothing is final yet, and all project members will have the opportunity to provide feedback on any concrete proposals brought forth (most likely in the form of RFCs).
> 
> ## Requirements
> 
> The following is a list of requirements for Rust governance. Some of these requirements are already met by the current governance structure.
> 
> * **Independent and autonomous teams**: Almost all decision making should be made at the team level. Teams should have the ability to decide their own directions and have the ultimate authority over their respective piece of the project.
> * **Cross team collaboration**: Collaboration between teams should be very high. This ensures teams can make decisions in the context of the larger project and not just within their own silo.
> * **Accountable teams**: Teams should be held accountable to each other for their work. There should be mechanisms for ensuring teams are meeting other team’s expectations and fulfilling their obligations to other teams.
> * **Centralized leadership**: While the majority of decisions are owned by and made by independent teams, some decisions affect the entire project. Any such decisions that are not in the purview of one or more specific teams are made by a centralized, accountable leadership body.
> * **Accountable leadership**: The leadership body should be a structure that has representation and buy-in from all project teams. Additionally, the membership of this group should be chosen transparently, should have large buy-in from other project members, and should be at least reaffirmed relatively often. Lastly, membership in the leadership group requires participation in the wider project and therefore membership in leadership cannot be the exclusive way for someone to participate in the project.
> * **Explicit and well-defined structure**: How the project is structured, who the decision makers are, and how decisions are made should be relatively transparent to both project participants and outsiders. While informal relationships and other soft power mechanisms exist, most collaboration happens transparently and openly.
> * **Structures for project management and administration**: There should be mechanisms independent from decision making that perform functions of project management and administration.
> * **Project leaders are not always technical leaders**: While it may be preferable for some of those who lead technical matters to hold project management leadership positions, being in a leadership position in the project should not require also being in a technical leadership role. In other words, the best language designers don’t necessarily make the best open source project leaders.
> * **Flexibility**: Project structure should be fluid enough to account for the fact that project members always contribute on a volunteer basis (even if they are paid by others for their volunteering), and this means that they are free to leave the project and change what they dedicate their time to whenever they want.
> * **Relatively little bureaucracy**: It’s important that the amount of project governance overhead and bureaucracy be as little as possible.
> * **Approachable by outsiders**: Many outside of the Rust project (e.g., users, potential users, other projects, press, partners, etc.) may wish to engage with the Rust project for a variety of reasons. This process should be clear and straightforward.
> 
> ## Under resourced work
> The following is a list of work that is not receiving the amount of investment that it should be receiving. Our governance structure should ensure that this work receives the proper attention.
> 
> * **Policy development**: Individual teams do a decent job of ensuring they have proper policies, but there are a few problems with this:
>     * Lack of accountability for when a lack of policy might be causing issues
>     * Lack of support in writing policies
> * **Project structure documentation**: the structure of the project has practical implications. For example, representation in leadership, permissions, inclusion in project communications, etc. No team is in charge of ensuring this structure is properly documented
> * **Diversity efforts**: while some teams may take it on themselves to do diversity work, there is no one responsible for this at the project level
> * **Identifying gaps**: Teams do a good job of servicing their own existing needs but sometimes lack the context to know if other needs outside of their purview are being taken care of.
> * **Contributor pipeline improvements**: Teams often do great work to improve their own workflow, but this assumes contributors are able to find the team they want to contribute on.
> * **Conflict resolution**: Project members don’t always agree and in certain situations a positive outcome cannot be reached without the involvement of those outside that conflict.
> * **Supporting team leads**: Some leads might be new to leadership and leadership in a vacuum without the support of others is hard. Team leads should not need to rely on personal, private relationships to get the support they need.
> * **Project self-reflection**: instead of waiting for a crisis to occur before addressing issues, the governance structures should have mechanisms for automatically self correcting (i.e., doing the work that Mara and Ryan are doing now on a more regular basis)
> * **Reporting progress**: there is a *lot* that happens in the Rust project much of which is not reported on. Having some mechanism for better ensuring that everyone has a good overview of what is happening in the project.
> * **Active community management**: actively fostering the culture of the Rust project
> * **Marketing**: the project had previously done more marketing around Rust usage - this work has largely moved to being performed exclusively by individuals
> * **Public relations**: The Rust project has obligations to speak to the outside world (i.e., press, companies, educational institutions, other OSS projects, etc.)
> * **User outreach**: while PR is a push mechanism, the project also needs some sort of pull mechanism for engaging with users and understanding their needs rather than solely relying on the individual insights that contributors bring.
> * **Vision work**: establishing high level project wide goals that cross between team boundaries.
> 
> ## Failure modes
> 
> The following are a list of identified failure modes that we want to ensure we avoid when discussing a new governance structure:
> 
> * **Lack of resources for admin/project management work**: Administrative and project management work is typically less resourced than technical work. Volunteers are typically more drawn towards technical work, and companies tend to fund technical work since they will more easily see a return on that investment. A properly running project is viewed as “table stakes” and thus less likely to see investment without purposeful resource allocation. A failure mode would be not having admin/project management functions be funded and eventually withering away leaving that work not done or done by those who are already busy with other concerns.
> * **Project leadership not held accountable**: Many teams depend on the work of other teams for their own success. When a team feels another team is not delivering on what they need to succeed, this can lead to cascading failures or even outright conflict. Holding others accountable can be difficult because it requires clear expectations, channels for delivering clear and actionable feedback, consequences for consistently not living up to expectations, and mechanisms for handling conflict should it arise.
> * **Project leadership detached from project**: As the project grows in complexity so does that admin/project management overhead. It is possible for a project wide decision making body to lose touch with goings-on in the project as they become busy due to this overhead. There are two ways this can manifest itself: the leadership body fails to keep up with what’s happening in the project and/or the project members lose sight of the leadership body leading to degraded authority. A failure mode would be that the project leadership body becomes detached from the project and the two effectively start acting independently.
> * **An overworked leadership body**: Many of the requirements described above assume a leadership body with the authority to make decisions. Additionally, the leadership body needs to derive its authority from its members involvement in the rest of the project. A possible failure mode is that the leadership body is tasked with more and more responsibility making it harder for its members to keep up with their responsibilities both inside and outside of that leadership body. The more members begin to focus on their work inside the leadership body, the less they can derive their authority from their work outside that body. Additionally, authority should be largely distributed and so an overworked leadership team is a sign of a failure to properly delegate authority.
> * **Lack of delegated authority**: Some administration and project management tasks require a combination of both authority and large amounts of time to be completed. If authority can only be derived through involvement in technical matters in the project, there is a risk that those charged with that work will not be able to do the work. For example, in the list of under resourced work items above both “identifying gaps” and “project self reflection” require a certain level of authority to have the findings make an impact. It would be necessary for the groups doing that work to somehow gain the level of authority needed to get that work done.
> * **Lack of transparency**: Project governance is composed of activities that live on a spectrum of how sensitive in nature they are. Some activities must be kept private as they directly involve the personal matters of individuals. On the other hand, some activities clearly need the involvement of the entire Rust project from the get-go. Many activities live somewhere in between. A potential failure mode is not consistently ensuring that information that can be made public is regularly made so. Even though this can in practice be very difficult and can make it difficult for some to participate in leadership positions, not doing so can lead to diminishing trust in leadership and a growing lack of accountability.
> * **Leadership not consistently held to same standards**: Those in leadership positions should be held at least to the same standards as those in the rest of the project. What’s more, it can be tempting to revise policies and procedures or interpret underspecified ones while they’re being exercised. A failure mode is that it becomes common practice for leadership to hold themselves to different standards leading to an erosion of trust.
> * **Underspecified processes/policies**: It can be tempting to write policy assuming participants have the same assumptions, understandings, etc. This is more flexible, requires less bureaucracy, and is easier to change on the fly. However, such processes/policies are easier to abuse (even without intentional malice). Important processes should be applicable even if the entire project membership is switched out.
> * **Changing of delegated decisions**: A distributed governance structure relies on delegation. However, true delegation means respecting the authority of the party that decision making power has been delegated to. A possible failure mode is constantly second guessing the decisions of teams that have received delegated power. Accountability and oversight are important, but it’s also important that a delegating power not only respect decisions that it would have made itself.
> * **Leadership as popularity contest**: Some systems of governance favor those who are willing to campaign and/or make themselves most visible. However, those most suitable to lead may not necessarily be those who participate in such activities. A possible failure mode is making project leadership a direct function of how popular/well-known someone is within the community or project.
> * **Diffusion of responsibility**: If no one is explicitly responsible for a thing then it won’t necessarily get done, even if the thing not getting done is obviously and clearly causing harm. Without an explicit mechanism for understanding the needs of the project and ensuring that those needs are filled, the project is liable to continue to see important work not getting done.
> 
> I'd like to take this opportunity to thank you for reading this very long email. Once again, if you'd like to participate or give feedback in any form, please do not hesitate to reach out.
> 
> Cheers,\
> Ryan
