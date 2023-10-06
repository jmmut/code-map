# code-map

You still have to add a BUTLER_API_KEY secret to the github repo so that the github workflow
can deploy to itch.io. If the github workflow still fails after this, try uploading manually an initial html5 build.

## Running this project

Clone this repo, then [Install rust](https://www.rust-lang.org/tools/install), then do `cargo run --release`.


## Arrangements

Different ways of plotting the hierarchical data are available.

### Linear

![linear](./screenshots/linear.png)

This arrangement is quite simple. Given a node with sub-nodes to be plotted in a rectangle, it will plot the sub-nodes along the longest side of the rectangle, sorted by size, biggest first.

Each sub-node will contain their own sub-sub-nodes in the same way.

This arrangement is not great when a node has many children. The children will be plotted as very thin lines.


## Roadmap

- input 
  - [ ] read counts from a file / stdin / sql dump
- UI
  - [ ] be able to click on a box and highlight only the parents (like click, backspace, backspace, enter)
    - [ ] right click to remove selection
  - [ ] box to search for a file (fuzzy search)
  - [ ] other arrangements that don't draw very thin lines
- cli/logs
  - [ ] -s --arrangement-strategy
  - [ ] -m --metric (file size, line count, code complexity, etc.)
  - [ ] -a --all-file-extensions
  - [ ] -i --input-file
  - [ ] -o --output-file (dump the hierarchical metrics to a file, as some metrics might be expensive to compute, e.g. code complexity)
  - [ ] aggregate counts of ignored files/extensions
      - [ ] list them with --verbose

