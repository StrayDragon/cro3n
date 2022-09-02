```
 ██████╗ ██████╗   ██████╗3 ███╗   ██╗
██╔════╝ ██╔══██╗ ██╔═══██╗ ████╗  ██║
██║      ██████╔╝ ██║   ██║ ██╔██╗ ██║
██║      ██╔══██╗ ██║   ██║ ██║╚██╗██║
╚██████╗ ██║  ██║ ╚██████╔╝ ██║ ╚████║
 ╚═════╝ ╚═╝  ╚═╝  ╚═════╝  ╚═╝  ╚═══╝
```

[![CI](https://github.com/StrayDragon/cro3n/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/StrayDragon/cro3n/actions/workflows/CI.yml)
[![GitHub last commit](https://img.shields.io/github/last-commit/straydragon/cro3n)](https://github.com/StrayDragon/cro3n/commits)
[![PyPI](https://img.shields.io/pypi/v/cro3n)](https://pypi.org/project/cro3n)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/cro3n)](https://pypi.org/project/cro3n)
[![PyPI - Implementation](https://img.shields.io/pypi/implementation/cro3n)](https://pypi.org/project/cro3n)
[![PyPI - License](https://img.shields.io/pypi/l/cro3n)](https://github.com/StrayDragon/cro3n/blob/main/LICENSE)

---

Another one **cron expression** utils package for python, wrapped on rust crate [zslayton/cron](https://github.com/zslayton/cron).

## Installation
```bash
pip install -U cro3n
```

## Example

### validate cron expression
```python
import cro3n

cron_expr = "" # write your expression
print(cro3n.check_cron_expression(cron_expr)) # empty str is ok, otherwise means errors
```

## Relation Projects

- [zslayton/cron](https://github.com/zslayton/cron)
