# peppec-test

A library of integer helper functions and the command line that exposes them. It is also the test bench for the PEPPEC agents, so its work is cut into small issues.

## Language

### Command line

**Command**:
The first word after `peppec-test`, naming what the program computes (`median`, `stats`).
_Avoid_: Subcommand, verb

**Number**:
One input value: a 32-bit signed integer written as an optional sign followed by decimal digits.
_Avoid_: Value, integer, argument

**Statistic**:
One of the six named results of the `stats` command: count, min, max, range, average, median.
_Avoid_: Metric, measure, field

**Canonical order**:
The fixed order in which statistics are always shown: count, min, max, range, average, median.
_Avoid_: Default order, display order

**Average**:
The integer mean of the numbers, truncated toward zero. It is not a decimal mean, so it can differ from the median of the same two numbers.
_Avoid_: Mean

### Work

**Spec**:
The parent issue that describes a whole feature and lists its scenarios.
_Avoid_: PRD, epic

**Slice**:
One sub-issue of a spec: a piece of work that delivers visible behaviour on its own and carries a size.
_Avoid_: Task, ticket, step

**Scenario**:
A given/when/then example written in a spec, which the last slice turns into an automated test.
_Avoid_: Acceptance test, case
