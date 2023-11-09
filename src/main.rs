use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <div style="text-align: center; font-size: 80px; color: white;">
                <h1>"Connect 4"</h1>
                <Grid/>
            </div>
        }
    })
}

#[derive(Debug, Clone)]
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

#[component]
fn Grid() -> impl IntoView {
    let (grid, set_grid) = create_signal(
        (0..6)
            .map(|_| vec![None; 7])
            .collect::<Vec<Vec<Option<Player>>>>(),
    );
    let (cur_player, set_cur_player) = create_signal(Player::A);

    view! {
        <table style="margin:0 auto; font-size: 80px">
            {move || {
                grid
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(x, cols)| {
                        view! {
                            <tr>
                                {cols
                                    .into_iter()
                                    .enumerate()
                                    .map(|(y, tile)| {
                                        let mut player_idx=0;
                                        let user_click = move |_| {
                                            leptos::logging::log!("clicked {x} {y}");
                                            set_grid
                                                .update(|g| {
                                                    leptos::logging::log!("{g:?}");
                                                    g[x][y] = Some(cur_player.get());
                                                });
                                            set_cur_player.set(cur_player.get().other_player());
                                        };
                                        let idk = match tile {
                                            Some(Player::A) => "🔴",
                                            Some(Player::B) => "🟡",
                                            None => "⚪",
                                        };
                                        view! { <td on:click=user_click style="cursor: crosshair;">{idk}</td> }
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
