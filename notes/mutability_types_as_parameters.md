<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" class="logo" width="120"/>

# Please show an example of a self usage instead of \&self

Certainly! In Rust, using `self` by value in a method means the method **takes ownership** of the instance and consumes it. This is useful when you want to **transform** the instance and return a new one, or when you want to **consume** it (e.g., to convert it into something else).

## Example: Using `self` by Value

Let's say you want a method that **consumes** the `MapData` instance and returns a new, updated one with a modified cell.

```rust
impl<const X: usize, const Y: usize> MapData<X, Y> {
    // Consumes self and returns a new MapData with the updated cell
    fn update_cell(self, x: usize, y: usize, t: ETypeFloor) -> Self {
        // Create a mutable copy of self (since self is owned)
        let mut new_map = self;

        let x = if new_map.invert_row_origin { new_map.map_size_x - 1 - x } else { x };
        let y = if new_map.invert_col_origin { new_map.map_size_y - 1 - y } else { y };

        new_map.cells[y][x].update(t);
        println!("UPDATE_CELL \n {:?} - {} {}", new_map.cells[y][x], x, y);

        new_map  // Return the updated map
    }
}
```


### How to use it:

```rust
let map = MapData::new(...); // create initial map
let updated_map = map.update_cell(2, 3, ETypeFloor::Door); // consumes `map`, returns updated map
```


---

## Key Points

- The method **takes ownership** of `self` (the original map).
- You can modify `self` freely because you own it.
- You return the updated instance.
- The original instance is **moved** and no longer usable after calling the method.
- This pattern is common in immutable data structures or builder patterns.

---

## Contrast with `&mut self`

Using `&mut self`:

- You borrow the instance mutably.
- You modify the instance in place.
- The instance remains usable after the method call.

Example:

```rust
fn update_cell(&mut self, x: usize, y: usize, t: ETypeFloor) {
    // modifies self in place
}
```


---

## Summary Table

| Receiver Type | Ownership | Can Modify? | Consumes Instance? | Usage Scenario |
| :-- | :-- | :-- | :-- | :-- |
| `self` | Yes | Yes | Yes | Transform and return new value |
| `&mut self` | No | Yes | No | Modify in place |
| `&self` | No | No | No | Read-only access |


---


