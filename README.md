# Chain Reaction

Chain reaction game is a strategy board game for two players.

## How to play ?

- Initially board contains number of empty cells.
- Each Player take turn in placing 1 atom each turn.
- Atom can be place in cell which is blank or has player's own atoms.
- Each cell has critical mass (corner: 2, edge: 3, others: 4).
- After reaching critical mass a cell's atom reacts and explosdes into surronding.
- Player creating explosion owns the surrounding cells.
- Last surviving player on the board wins.

## How to run cli ?

```bash
cargo run
```

## How to run web ui ?

```bash
cargo install trunk
trunk serve
```

<!---
Generate for GitHub pages at `/url`.
```bash
trunk serve --public-url="/url"
```
-->
