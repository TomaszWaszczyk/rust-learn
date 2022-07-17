//! Drawing cards using VRFs

extern crate schnorrkel;
use merlin::Transcript;
use schnorrkel::{Keypair, PublicKey, vrf::{VRFInOut, VRFPreOut, VRFProof}};
use rand::Rng;

const NUM_DRAWS : u8 = 8;
const NUM_CARDS : u16 = 52;

struct Player {
    keypair: Keypair,
    cards: Vec<(u16, [u8; 97])>,
    balance: i32,
}

impl Player {
    pub fn create(keypair: Keypair, balance: i32) -> Self {
        Player {
            keypair,
            cards: vec![],
            balance,
        }
    }
    pub fn cards_in_hand(&mut self, cards: Vec<(u16, [u8; 97])>) {
        self.cards = cards;
    }
}

fn main(){
    println!("Hello VRF");
    let VRF_seed = &[0u8; 32];
    let mut vault = 0;
    let mut csprng = rand_core::OsRng;


    // generate 5 players
    let mut players: Vec<Player> = (0..5)
        .map(|_| Player::create(Keypair::generate_with(&mut csprng), 1000))
        .collect();

    // each player get 2 cards
    players.iter_mut().for_each(|player| {
        let cards: Vec<(u16, [u8; 97])> = (0..2)
            .filter_map(|i| try_draw(&player.keypair, VRF_seed, i))
            .collect();
        player.cards_in_hand(cards);
    });

    bid(&mut players, &mut vault);

    let table = Keypair::generate_with(&mut csprng);
    
    let mut cards: Vec<(u16, [u8; 97])> = (0..3)
        .filter_map(|i| try_draw(&table, VRF_seed, i))
        .collect();
    

    //3rd card
    let card = try_draw(&table, VRF_seed, 2).unwrap();
    cards.push(card);


    bid(&mut players, &mut vault);

    //4th card
    let card = try_draw(&table, VRF_seed, 3).unwrap();
    cards.push(card);


    let table_cards = reveal_cards(&cards, &table.public, VRF_seed);

    println!("Cards on the table are: {:?}", table_cards);

    let table_sum: u16 = table_cards.iter().sum();

    let mut highest_score = (0, &PublicKey::default());

    players.iter().for_each(|player| {
        let player_cards = reveal_cards(&player.cards, &player.keypair.public, VRF_seed);
        println!("Player with public key: {:?} has cards: {:?}", player.keypair.public.to_bytes(), player_cards);

        let sum: u16 = player_cards.iter().sum::<u16>();

        let player_sum = table_sum + sum;
        if highest_score.0 < player_sum {
            highest_score = (player_sum, &player.keypair.public);
        }
    });

    println!(
        "Player with public key: {:?} is a winner with the score {}. He wins ${}",
        highest_score.1.to_bytes(),
        highest_score.0,
        vault
    );


    // call draw calls
    let mut csprng = rand_core::OsRng;
    let mut keypair = Keypair::generate_with(&mut csprng);

    let mut draw = draws(&keypair, VRF_seed);
    
    let (card, signature)= draw[0];
    
    // println!("This is your card: {:?}", card);
    // println!("***************************");
    // println!("This is your draw cards (card, signature): {:?}", draw);
    // println!("***************************");
    // println!("This is your signature: {:?}", signature);
    // println!("***************************");
    
    // reveal cards we must call receive
    let public_key = keypair.public;

    let reveal_card = recieve(&public_key, &signature, VRF_seed);

    // println!("This is the revealed card: {:?}", reveal_card);

}

/// Processes VRF inputs, checking validity of the number of draws
fn draw_transcript(seed: &[u8; 32], draw_num: u8) -> Option<Transcript> {
        if draw_num > NUM_DRAWS { return None; }
        let mut t = Transcript::new(b"Card Draw Transcript");
        t.append_message(b"seed",seed);
        t.append_u64(b"draw", draw_num as u64);
        Some(t)
}

/// Computes actual card draw from VRF inputs & outputs together
fn find_card(io: &VRFInOut) -> Option<u16> {
        let b: [u8; 8] = io.make_bytes(b"card");
        // We make one in half the draws invalid so nobody knows how many cards anyone else has
        // if b[7] & 0x80 { return None; }
        Some( (u64::from_le_bytes(b) % (NUM_CARDS as u64)) as u16 )
}

/// Attempts to draw a card
fn try_draw(
        keypair: &Keypair,
        seed: &[u8; 32],
        draw_num: u8
) -> Option<(u16, [u8; 97])> {
        let t = draw_transcript(seed, draw_num) ?;
        let (io, proof, _) = keypair.vrf_sign(t);
        let card = find_card(&io) ?;
        let mut vrf_signature = [0u8; 97];
        // the first 32 bytes are io
        vrf_signature[..32].copy_from_slice(& io.to_preout().to_bytes()[..]);
        // the next 64 bytes are the proof
        vrf_signature[32..96].copy_from_slice(& proof.to_bytes()[..]);
        // the final byte is the draw number
        vrf_signature[96] = draw_num;
        Some((card, vrf_signature))
}

/// Draws all our cards for the give seed
fn draws(
        keypair: &Keypair,
        seed: &[u8; 32],
) -> Vec<(u16, [u8; 97])>
{
        (0..NUM_DRAWS).filter_map(|i| try_draw(keypair, seed, i)).collect()
}

/// Verifies a card play
///
/// We depend upon application code to enforce the public key and seed
/// being chosen correctly.
///
/// We encode the draw number into the vrf signature since an honest 
/// application has no use for this, outside the verification check in
/// `draw_transcript`.
fn recieve(
        public: &PublicKey,
        vrf_signature: &[u8; 97],
        seed: &[u8; 32],
) -> Option<u16>
{
        let t = draw_transcript(seed, vrf_signature[96]) ?;
        let out = VRFPreOut::from_bytes(&vrf_signature[..32]).ok() ?;
        let proof = VRFProof::from_bytes(&vrf_signature[32..96]).ok() ?;
        // We need not understand the error type here, but someone might
        // care about invalid signatures vs invalid card draws.
        let (io, _) = public.vrf_verify(t, &out, &proof).ok() ?;
        find_card(&io)
}

fn bid(players: &mut [Player], bank: &mut i32) {
    players.iter_mut().for_each(|player| {
        let bid = rand::thread_rng().gen_range(0..301);
        player.balance -= bid;
        println!(
            "Player with key {:?} made a bid of {}",
            player.keypair.public.to_bytes(), bid
        );
        *bank += bid;
    });
}

fn reveal_cards(cards: &[(u16, [u8; 97])], key: &PublicKey, seed: &[u8; 32]) -> Vec<u16> {
    cards
        .iter()
        .filter_map(|card| recieve(key, &card.1, seed))
        .collect()
}
