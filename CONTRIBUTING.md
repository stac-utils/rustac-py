# Contributing

First off, thanks for contributing!
We appreciates you.

## Relationship to **stac-utils/rustac**

[stac-utils/rustac](https://github.com/stac-utils/rustac) is a Rust monorepo that provides _most_ of the functionality of this Python package.
It's pretty common that a bug in **rustac-py** is actually a bug in **rustac**.
Knowing which repository to work in can be a little tricky, so don't hesitate to reach out and ask.

## Python environment

It can be a little tricky to ensure that your Python environment is always up-to-date while developing a Rust+Python project.
We're still figuring out the _best_ way, but for now, try this:

```shell
uv sync
python -m maturin_import_hook site install
```

This _should_ make sure your **pytest** runs are picking up the latest changes to your Rust code.

## Testing

We aim for comprehensive unit testing of this library.
Please provide tests for any new features, or to demonstrate bugs.
Draft pull requests with a failing test to demonstrate a bug are much appreciated.

## Submitting changes

Please open a [pull request](https://docs.github.com/en/pull-requests) with your changes -- make sure to include unit tests.
Please follow standard git commit formatting (subject line 50 characters max, wrap the body at 72 characters).

We use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).
Your commits do not have to but if you'd like to format them this way, we would be grateful.

If you can, use `git rebase -i` to create a clean, well-formatted history before opening your pull request.
If you need to make changes after opening your pull request (e.g. to fix CI breakages) we will be grateful if you squash those fixes into their relevant commits.

Thanks so much!

-Pete Gadomski
