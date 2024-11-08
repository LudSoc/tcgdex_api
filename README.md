This is a Pokémon TCG SDK wrapper around the API of [tcgdex]([https://pokemontcg.io](https://www.tcgdex.net)).

# To start

Create a unique instance of **Tcgdex**.
```
let tcgdex = Tcgdex::new();
```

English is default language. You can change it like this :
```
let tcgdex = Tcgdex::new();
tcgdex.set_lang(Lang::FR);
```

**Tcgdex** instance must be used for all requests.

### Request

All requests are blocking for the moment.

# Functions available

### Find card by id
```
let filter = Query::new().with_id("swsh3-136");
let card: Card = tcgdex
  .cards()
  .fetch(Some(&filter))
  .unwrap();
```

### Filter cards via query parameters
```
let filter = Query::new().with_filtering(vec!["name=furret", "id=ex"]);
let cards: Vec<CardBrief> = tcgdex
  .cards()
  .fetch(Some(&filter))
  .unwrap();
```

### Find all cards (firts page only)
```
let cards: Vec<CardBrief> = tcgdex
  .cards()
  .fetch(None)
  .unwrap();
```

### Find set by id
```
let filter = Query::new().with_id("swsh3");
let set: Set = tcgdex
  .sets()
  .fetch(Some(&filter))
  .unwrap();
```

### Filter sets via query parameters
```
let query: Query = Query::new()
      .with_q("legalities.standard:legal")
      .with_order_by("-releaseDate")
      .with_select("id,name,releaseDate");
let sets_list = pkmn_tcg.sets().where_(&query).unwrap();
```

### Find all sets
```
let sets: Vec<SetBrief> = tcgdex
  .sets()
  .fetch(None)
  .unwrap();
```

### About series

The principle is the same than for set and card. You can get a serie by id, or all series or a filtered list of series.

### Find all types
```
let types: Types = tcgdex.types().fetch().unwrap();
```

### Others data

You can do the same for rarities, hp, illustrators, retreat costs and categories.
