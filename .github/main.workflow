workflow "Build on push" {
  on = "push"
  resolves = ["Test"]
}

action "Test" {
  uses = "./.github/actions/cargo"
  runs = "cargo test"
}
