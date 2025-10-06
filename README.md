# Symmetria-core

[![!pypi](https://img.shields.io/pypi/v/symmetria-core?color=orange)](https://pypi.org/project/symmetria-core/)
[![MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/VascSch92/symmetria-core/blob/main/LICENSE)

This package provides the core functionality for [symmetria](https://github.com/VascoSch92/symmetria) package.

## Example of direct usage

**Note**: you should not use symmetria-core directly; instead use symmetria, which use symmetria-core as backend.

```py
from symmetria_core import permutation

# a permutation is defined via its image
perm: tuple[int, ...] = (3, 4, 5, 2, 1, 6, 7)

ascents = permutation.ascents(image=perm)
assert ascents == [1, 2, 5, 6]

descents = permutation.descents(image=perm)
assert descents == [3, 4]

exceedances = permutation.exceedances(image=perm, weakly=True)
assert exceedances == [1, 2, 3, 6, 7]

```
## Prerequisites

To contribute to this project you'll need:
1. **[Rust](https://rustup.rs/)** - Rust stable
2. **[uv](https://docs.astral.sh/uv/getting-started/installation/)** - Fast Python package manager
3. **[git](https://git-scm.com/)** - For version control
4. **[make](https://www.gnu.org/software/make/)** - For running development commands (or use `nmake` on Windows)
