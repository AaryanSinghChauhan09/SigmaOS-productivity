# SigmaOS Productivity

Rust library containing experimental productivity-related modules, including
notes, tasks, calendar, email, media, PDF, editor, clipboard, finance, and
office-oriented data models.

## Current integration limits

The large module tree is a collection of APIs, not proof of a complete office
suite or integrated desktop applications. Assess each module independently
for persistence, file-format compatibility, privacy, and error handling before
depending on it. This README does not claim that data is saved, synchronized,
or interoperable with other office products.

Favor fast keyboard-first workflows, cohesive defaults, and user-controlled
data storage without weakening security boundaries. Keep each module's public
contract and evidence close to its implementation rather than treating the
entire crate as one verified feature set.

## Development

The repository CI workflow resolves SigmaOS path dependencies and runs Rust
tests. Review its result for the commit being considered; compilation alone
does not establish user-facing compatibility.
