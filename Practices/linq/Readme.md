# LINQ (Language INtegrated Query) Benzeşimleri

C# dilinde LINQ sorgularını sıklıkla kullanıyorum. Örneğin oyun bilgileri taşıyan bir veri seti üzerinde aşağıdaki gibi sorgular yazmak mümkün.

```csharp
/// <summary>
/// Türe göre oyunların listesini döndürür
/// </summary>
/// <param name="genre">Oyun türü</param>
/// <returns>Oyun Listesi</returns>
public List<VideoGame> GetByGenre(Genre genre)
{
    return games.Where(g => g.Genre == genre).ToList();
}

/// <summary>
/// Oyun listesini sıralama yönüne göre verir
/// </summary>
/// <param name="ordering">Sıralama yönü. Ascending veya Descending</param>
/// <returns>Oyun Listesi</returns>
public List<VideoGame> OrderByReleaseYear(Order ordering)
{
    if (ordering == Order.Descending)
        return games.OrderByDescending(g => g.ReleaseYear).ToList();

    return games.OrderBy(g => g.ReleaseYear).ToList();
}

/// <summary>
/// Oyun adlarını döndürür
/// </summary>
/// <returns>Oyun adları listesi</returns>
public List<string> GetGameTitles()
{
    return games.Select(g => g.Title).ToList();
}

/// <summary>
/// Bir programcının dahil olduğu oyunların listesini döndürür
/// </summary>
/// <param name="name">Programcının adı</param>
/// <returns>Oyun listesi</returns>
public List<VideoGame> GetGamesByProgrammer(string name)
{
    return games.Where(g => g.Programmers.Any(p => p.Name == name)).ToList();
}

/// <summary>
/// Belli bir yıldan sonraki oyunların listesini verir
/// </summary>
/// <param name="year">Sürüm yılı</param>
/// <returns>Oyun Listesi</returns>
public List<VideoGame> GetGamesAfterReleaseYear(int year)
{
    return games.Where(g => g.ReleaseYear > year).ToList();
}

/// <summary>
/// Oyunları türüne göre gruplayarak döndürür
/// </summary>
/// <returns>Kayıt listesi</returns>
public List<GroupedGame> CountingGameByGenre()
{
    return games.GroupBy(g => g.Genre).Select(grp => new GroupedGame { Genre = grp.Key, Count = grp.Count() }).ToList();
}

/// <summary>
/// Birden fazla yeteneği olan programcıların adlarını döndürür
/// </summary>
/// <returns>Programcı adları</returns>
public List<string> FindProgrammersByMultiSkills()
{
    return games.SelectMany(g => g.Programmers).GroupBy(p => p.Name).Where(group => group.Count() > 1).Select(group => group.Key).ToList();
}

/// <summary>
/// Oyun ve programcılarına ait listeyi döndürür
/// </summary>
/// <returns>Oyun ve programcılar listesi</returns>
public List<GameAndProgrammers> GetGamesAndProgrammers()
{
    return games.Select(g => new GameAndProgrammers(g.Title, g.Programmers.Select(p => p.Name))).ToList();
}

/// <summary>
/// Veri setindeki en eski oyun bilgisini döndürür
/// </summary>
/// <returns>Oyun bilgisi</returns>
public VideoGame? GetOldestGame()
{
    return games.OrderByDescending(g => g.ReleaseYear).LastOrDefault();
}

/// <summary>
/// Uzmanlık bilgisine göre oyun listesini döndürür
/// </summary>
/// <param name="expertise">Uzmanlık bilgisi</param>
/// <returns>Oyun Listesi</returns>
public List<VideoGame> GetGamesBySpecificExpertise(Expertise expertise)
{
    return games.Where(g => g.Programmers.Any(p => p.Expertise == expertise)).ToList();
}
```

Benzer işlevleri Rust tarafında nasıl karşılayabilirim merakında yola çıkarak bu örnek çalışmayı hazırlıyorum.
Her ne kadar yukarıdakileri birebir yapmamış olsam da fikir verecek kadarını elde ettim.

```rust
use crate::domain::{Expertise, Game, Genre, Order, Programmer};
use std::collections::HashMap;

pub struct GameQueryEngine {
    pub games: Vec<Game>,
}

impl GameQueryEngine {
    pub fn get_by_rate_gte(&self, rate: f32) -> Vec<Game> {
        self.games
            .iter()
            .filter(|&g| g.rating >= rate)
            .cloned()
            .collect()
    }

    pub fn get_game_names(&mut self, order: Order) -> Vec<String> {
        let mut sorted_games = self.games.clone();
        match order {
            Order::Ascending => sorted_games.sort_by(|g1, g2| g1.name.cmp(&g2.name)),
            Order::Descending => sorted_games.sort_by(|g1, g2| g2.name.cmp(&g1.name)),
        }
        sorted_games.iter().map(|g| g.name.clone()).collect()
    }

    pub fn get_average_rate(&self) -> f32 {
        let l = self.games.len();
        self.games.iter().map(|g| g.rating).sum::<f32>() / l as f32
    }

    pub fn grouped_by_genre(&self) -> HashMap<Genre, Vec<Game>> {
        let mut games_by_genre = HashMap::new();
        for game in self.games.iter() {
            games_by_genre
                .entry(game.genre.clone())
                .or_insert_with(Vec::new)
                .push(game.clone());
        }
        games_by_genre
    }

    pub fn grouped_by_genre_with_game_count(&self) -> HashMap<Genre, usize> {
        let mut counts_by_genre = HashMap::new();
        for game in &self.games {
            *counts_by_genre.entry(game.genre.clone()).or_insert(0) += 1;
        }
        counts_by_genre
    }

    pub fn get_higher_rated_game(&self) -> Option<Game> {
        self.games.iter().max_by_key(|g| g.rating as u32).cloned()
    }

    pub fn init() -> Self {
        Self {
            games: vec![
                Game {
                    name: "Space Adventure".to_string(),
                    genre: Genre::Action,
                    rating: 4.5,
                    programmer: vec![
                        Programmer {
                            name: "Alicey Johnson".to_string(),
                            expertise: Expertise::AI,
                        },
                        Programmer {
                            name: "Bob Smith".to_string(),
                            expertise: Expertise::Gameplay,
                        },
                    ],
                },
                Game {
                    name: "Speed Racer".to_string(),
                    genre: Genre::Racing,
                    rating: 4.7,
                    programmer: vec![
                        Programmer {
                            name: "Chlie Brown".to_string(),
                            expertise: Expertise::Gameplay,
                        },
                        Programmer {
                            name: "Alicey Johnson".to_string(),
                            expertise: Expertise::AI,
                        },
                        Programmer {
                            name: "Ben Johnsyn".to_string(),
                            expertise: Expertise::Graphics,
                        },
                    ],
                },
                Game {
                    name: "Mystic Quest".to_string(),
                    genre: Genre::Adventure,
                    rating: 4.2,
                    programmer: vec![
                        Programmer {
                            name: "David Lee".to_string(),
                            expertise: Expertise::LevelDesign,
                        },
                        Programmer {
                            name: "Alicey Johnson".to_string(),
                            expertise: Expertise::AI,
                        },
                        Programmer {
                            name: "Eevva Green".to_string(),
                            expertise: Expertise::Physics,
                        },
                        Programmer {
                            name: "Oliiva Wild".to_string(),
                            expertise: Expertise::Gameplay,
                        },
                    ],
                },
                Game {
                    name: "Future Shock".to_string(),
                    genre: Genre::Shooter,
                    rating: 4.8,
                    programmer: vec![Programmer {
                        name: "Monaa Lissa".to_string(),
                        expertise: Expertise::Gameplay,
                    }],
                },
                Game {
                    name: "Battlefield Glory".to_string(),
                    genre: Genre::Shooter,
                    rating: 4.3,
                    programmer: vec![Programmer {
                        name: "Natahan Darke".to_string(),
                        expertise: Expertise::Gameplay,
                    }],
                },
                Game {
                    name: "Magic Tower".to_string(),
                    genre: Genre::Puzzle,
                    rating: 3.9,
                    programmer: vec![Programmer {
                        name: "Umaa Turman".to_string(),
                        expertise: Expertise::Story,
                    }],
                },
                Game {
                    name: "Race Challenge".to_string(),
                    genre: Genre::Racing,
                    rating: 4.1,
                    programmer: vec![
                        Programmer {
                            name: "Monaa Lissa".to_string(),
                            expertise: Expertise::Gameplay,
                        },
                        Programmer {
                            name: "Umaa Turman".to_string(),
                            expertise: Expertise::Story,
                        },
                    ],
                },
                Game {
                    name: "Deep Dive".to_string(),
                    genre: Genre::Adventure,
                    rating: 4.4,
                    programmer: vec![Programmer {
                        name: "Viktor Hogo".to_string(),
                        expertise: Expertise::Gameplay,
                    }],
                },
                Game {
                    name: "Soccer Sim".to_string(),
                    genre: Genre::Sports,
                    rating: 3.8,
                    programmer: vec![
                        Programmer {
                            name: "Viktor Hogo".to_string(),
                            expertise: Expertise::LevelDesign,
                        },
                        Programmer {
                            name: "Lee Buruse".to_string(),
                            expertise: Expertise::Story,
                        },
                    ],
                },
                Game {
                    name: "Galaxy Invaders".to_string(),
                    genre: Genre::Action,
                    rating: 4.6,
                    programmer: vec![
                        Programmer {
                            name: "The Chosen Bill".to_string(),
                            expertise: Expertise::Gameplay,
                        },
                        Programmer {
                            name: "Lee Buruse".to_string(),
                            expertise: Expertise::Story,
                        },
                        Programmer {
                            name: "Donna Roni".to_string(),
                            expertise: Expertise::LevelDesign,
                        },
                    ],
                },
                Game {
                    name: "Treasure Island".to_string(),
                    genre: Genre::Adventure,
                    rating: 4.5,
                    programmer: vec![
                        Programmer {
                            name: "Viktor Hogo".to_string(),
                            expertise: Expertise::LevelDesign,
                        },
                        Programmer {
                            name: "Donna Roni".to_string(),
                            expertise: Expertise::LevelDesign,
                        },
                    ],
                },
            ],
        }
    }
}
```