# Releasing

1. Determine the next version.
   We follow (to the best of our ability) [semantic versioning](https://semver.org/).
2. Create a new branch, `git checkout -b release/vX.Y.Z`
3. Update [Cargo.toml](./Cargo.toml) with the new version
4. Update and audit [CHANGELOG.md](./CHANGELOG.md)
5. `git push -u origin`
6. Open a PR
7. When merged, tag the merge commit with the version number: `git tag -as vX.Y.Z -m vX.Y.Z`
8. Push the tag: `git push origin vX.Y.Z`
9. Create a new release
