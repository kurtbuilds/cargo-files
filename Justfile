install:
    cargo install --path cargo-files

# Bump version. level=major,minor,patch
version level:
    git diff-index --exit-code HEAD > /dev/null || ! echo You have untracked changes. Commit your changes before bumping the version.
    cargo set-version --bump {{level}}
    cargo update # This bumps Cargo.lock
    VERSION=$(rg  "version = \"([0-9.]+)\"" -or '$1' cargo-files/Cargo.toml | head -n1) && \
        git commit -am "Bump version to $VERSION" && \
        git tag v$VERSION && \
        git push origin v$VERSION
    git push

#publish:
#    cargo publish

test:
    cargo test

patch: test
    just version patch
    # just publish
