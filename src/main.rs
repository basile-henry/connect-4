use leptos::*;

fn check_layout<'a, I: Iterator<Item = &'a Option<Player>>>(it: I) -> bool {
    let mut hist = [None; 4];
    let idxs = [0, 1, 2, 3];
    for (cell, i) in it.zip(idxs.iter().cycle()) {
        hist[*i] = *cell;
        if hist.iter().all(|p| *p == Some(Player::A)) || hist.iter().all(|p| *p == Some(Player::B))
        {
            return true;
        }
    }
    false
}

fn main() {
    mount_to_body(|| {
        view! {
            <div style="text-align: center; font-size: 80px; color: white;">
                <h1>"Connect 4"</h1>
                <div class="fireworks-wrapper">
                    <div class="firework"></div>
                    <div class="firework"></div>
                    <div class="firework"></div>
                </div>
                <Grid/>
            </div>
        }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    A,
    B,
}

impl Player {
    pub fn other_player(&self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Vec<Option<Player>>>,
}

impl Board {
    fn new() -> Self {
        Self {
            grid: (0..6).map(|_| vec![None; 7]).collect(),
        }
    }

    fn place_in_column(&mut self, column: usize, player: Player) {
        if let Some(row) = self.grid.iter_mut().rfind(|row| row[column].is_none()) {
            row[column] = Some(player);
        }
    }

    pub fn has_win(&self) -> bool {
        let it1 = self.grid.iter().flatten();

        let c_len = self.grid[0].len();
        let it2 = self.grid.iter().flatten().step_by(c_len);
        if check_layout(it1) || check_layout(it2) {
            return true;
        }
        for i in 0..c_len {
            for j in 3..self.grid.len() {
                let it = self
                    .grid
                    .iter()
                    .flatten()
                    .skip(i * c_len + j)
                    .step_by(c_len - 1);

                if check_layout(it) {
                    return true;
                }
            }

            for j in 0..self.grid.len() - 3 {
                let it = self
                    .grid
                    .iter()
                    .flatten()
                    .skip(i * c_len + j)
                    .step_by(c_len + 1);

                if check_layout(it) {
                    return true;
                }
            }
        }
        false
    }
}

fn set_win_screen() {
    if let Some(body) = leptos::document().body() {
        body.set_class_name("game_finished");
    }
}

#[component]
fn Grid() -> impl IntoView {
    let (grid, set_grid) = create_signal(Board::new());
    let (cur_player, set_cur_player) = create_signal(Player::A);

    view! {
        <table style="margin: 0 auto;">
            {move || {
                grid
                    .get()
                    .grid
                    .into_iter()
                    .enumerate()
                    .map(|(x, cols)| {
                        view! {
                            <tr>
                                {cols
                                    .into_iter()
                                    .enumerate()
                                    .map(|(y, tile)| {
                                        let user_click = move |_| {
                                            leptos::logging::log!("clicked {x} {y}");
                                            set_grid
                                                .update(|b| {
                                                    if !b.has_win() {
                                                        b.place_in_column(y, cur_player.get());
                                                    }
                                                    leptos::logging::log!("{b:?}");
                                                    if b.has_win() {
                                                        leptos::logging::log!("WIN!!!");
                                                        set_win_screen();
                                                    }
                                                });
                                            set_cur_player.set(cur_player.get().other_player());
                                        };
                                        let idk = match tile {
                                            Some(Player::A) => "🔴",
                                            Some(Player::B) => "🟡",
                                            None => "⚪",
                                        };
                                        view! {
                                            <td
                                                on:click=user_click
                                                style="cursor: crosshair; user-select: none;"
                                            >
                                                {idk}
                                            </td>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </tr>
                        }
                    })
                    .collect::<Vec<_>>()
            }}

        </table>
    }
}
