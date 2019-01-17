# open-graph
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Generate HTML for Open Graph integration.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Basic usage__
```rust
use open_graph::{OpenGraph, ContentType};

let card = OpenGraph::builder()
  .site("@flickr")
  .title("The Rock")
  .type(ContentType::VideoMovie)
  .image("http://ia.media-imdb.com/images/rock.jpg")
  .build();
```
```html
<--! Output -->
<meta property="og:title" content="The Rock" />
<meta property="og:type" content="video.movie" />
<meta property="og:url" content="http://www.imdb.com/title/tt0117500/" />
<meta property="og:image" content="http://ia.media-imdb.com/images/rock.jpg" />
```

## Installation
```sh
$ cargo add open-graph
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
- [Open Graph protocol specification](http://ogp.me/)
- [Facebook object debugger](https://developers.facebook.com/tools/debug/)
- [LinkedIn post inspector](https://www.linkedin.com/post-inspector/inspect/)

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/open-graph.svg?style=flat-square
[2]: https://crates.io/crates/open-graph
[3]: https://img.shields.io/travis/rust-net-web/open-graph/master.svg?style=flat-square
[4]: https://travis-ci.org/rust-net-web/open-graph
[5]: https://img.shields.io/crates/d/open-graph.svg?style=flat-square
[6]: https://crates.io/crates/open-graph
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/open-graph

[releases]: https://github.com/rust-net-web/open-graph/releases
[contributing]: https://github.com/rust-net-web/open-graph/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/rust-net-web/open-graph/labels/good%20first%20issue
[help-wanted]: https://github.com/rust-net-web/open-graph/labels/help%20wanted
