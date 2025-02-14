# Cutting a release

We rely on [`release-plz`](https://release-plz.dev/docs) for our release automation.\
To cut a new release, trigger the [Release PR workflow](https://github.com/mainmatter/eserde/actions/workflows/release-pr.yml).\
It'll open a new PR for you to review, including changelog updates. When you merge it,
the [Publish workflow](https://github.com/mainmatter/eserde/actions/workflows/publish.yml) will kick off, publishing the
crates to [crates.io](https://crates.io).
