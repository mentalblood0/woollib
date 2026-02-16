# 🐈 Woollib

A Rust library for managing theses. Each thesis is either text or relation. Supports tagging, aliasing and graph generation.

## Overview

Woollib provides a structured way to manage interconnected ideas (theses) and their relationships. It uses a text-based command system for easy input and generates graph visualizations. The library is built on top of [trove](https://github.com/mentalblood0/trove), an all-inexing document store for JSON-structured data.

## Key Features

- **Theses Management**: create, store, and retrieve propositions with unique identifiers
- **Relations**: define relationships between theses (e.g., "includes", "therefore", "negates", "answers" or any other defined in configuration file)
- **Tags**: categorize theses with custom tags for organization
- **Text-Based Commands**: simple syntax to add theses, relations, tags, and aliases from plain text
- **Aliases**: use short, human-readable names for referencing theses
- **Graph Generation**: export your knowledge graph to DOT format for visualization
- **ACID Transactions**: thread-safe operations with full transaction support

## Architecture

- **Sweater**: main entry point, manages database and configuration
- **Thesis**: a proposition with content (text or relation) and tags
- **Relation**: directed connection between two theses with a kind
- **Tag**: categorization label for theses
- **Alias**: human-readable name for referencing theses
- **ReadTransaction/WriteTransaction**: safe database access with ACID guarantees

## Commands Format

- `+ alias` - add text thesis (next line is content)
- `+ alias` - add relation thesis (next 3 lines: from, relation_kind, to)
- `- alias` - remove thesis
- `# alias tag1 tag2...` - add tags to thesis
- `^ alias tag1 tag2...` - remove tags from thesis

See [`src/example.txt`](src/example.txt) for a complete philosophical argument about relativism vs. absolutism. This example demonstrates:
- creating theses with content and relations
- using aliases for convenient referencing
- adding tags for categorization
- building complex argument structures

## Dependencies

- `trove` - database backend
- `serde`/`serde_json` - serialization to use with `trove`
- `regex` - commands parsing, text validation
- `xxhash-rust` - hashing
- `anyhow` - error handling
- `fallible-iterator` - Safe iterator usage with error handling
- `bincode` - binary serialization to generate identifiers as hashes of binary-encoded relation structure
- `html-escape` - encoding aliases for graph output

## Testing

```bash
cargo test
```

The test suite includes:
- **Generative tests**: performs randomly generated theses and relations actions to verify consistency
- **Example tests**: parses and processes the example file

## License

MIT
