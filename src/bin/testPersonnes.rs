use std::collections::HashMap;
use ternary_rs::if_else;

fn main() {
    let mut personnes = Personnes::new();
    let person = Person {
        product_id: 1,
        nom: String::from("TestNom"),
        prenom: String::from("TestPrenom"),
        email: Some("test@mail.com".to_string()),
        actif: true,
    };
    let person2 = Person {
        product_id: 2,
        nom: String::from("TestNom"),
        prenom: String::from("TestPrenom"),
        email: Some("test@mail.com".to_string()),
        actif: false,
    };
    let product_id = person.product_id;
    personnes.add(person);
    personnes.add(person2);
    read_person(&personnes);
    update_personnes(&mut personnes, &product_id, "VincenZO".to_string());
    println!("Read basic method : ");
    //read_person(&personnes);
    println!("Read While Let method : ");
    //read_person_while_let(&personnes);
    let active_persons: Vec<_> = personnes
        .inner
        .values()
        .filter(|&person| if_else!(person.actif, true, false))
        .collect();
    println!("active_person : {:?}", active_persons.len());
    read_active_person(active_persons);
}

fn read_active_person(personnes: Vec<&Person>) {
    println!("read_active_person : ");
    let mut personnes_iter = personnes.iter();
    while let Some(person) = personnes_iter.next() {
        println!("Personne active : {:?}", person);
    }
}

fn read_person(personnes: &Personnes) {
    for person in personnes.inner.values() {
        let product_id = person.product_id;
        let nom = &person.nom;
        let prenom = &person.prenom;
        let email = match &person.email {
            Some(_email) => &person.email,
            None => return,
        };
        // let email2 = email.unwrap_or_else(|| String::from("vv"));
        let actif = &person.actif;
        println!(
            "{:?}, {:?}, {:?}, {:?}, {:?}",
            product_id, nom, prenom, email, actif
        );
        println!("Affichage personne : {:?}", person);
    }
}

fn read_person_while_let(personnes: &Personnes) {
    let mut personnes_iter = personnes.inner.iter();
    while let Some(person) = personnes_iter.next() {
        println!("{:?}", person);
    }
}

fn update_personnes(personnes: &mut Personnes, product_id: &i64, nom: String) {
    if personnes.update(&product_id, nom) {
        println!("personne updated");
    } else {
        println!("person was not found");
    }
}

struct Personnes {
    inner: HashMap<i64, Person>,
}

impl Personnes {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, person: Person) {
        self.inner.insert(person.product_id, person);
    }

    fn remove(&mut self, product_id: &i64) -> bool {
        self.inner.remove(product_id).is_some()
    }

    fn update(&mut self, product_id: &i64, nom: String) -> bool {
        match self.inner.get_mut(product_id) {
            Some(person) => {
                if nom != "" {
                    person.nom = nom;
                }
                true
            }
            None => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Person {
    product_id: i64,
    nom: String,
    prenom: String,
    email: Option<String>,
    actif: bool,
}
