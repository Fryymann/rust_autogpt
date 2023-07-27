

trait Attacker {
    fn choose_style(&self) -> String;
    fn choose_weapon(&self) -> String;
}


#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "wing chun".to_string(),
            Character::Archer => "kung fu".to_string(),
            Character::Wizard => "tai chi".to_string(),
        }
    }

    fn choose_weapon(&self) -> String {
        match self {
            Character::Warrior => "sword".to_string(),
            Character::Archer => "nunchucks".to_string(),
            Character::Wizard => "ki".to_string(),
        }
    }
}



#[cfg(test)]
    use super::*;

    #[test]
    fn test_traits() {
        let my_character: Character = Character::Archer;
        let chosen_fighting_style: String = my_character.choose_style();
        let chosen_weapon: String = my_character.choose_weapon();

        dbg!(chosen_fighting_style);
    }