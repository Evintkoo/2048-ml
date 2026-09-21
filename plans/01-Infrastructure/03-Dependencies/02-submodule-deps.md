# Submodule Dependency Management

## 1. Submodule Overview

The automl repository is included as a git submodule. This section defines how to manage updates, compatibility, and version pinning.

## 2. Submodule Initialization

```bash
# Clone with submodule
git clone --recurse-submodules https://github.com/Evintkoo/2048-ml

# Or initialize after clone
git submodule update --init --recursive
```

## 3. Version Pinning

The submodule commit is pinned at a specific hash for reproducibility:

```bash
# Check current submodule commit
git submodule status automl

# Pin to specific commit
cd automl && git checkout <commit-hash> && cd ..
git add automl
```

## 4. Update Strategy

| Update Type | Frequency | Procedure |
|-------------|-----------|-----------|
| Patch | Monthly | `git submodule update --remote` |
| Minor | Quarterly | Review changelog, test compatibility |
| Major | As needed | Breaking changes, full retest |
| Hotfix | Immediate | Critical bug fixes only |

## 5. Compatibility Testing

Before updating the submodule:

1. **API Compatibility:** Verify `TrainingConfig`, `TrainEngine`, `ModelType` haven't changed
2. **Model Compatibility:** Verify all model types used in experiments still exist
3. **Benchmark Compatibility:** Verify all benchmark functions still work
4. **Test Suite:** Run `cargo test` in both automl and 2048 project

```bash
# Run automl tests before update
cd automl && cargo test && cd ..

# Run 2048 tests before update
cargo test

# Update submodule
git submodule update --remote

# Run automl tests after update
cd automl && cargo test && cd ..

# Run 2048 tests after update
cargo test
```

## 6. Submodule Branch Tracking

```bash
# Track main branch
git submodule set-branch --branch main automl

# Fetch latest
git submodule update --remote --merge
```

## 7. Conflict Resolution

If submodule update causes conflicts:

1. Do NOT manually edit `automl/` directory
2. Revert submodule: `git checkout -- automl && git submodule update --init`
3. File issue on automl repo if API breaks
4. Use git bisect on submodule to find breaking commit

## 8. Submodule Health Check

```bash
# Verify submodule is initialized
test -d automl/.git && echo "Submodule OK" || echo "Submodule missing"

# Verify submodule commit
git -C automl rev-parse HEAD

# Check for uncommitted changes
git -C automl status --short
```
