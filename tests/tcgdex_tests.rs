use tcgdex_api::endpoints::cards::{Card, CardBrief};
use tcgdex_api::endpoints::series::{Serie, SerieBrief};
use tcgdex_api::endpoints::sets::{Set, SetBrief};
use tcgdex_api::query::Query;
use tcgdex_api::*;

#[test]
fn get_types() {
    let tcgdex = Tcgdex::new();
    let types = tcgdex
        .types()
        .fetch()
        .expect("The API should returns a types list");
    assert!(types.len() >= 11)
}

#[test]
fn get_categories() {
    let tcgdex = Tcgdex::new();
    let categories = tcgdex
        .categories()
        .fetch()
        .expect("The API should returns a categories list");
    assert!(categories.len() >= 3)
}

#[test]
fn get_hps() {
    let tcgdex = Tcgdex::new();
    let hps = tcgdex
        .hps()
        .fetch()
        .expect("The API should returns a HP list");
    assert!(hps.len() >= 32)
}

#[test]
fn get_illustrators() {
    let tcgdex = Tcgdex::new();
    let illustrators = tcgdex
        .illustrators()
        .fetch()
        .expect("The API should returns an illustrators list");
    assert!(illustrators.len() >= 251)
}

#[test]
fn get_rarities() {
    let tcgdex = Tcgdex::new();
    let rarities = tcgdex
        .rarities()
        .fetch()
        .expect("The API should returns a rarities list");
    assert!(rarities.len() >= 27)
}

#[test]
fn get_retreats() {
    let tcgdex = Tcgdex::new();
    let retreats = tcgdex
        .retreats()
        .fetch()
        .expect("The API should returns a retreat costs list");
    assert!(retreats.len() >= 5)
}

#[test]
fn get_all_series() {
    let tcgdex = Tcgdex::new();
    let series: Vec<SerieBrief> = tcgdex
        .series()
        .fetch(None)
        .expect("The API should returns a series list");
    assert!(series.len() >= 19)
}

#[test]
fn get_specific_serie() {
    let tcgdex = Tcgdex::new();
    let filter = Query::new().with_id("swsh");
    let serie: Serie = tcgdex
        .series()
        .fetch(Some(&filter))
        .expect("The API should returns a serie");
    assert_eq!(serie.id, "swsh");
    assert_eq!(serie.name, "Sword & Shield")
}

#[test]
fn get_filtered_series() {
    let tcgdex = Tcgdex::new();
    let filter = Query::new().with_filtering(vec!["name=Sword & Shield"]);
    let series: Vec<SerieBrief> = tcgdex
        .series()
        .fetch(Some(&filter))
        .expect("The API should returns a series list");
    assert_eq!(series.len(), 1)
}

#[test]
fn get_all_sets() {
    let tcgdex = Tcgdex::new();
    let sets: Vec<SetBrief> = tcgdex
        .sets()
        .fetch(None)
        .expect("The API should returns a sets list");
    assert!(sets.len() >= 173)
}

#[test]
fn get_specific_set() {
    let tcgdex = Tcgdex::new();
    let filter = Query::new().with_id("swsh3");
    let set: Set = tcgdex
        .sets()
        .fetch(Some(&filter))
        .expect("The API should returns a set");
    assert_eq!(set.id, "swsh3");
    assert_eq!(set.release_date, "2020-08-14");
    assert_eq!(set.serie.name, "Sword & Shield");
    assert_eq!(set.serie.id, "swsh");
    assert!(set.legal.expanded);
    assert_eq!(set.name, "Darkness Ablaze")
}

#[test]
fn get_filtered_sets() {
    let tcgdex = Tcgdex::new();
    let filter = Query::new().with_filtering(vec!["cardCount.total=201"]);
    let sets: Vec<SetBrief> = tcgdex
        .sets()
        .fetch(Some(&filter))
        .expect("The API should returns a sets list");
    assert!(!sets.is_empty())
}

#[test]
fn get_all_cards() {
    let tcgdex = Tcgdex::new();
    let cards: Vec<CardBrief> = tcgdex
        .cards()
        .fetch(None)
        .expect("The API should returns a cards list");
    assert!(cards.len() >= 17915)
}

#[test]
fn get_specific_card() {
    let tcgdex = Tcgdex::new();
    let filter = Query::new().with_id("swsh3-136");
    let card: Card = tcgdex
        .cards()
        .fetch(Some(&filter))
        .expect("The API should returns a card");
    assert_eq!(card.id, "swsh3-136");
    assert_eq!(card.illustrator, "tetsuya koizumi");
    assert_eq!(card.name, "Furret");
    assert_eq!(card.set.card_count.official, 189);
    assert_eq!(card.hp, 110);
    assert_eq!(card.types, vec!["Colorless"]);
    assert_eq!(card.weaknesses[0]._type, "Fighting");
}

#[test]
fn get_filtered_cards() {
    let tcgdex = Tcgdex::new();
    let filter = Query::new().with_filtering(vec!["name=furret", "id=ex"]);
    let cards: Vec<CardBrief> = tcgdex
        .cards()
        .fetch(Some(&filter))
        .expect("The API should returns a cards list");
    assert!(cards.len() >= 2);
    assert_eq!(cards[0].id, "ex7-22");
    assert_eq!(cards[1].id, "ex12-33");
}

#[test]
fn get_tcgdex_error_message() {
    let tcgdex = Tcgdex::new();
    let filter = Query::new().with_id("sih3-136");

    let card_result = tcgdex.cards().fetch::<Card>(Some(&filter));
    assert!(card_result.is_err());

    let error = card_result.err().unwrap();
    assert!(error.is_tcgdexapi());
    assert_eq!(
        format!("{error}"),
        "Tcgdex error : The resource you are trying to reach does not exists"
    );

    let message = error.get_tcgdex_error().unwrap();
    assert_eq!(message._type, "https://tcgdex.dev/errors/not-found");
    assert_eq!(
        message.title,
        "The resource you are trying to reach does not exists"
    );
    assert_eq!(message.status, 404);
    assert_eq!(message.endpoint, "/en/cards/sih3-136");
    assert_eq!(message.method, "GET");
}
