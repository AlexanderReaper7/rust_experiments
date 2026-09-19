//! Beggar-my-neighbour is a simple choice-free card game. https://en.wikipedia.org/wiki/Beggar-my-neighbour
//! The game is played with a standard deck of 52 cards. The deck is shuffled and divided evenly between the two players.
//! Each player plays a card from the top of their deck. If the card is a face card, the other player must play a face card or the stack of cards is given to the player who played the face card.
//! The game continues until one player has all the cards.
//! The player with all the cards wins.
use rand::Rng;

struct Game {
    /// Cards are represented as u8 values. Where 0 is a non-face card, and 1 is a jack, 2 is a queen, 3 is a king, and 4 is an ace.
    /// The Value of the card is the number of cards the other player can play where another face card must be played unless the played stack of cards is given to the player who played the face card.
    deck: [u8; 52],
    /// The position of the deck where the deck is split between the two players, inclusive of the split_position.
    /// The game ends when one player has all the cards. I.e. when the split_position is equal to 0 or 52.
    /// The first player wins if the split_position is 52, and the second player wins if the split_position is 0.
    split_position: usize,
    cards_played: u32,
    rounds_played: u32,
    p1_cards_played: usize,
    p2_cards_played: usize,
    stack: Vec<u8>,
}
impl Game {
    /// Create a sorted deck with the face cards in the beginning with ascending order.
    fn create_deck() -> [u8; 52] {
        let mut deck = [0; 52];
        deck[0..4].fill(1);
        deck[4..8].fill(2);
        deck[8..12].fill(3);
        deck[12..16].fill(4);
        deck
    }

    /// Shuffles a deck of cards.
    pub fn shuffle_deck(deck: &mut [u8; 52]) {
        let mut rng = rand::thread_rng();
        for i in 0..52 {
            let j = rng.gen_range(0..52);
            deck.swap(i, j);
        }
    }

    /// Shuffle the deck of the game.
    pub fn shuffle(&mut self) {
        Game::shuffle_deck(&mut self.deck);
    }

    /// Create a new game with a shuffled deck.
    pub fn new() -> Self {
        // Create the deck and fill it with the face cards.
        let mut deck = Game::create_deck();
        // Shuffle the deck.
        Game::shuffle_deck(&mut deck);
        // Return the game.
        Game {
            deck,
            split_position: 26, // The deck is split in the middle.
            cards_played: 0,
            rounds_played: 0,
            p1_cards_played: 0_usize,
            p2_cards_played: 0_usize,
            stack: Vec::with_capacity(52),
        }
    }

    pub fn reset(&mut self) {
        self.shuffle();
        self.split_position = 27;
    }

    /// Display a sequence of cards.
    pub fn display_cards(cards: &[u8]) -> String {
        let mut out: String = String::with_capacity(cards.len());
        for card in cards {
            out.push_str(match card {
                0 => "-",
                1 => "J",
                2 => "Q",
                3 => "K",
                4 => "A",
                _ => "?",
            });
        }
        out
    }

    /// Present the current state of the game.
    pub fn present(&self, message: &str) {
        println!("{}", message);
        println!("Round {}, Card {}", self.rounds_played, self.cards_played);
        println!("Player 1: {}", Self::display_cards(&self.deck[self.p1_cards_played..self.split_position]));
        println!("Player 2: {}", Self::display_cards(&self.deck[self.split_position+self.p2_cards_played..52]));
        println!("Stack: {}", Self::display_cards(&self.stack));
    }

    /// Play the game until one player has all the cards.
    pub fn play(&mut self) -> GameResult {
        while self.split_position > 0 && self.split_position < 52 {
            self.play_round();
        }
        
        return GameResult {
            is_first_player_winner: self.split_position == 52,
            cards_played: self.cards_played,
            rounds_played: self.rounds_played,
            deck: self.deck,
        };
    }
    
    /// Play a single round of the game - optimized version
    fn play_round(&mut self) {
        // Pre-allocate vector with capacity to avoid reallocations
        self.stack.clear();
        
        self.p1_cards_played = 0;
        self.p2_cards_played = 0;
        
        let mut is_p1_turn = true;
        let mut face_card_value = 0;
        let mut challenge_mode = false;
        
        while self.split_position > 0 && self.split_position < 52 {
            // If we're not in challenge mode, alternate players
            if !challenge_mode {
                is_p1_turn = !is_p1_turn;
            }
            
            // Check if current player has cards
            if (is_p1_turn && self.p1_cards_played >= self.split_position) || 
               (!is_p1_turn && self.split_position + self.p2_cards_played >= 52) {
                self.award_stack_to(!is_p1_turn);
                return;
            }
            
            // Play a card - using direct indexing
            let card_index = if is_p1_turn {
                self.p1_cards_played
            } else {
                self.split_position + self.p2_cards_played
            };
            
            // Safety check
            if card_index >= 52 {
                self.award_stack_to(!is_p1_turn);
                return;
            }
            
            let card = self.deck[card_index];
            self.stack.push(card);
            self.cards_played += 1;
            
            // Update player's played cards count
            if is_p1_turn {
                self.p1_cards_played += 1;
            } else {
                self.p2_cards_played += 1;
            }
            
            // Only generate debug messages if needed for debugging
            #[cfg(debug_assertions)]
            {
                let message = if challenge_mode {
                    format!("challenge, p1:{}, value:{}", is_p1_turn, face_card_value)
                } else {
                    if is_p1_turn { "p1 turn".to_string() } else { "p2 turn".to_string() }
                };
                self.present(&message);
            }
            
            // Face card handling - optimized conditional structure
            if card > 0 {
                // Face card played
                if challenge_mode {
                    is_p1_turn = !is_p1_turn;
                }
                face_card_value = card;
                challenge_mode = true;
                self.rounds_played += 1;
            } else if challenge_mode {
                // Normal card during challenge
                face_card_value -= 1;
                if face_card_value == 0 {
                    self.award_stack_to(!is_p1_turn);
                    return;
                }
            }
        }
    }
    
    /// Award the stack to the specified player
    fn award_stack_to(&mut self, to_player_1: bool) {
        // Safety check - make sure we have valid indices
        if self.p1_cards_played > self.split_position {
            self.p1_cards_played = self.split_position;
        }
        
        if self.split_position + self.p2_cards_played > 52 {
            self.p2_cards_played = 52 - self.split_position;
        }
        
        // Calculate remaining cards for each player
        let p1_remaining = self.split_position - self.p1_cards_played;
        let p2_remaining = 52 - (self.split_position + self.p2_cards_played);
        let stack_size = self.stack.len();
        
        // Create a temporary array with exactly the right capacity
        let mut new_deck = [0u8; 52];
        
        if to_player_1 {
            // Player 1 wins the stack
            
            // 1. Copy player 1's remaining cards to the beginning
            if p1_remaining > 0 {
                new_deck[0..p1_remaining].copy_from_slice(&self.deck[self.p1_cards_played..self.split_position]);
            }
            
            // 2. Copy stack after player 1's cards
            if stack_size > 0 {
                new_deck[p1_remaining..p1_remaining + stack_size].copy_from_slice(&self.stack);
            }
            
            // 3. Copy player 2's remaining cards after stack
            let new_split = p1_remaining + stack_size;
            if p2_remaining > 0 {
                new_deck[new_split..new_split + p2_remaining].copy_from_slice(
                    &self.deck[self.split_position + self.p2_cards_played..52]
                );
            }
            
            // Update split position
            self.split_position = new_split;
        } else {
            // Player 2 wins the stack
            
            // 1. Copy player 1's remaining cards to beginning
            if p1_remaining > 0 {
                new_deck[0..p1_remaining].copy_from_slice(&self.deck[self.p1_cards_played..self.split_position]);
            }
            
            // 2. Copy stack right after player 1's cards
            if stack_size > 0 {
                new_deck[p1_remaining..p1_remaining + stack_size].copy_from_slice(&self.stack);
            }
            
            // 3. Copy player 2's remaining cards after the stack
            let player2_start = p1_remaining + stack_size;
            if p2_remaining > 0 {
                new_deck[player2_start..player2_start + p2_remaining].copy_from_slice(
                    &self.deck[self.split_position + self.p2_cards_played..52]
                );
            }
            
            // Update split position to where player 1's cards end
            self.split_position = p1_remaining;
        }
        
        // Replace the old deck with the new one
        self.deck = new_deck;
        
        // Ensure split position is within bounds
        if self.split_position > 52 {
            self.split_position = 52;
        }
        
        // Clear the stack and reset played cards counters
        self.stack.clear();
        self.p1_cards_played = 0;
        self.p2_cards_played = 0;
        
        self.present(if to_player_1 { "p1 wins stack" } else { "p2 wins stack" });
    }
}

struct GameResult {
    /// The player who won the game.
    is_first_player_winner: bool,
    /// The number of cards played.
    cards_played: u32,
    /// The number of rounds played. (how many face cards were played)
    rounds_played: u32,
    /// The final state of the deck.
    deck: [u8; 52],
}

pub fn main() {
    let mut game = Game::new();
    game.present("start");
    let result = game.play();
    println!("Player 1 wins: {}", result.is_first_player_winner);
    println!("Cards played: {}", result.cards_played);
    println!("Rounds played: {}", result.rounds_played);
    game.present("end");
}
