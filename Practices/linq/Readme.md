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

```rust

```