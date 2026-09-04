# Rust track — learning path

Exercism keeps each exercise at `rust/<slug>/` so `exercism submit` still works.

The CLI will not download an exercise until it is **unlocked** on the site.
Start with `hello-world`, submit it, then download the next unlocked one:

```bash
exercism download --track rust --exercise hello-world
cd rust/hello-world
cargo test
exercism submit
```

The CLI workspace is this `exercism/` directory.

When you have a choice, prefer this order: syllabus (concept) exercises if
they appear, then practice exercises in the track’s official `config.json`
order. The Rust syllabus is still a work in progress on Exercism
(`concept_exercises` is not enabled), so most of the track is practice.

## 1. Concept / syllabus

| # | Slug | Teaches |
|---|------|---------|
| 1 | `lucians-luscious-lasagna` | functions |
| 2 | `assembly-line` | integers, floats |
| 3 | `semi-structured-logs` | enums |
| 4 | `short-fibonacci` | `vec!` |
| 5 | `resistor-color` | external crates |
| 6 | `health-statistics` | methods, structs |
| 7 | `low-power-embedded-game` | tuples, destructuring |
| 8 | `role-playing-game` | `Option` |
| 9 | `magazine-cutout` | HashMap entry API |
| 10 | `rpn-calculator` | vec as stack |

Skipped: `csv-builder` (WIP).

## 2. Practice (easy → later difficulty, track order)

See `rust/` for the downloaded crates. Official practice order starts with
`hello-world`, `reverse-string`, `gigasecond`, then mixed difficulties as on
[the Rust track](https://exercism.org/tracks/rust/exercises).
