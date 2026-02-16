# 🐈 Woollib

A Rust library for managing theses and relations, supports tagging, aliasing and graph generation.

## Overview

Woollib provides a structured way to manage interconnected ideas (theses) and their relationships. It uses a text-based command system for easy input and generates graph visualizations. The library is built on top of [trove](https://github.com/mentalblood0/trove), an all-inexing document store for JSON-structured data.

## Key Features

- **Theses Management**: Create, store, and retrieve propositions with unique identifiers
- **Relations**: Define relationships between theses (e.g., "includes", "therefore", "negates", "answers" or any other defined in configuration file)
- **Tags**: Categorize theses with custom tags for organization
- **Text-Based Commands**: Simple syntax to add theses, relations, tags, and aliases from plain text
- **Aliases**: Use short, human-readable names for referencing theses
- **Graph Generation**: Export your knowledge graph to DOT format for visualization
- **ACID Transactions**: Thread-safe operations with full transaction support

## Architecture

- **Sweater**: Main entry point, manages database and configuration
- **Thesis**: A proposition with content (text or relation) and tags
- **Relation**: Directed connection between two theses with a kind
- **Tag**: Categorization label for theses
- **Alias**: Human-readable name for referencing theses
- **ReadTransaction/WriteTransaction**: Safe database access with ACID guarantees

## Commands Format

- `+ alias` - Add text thesis (next line is content)
- `+ alias` - Add relation thesis (next 3 lines: from, relation_kind, to)
- `- alias` - Remove thesis
- `# alias tag1 tag2...` - Add tags to thesis
- `^ alias tag1 tag2...` - Remove tags from thesis

See [`src/example.txt`](src/example.txt) for a complete philosophical argument about relativism vs. absolutism. This example demonstrates:
- Creating theses with content and relations
- Using aliases for convenient referencing
- Adding tags for categorization
- Building complex argument structures

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
- **Generative tests**: Randomly generates theses and relations to verify consistency
- **Example tests**: Parses and processes the example file

## License

MIT
