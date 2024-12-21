use crate::prelude::*;

use super::person::{Person, SecretFriend};

use rand::{seq::SliceRandom, thread_rng};
use std::collections::{HashMap, HashSet};

fn validate_people(people: &[Person], amount: usize) -> Result<()> {
    if amount < 1 {
        return Err(Error::Generic(f!(
            "A quantidade de sorteios ({{amount}}) deve ser maior que 0"
        )));
    }
    if people.len() < 2 {
        return Err(Error::Generic(f!(
            "O número de pessoas ({{people.len()}}) deve ser maior que 1"
        )));
    }
    if amount > people.len() {
        return Err(Error::Generic(f!(
            "A quantidade de sorteios ({{amount}}) deve ser menor ou igual ao número de pessoas ({{people.len()}})"
        )));
    }
    if people.len() % 2 != 0 {
        return Err(Error::Generic(f!(
            "O número de pessoas ({{people.len()}}) deve ser par"
        )));
    }

    Ok(())
}

pub fn sort_secret_friends(
    people: Vec<Person>,
    amount: Option<usize>,
) -> Result<Vec<SecretFriend>> {
    let amount = amount.unwrap_or(1);

    validate_people(&people, amount)?;

    let mut rng = thread_rng();
    let mut result = Vec::new();

    // Cria uma matriz de disponibilidade
    // Para cada pessoa, quantas vezes ela ainda pode ser sorteada
    let mut available_slots: HashMap<String, usize> =
        people.iter().map(|p| (p.name.clone(), amount)).collect();

    for person in &people {
        let mut chosen_friends = HashSet::new();

        // Para cada sorteio dessa pessoa
        for _ in 0..amount {
            // Cria lista de candidatos válidos
            let mut valid_receivers: Vec<&Person> = people
                .iter()
                .filter(|p| {
                    let slots_left = available_slots.get(&p.name).unwrap_or(&0);
                    p.name != person.name && // Não pode ser a própria pessoa
                    !chosen_friends.contains(&p.name) && // Não pode já ter sido escolhido
                    *slots_left > 0 // Ainda pode receber presentes
                })
                .collect();

            if valid_receivers.is_empty() {
                return Err(Error::Generic(
                    "Não foi possível encontrar uma distribuição válida".into(),
                ));
            }

            valid_receivers
                .sort_by_key(|p| std::cmp::Reverse(*available_slots.get(&p.name).unwrap_or(&0)));

            let max_slots = available_slots.get(&valid_receivers[0].name).unwrap_or(&0);
            let best_candidates: Vec<&Person> = valid_receivers
                .into_iter()
                .take_while(|p| available_slots.get(&p.name).unwrap_or(&0) == max_slots)
                .collect();

            let secret_friend = best_candidates.choose(&mut rng).unwrap().clone();

            chosen_friends.insert(secret_friend.name.clone());
            if let Some(slots) = available_slots.get_mut(&secret_friend.name) {
                *slots -= 1;
            }

            result.push(SecretFriend {
                person: person.clone(),
                secret_friend: secret_friend.clone(),
            });
        }
    }

    Ok(result)
}
