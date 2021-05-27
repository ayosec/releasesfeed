# releasesfeed

releasesfeed generates a [RSS feed](https://en.wikipedia.org/wiki/RSS) with the
latest releases from the starred repositories of a GitHub account.

## Usage

### Installation

You can download a precompiled binary from the [releases page].

If you want to build by yourself you have to install the [Rust compiler]. Then,
download the source code of this repository, and build it with Cargo:

```console
$ git clone https://github.com/ayosec/releasesfeed.git

$ cd releasesfeed

$ cargo build --release
```

The final binary is `target/release/releasesfeed`.

[releases page]: https://github.com/ayosec/releasesfeed/releases
[Rust compiler]: https://www.rust-lang.org/tools/install

### Access Tokens

The application needs an [access token] to use the GitHub API. This token does
not need any permission, it is used to avoid the rate limits of anonymous
requests.

To create a new access token:

1. Go to [Profile settings / Developer settings / Personal access tokens](https://github.com/settings/tokens).
2. Click on the *Generate new token* button.
3. Type a name for the token, and leave the *scopes* list empty.
4. Click on *Generate token*, and copy the secret value.

[access token]: https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token

### Run

The tokens are read from the environment variable `RF_GITHUB_TOKENS`. This
variable is a list of `username:token` pairs, separated by `;`.

For example, the token `ghp_ABCDEF` for the GitHub user `alice` can be used
with something like this:

```console
$ cat > tokens.secret
alice:ghp_ABCDEF
^D

$ RF_GITHUB_TOKENS=$(cat tokens.secret) releasesfeed > rss-feed.xml
```

If you want to generate a feed using starred repositories from multiple
accounts, you have to join the pairs with `;`. For example, to use `alice` and
`bob` users:

```console
$ cat > tokens.secret
alice:ghp_ABCDEF;bob:ghp_012345
^D
```

By default, it includes the 50 newest releases. This value can be modified with
the `RF_RELEASES_COUNT` variable.

## Usage in GitHub Actions

It is possible to generate a feed for your starred repositories using GitHub
Actions.

You need a new repository, which is used only to generate the feed and make it
available as a [GitHub Pages](https://pages.github.com/) site.

You also need to create an access token, as described above, and register it in
the new repository as a [repository secret] with the name `RF_GITHUB_TOKENS`
and the value `username:token`.

When the secret is set, copy the [`deploy/workflow.yaml`](./deploy/workflow.yaml)
file to `.github/workflows/update.yaml` in your repository. Push the changes and
wait until the first run of the workflow is done. You can follow the progress of
the run in the *Actions* tab of your repository.

If the workflow is completed successfully you will have a `gh-pages` branch in
your repository. Finally, you have to [configure a source] for the GitHub Pages
site. Follow the official documentation to enable the `gh-pages` as the source.

Your feed will be available in `https://$USER.github.io/$REPOSITORY/feed.xml`.
This is URL can be added to any news aggregator with support for RSS.

The workflow is executed every hour, so the feed is updated automatically.

[repository secret]: https://docs.github.com/en/actions/reference/encrypted-secrets
[configure a source]: https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site
