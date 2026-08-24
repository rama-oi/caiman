# Caiman

![Preview](https://raw.githubusercontent.com/rama-oi/caiman/refs/heads/main/assets/0.png)

Keyboard layout preview with theme support. TUI replacement of `xev`

```sh
┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬───────────┐
│ ~ ~ │ ! ! │ @ @ │ # # │ $ $ │ % % │ ^ ^ │ & & │ * * │ ( ( │ ) ) │ _ _ │ + + │ backspace │
│ ` ` │ 1 1 │ 2 2 │ 3 3 │ 4 4 │ 5 5 │ 6 6 │ 7 7 │ 8 8 │ 9 9 │ 0 0 │ - - │ = = │ backspace │
├─────┴─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┬─────┤
│    tab    │ Q Q │ W W │ E E │ R R │ T T │ Y Y │ U U │ I I │ O O │ P P │ { { │ } } │ | | │
│    tab    │ q q │ w w │ e e │ r r │ t t │ y y │ u u │ i i │ o o │ p p │ [ [ │ ] ] │ \ \ │
├───────────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┴─────┤
│ caps lock │ A A │ S S │ D D │ F F │ G G │ H H │ J J │ K K │ L L │ : : │ " " │   enter   │
│ caps lock │ a a │ s s │ d d │ f f │ g g │ h h │ j j │ k k │ l l │ ; ; │ ' ' │   enter   │
├───────────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┴───────────┤
│  l-shift  │ Z Z │ X X │ C C │ V V │ B B │ N N │ M M │ < < │ > > │ ? ? │     r-shift     │
│  l-shift  │ z z │ x x │ c c │ v v │ b b │ n n │ m m │ , , │ . . │ / / │     r-shift     │
├─────┬─────┼─────┼─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┬─────┬─────┤
│ ctr │ sup │ alt │                        spacebar                           │ alt │ ctr │
│ ctr │ sup │ alt │                        spacebar                           │ alt │ ctr │
└─────┴─────┴─────┴───────────────────────────────────────────────────────────┴─────┴─────┘
```

```sh
nix-shell --run "cargo build"
```

```sh
cargo run
```