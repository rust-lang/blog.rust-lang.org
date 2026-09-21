+++
path = "2026/09/21/github-actions-leaking-secrets-when-miri-output-is-cached"
title = "GitHub Actions leaking secrets when miri output is cached"
authors = ["Manish Goregaokar"]

[extra]
team = "security-response"
team_url = "https://www.rust-lang.org/governance/teams/#team-security-response"
+++


The Rust Security Response Team was notified that Miri stores all environment variables to `target/`, allowing secrets to persist in caches.

While not necessary a vulnerability in and of itself, when paired with GitHub Actions caching behavior, it is possible for this to expose secrets to PRs.


## Overview


GitHub Actions makes it possible to cache directories between runs. Typical setups allow CI runs on `main` (and other branches) to *write* to cache, and PRs can only *read* from cache (preventing cache poisoning). Rust projects tend to speed up CI by caching binaries built by `cargo install` and sometimes the contents of `target/`.

PR CI can be triggered by anyone who can open PRs on your repository. GitHub requires maintainer approval for the *first* PR, but future PRs will rerun CI on every push. Anyone who has previously landed a change can trigger a CI run extracting information from cached `target/` and then cover their tracks by pushing a second commit to the PR.

GitHub sometimes hides overwritten commits in its UI, making this kind of attack harder to detect. CI run logs and overwritten commits are also deleted after a few months.

When `cargo miri` is invoked, miri needs to retain build-relevant environment variables between runs[^1]. The current code to do so achieves this by storing [all environment variables to `target/`](https://github.com/rust-lang/miri/blob/165a9c3c96f0f4e6232278e62cb64648215bbc37/cargo-miri/src/util.rs#L40-L42). This, of course, persists when `target/` is cached. 

If your environment contained secrets, these can now be accessed by PRs via the cache.
 
## Our fix

Our [short term fix][miri-fix] for this is to make miri only preserve `CARGO_*` environment variables (excepting `CARGO_*_TOKEN`) and `OUT_DIR`. In the longer term, miri and cargo may figure out better ways to inform miri of the relevant list of environment variables. Note that this patch may not be available on nightly yet.

We also performed an ecosystem scan of GitHub repositories and identified 1 repository with this issue and 7 repositories that do not appear to be vulnerable but should be cautious anyway. We have reached out to those maintainers.

 [miri-fix]: https://github.com/rust-lang/miri/pull/5337


## Am I affected?

It is likely that our scan was imperfect, so we recommend you check your own GitHub Actions setups if you run miri.

You are vulnerable if:

 * You run `cargo miri` in CI
 * The step that runs `cargo miri` [has access to secrets](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/use-secrets#using-secrets-in-a-workflow) as an environment variable:
     * By being passed in to the step itself as an environment variable
     * By being set in `env` for the workflow
     * By being passed in to a previous step that persists it in the environment somehow
 * The workflow being used caches the `target` directory, usually done via [`actions/cache`](https://github.com/actions/cache) or [`swatinem/rust-cache`](https://github.com/swatinem/rust-cache)
 * The cache is accessible to PRs (common and often the intended use case)

Possible quick fixes include:

 * Disabling cache for that job
 * Scoping secrets to steps in that job that do not call miri
 * Temporarily disabling miri.

Once done, [please clear the cache](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/manage-caches#deleting-cache-entries). Consider rotating any secrets that might have leaked.

The miri release in the upcoming nightly (DATE) will no longer have this problem.


**Even if you do not run miri** we would recommend you ensure that secrets are unavailable to jobs that can influence the content of public caches, since few tools are written under the assumption that the environment contains secrets that must not be leaked to the file system.

## Threat model

We consider it bad practice to have a cache that can easily be tainted by secrets.

If caching `target/`, it is worth making sure that the inputs to processes that create `target/` (anything invoking `cargo`) do not have secrets available. It is generally rare for standard `cargo` build/test subcommands to need any secrets or tokens[^2], so this is mostly a matter of being careful about having secrets exposed as environment variables to the entire job.

Cargo/Miri/Rust does not guarantee that environment variables will be safe from being copied into `target/`. While we are treating this as a security issue and patching it out of an abundance of caution, this is not something you should rely on in general. Beyond official Rust tooling, it is possible for build scripts to be doing things that lead to the environment being stored in compilation artifacts.

## Acknowledgements

Thanks to [Predrag Gruevski](https://github.com/obi1kenobi) of OpenAI for reporting this issue to us.

Issue triage and remediation was performed by Manish Goregaokar, Ralf Jung, Ben Kimock, Weihang Lo, Jacob Finkelman, Walter Pearce, Josh Stone, and Mark Rousskov.


[^1]: Miri is invoked multiple times by `cargo miri`  for complicated reasons
[^2]: In theory it could come up with build scripts reading from the network

