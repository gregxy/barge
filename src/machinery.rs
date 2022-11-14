use std::sync::Arc;
use tokio::time::sleep;

use crate::inner::{Role, BargeCore};

pub(crate) async fn wait_for_heartbeat(abc: Arc<BargeCore>, tick: u64) {
    sleep(abc.config.pick_heartbeat_timeout()).await;

    let mut do_election: bool = false;

    {
        let mut state = abc.state.lock();
    
        if state.tick != tick || state.role != Role::Follower{
            return;
        }

        state.tick += 1;
        state.role = Role::Candidate;

        do_election = true;
    }

    if !do_election {
        return;
    }

    tokio::spawn(wait_for_election(abc.clone(), tick + 1));

    // TODO: send vote requests
}

async fn wait_for_election(abc: Arc<BargeCore>, tick: u64) {
    let mut expect_tick: u64 = tick;

    loop {
        sleep(abc.config.pick_election_timeout()).await;

        let mut do_election_again: bool = false;
        {
            let mut state = abc.state.lock();
 
            if state.tick != expect_tick || state.role != Role::Candidate {
                return;
            }
 
            state.tick += 1;
            expect_tick += 1;
            do_election_again = true;
        }

        if !do_election_again {
            return;
        }

        // TODO: send vote requests
    }
}

