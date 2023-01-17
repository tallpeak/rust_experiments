use rand::seq::SliceRandom;
use rand::thread_rng;
const PRISONER_COUNT : usize = 100;
const DRAW_COUNT : usize = PRISONER_COUNT / 2; // how many times to open a drawer
const LOOP_COUNT : usize = 1000000; // count for simulation

// open half of the drawers
// return true if prisoner found his card
fn open_drawers(prisoner : usize, drawers : & mut Vec<usize>) -> bool {
    let mut dn = prisoner; // initial drawer number to open
    for _drawer in 1..=DRAW_COUNT {
        let pn = drawers[dn-1]; // convert 1-based index to 0-based
        if pn == prisoner {
            return true;
        }
        dn = pn;
    }
    false
}

fn main() {
    // drawer[i] contains a random (shuffled) prisoner number
    let prisoners = (1..=PRISONER_COUNT).collect::<Vec<usize>>();
    let mut rng = thread_rng();
    let mut lost: i64 = 0;
    let mut won: i64 = 0;
    let mut drawers: Vec<usize> = prisoners.clone();
    for _loop in 0..LOOP_COUNT {
        // for (i,card) in cards.iter().enumerate() {
        //     println!("cards[{}]={}", i, card);
        // }
        //println!("Unshuffled: {:?}", drawers);
        // reinitialize to 1..=PRISONER_COUNT before shuffle (shouldn't be necessary but seems more proper?)
        drawers.copy_from_slice(&prisoners[..]); 
        drawers.shuffle(&mut rng);
        //println!("Shuffled:   {:?}", drawers);
        let mut lost_round = false;
        for prisoner in prisoners.clone() {
            let found = open_drawers(prisoner, & mut drawers);
            if !found {
                lost_round = true;
                break;
            }
        }
        if lost_round {
            lost+=1; // everyone killed
        } else {
            won+=1; // prisoners saved
        }
    }
    println!{"loops: {}  prisoners: {} won: {}  lost: {}", LOOP_COUNT, PRISONER_COUNT, won, lost};
} 