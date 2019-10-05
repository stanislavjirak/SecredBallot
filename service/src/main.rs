use map_vec::Set;
use oasis_std::{Address, Context};

// Each service definition contains a struct that derives `Service`.
// This struct represents the service's persistent state.
// Changes to the state will be persisted across service invocations.
#[derive(oasis_std::Service)]
struct Ballot {
    description: String,
    candidates: Vec<String>,
    // Vetes nova vetes
    tally: Vec<i32>,
    accepting_votes: bool,
    admin: Address,
    voters: Set<Address>,
}

type Result<T> = std::result::Result<T, String>; // define our own result type, for convenience

impl Ballot {
    /// Constructs a new `Ballot`.
    pub fn new(ctx: &Context, description: String, candidates: Vec<String>) -> Self {
        Self {
            description,
            tally: vec![0; candidates.len()],
            candidates,
            accepting_votes: true,
            admin: ctx.sender(),
            voters: Set::new(),
        }
    }

    /// Returns the candidates being voted upon.
    pub fn candidates(&self, _ctx: &Context) -> Vec<&str> {
        self.candidates.iter().map(String::as_ref).collect()
    }

    /// Returns the description of this ballot.
    pub fn description(&self, _ctx: &Context) -> &str {
        &self.description
    }

    /// Returns whether voting is still open.
    pub fn voting_open(&self, _ctx: &Context) -> bool {
        self.accepting_votes
    }

    /// Cast a vote for a candidate.
    /// `candidate_num` is the index of the chosen candidate in `Ballot::candidates`.
    /// If you have already voted, this will change your vote to the new candidate.
    /// Voting for an invalid candidate or after the ballot has closed will return an `Err`.
    pub fn vote(&mut self, ctx: &Context,
                candidate_num2: i32,
                candidate_num1: i32,
                candidate_num_minus: i32) -> Result<()> {
        if !self.accepting_votes {
            return Err("Voting is closed.".to_string());
        }
        if candidate_num2 >= 0
            && candidate_num2 as usize >= self.candidates.len() {
            return Err(format!("Invalid candidate with 2 votes `{}`.", candidate_num2));
        }
        if candidate_num1 >= 0
            && candidate_num1 as usize >= self.candidates.len() {
            return Err(format!("Invalid candidate with 1 votes `{}`.", candidate_num1));
        }
        if candidate_num_minus >= 0
            && candidate_num_minus as usize >= self.candidates.len() {
            return Err(format!("Invalid candidate with -1 votes `{}`.", candidate_num_minus));
        }

        if self.voters.contains(&ctx.sender()) {
            return Err("You have already voted".to_string());
        }

        self.voters.insert(ctx.sender());

        if (candidate_num2 != -1 && candidate_num2 == candidate_num1)
            || (candidate_num2 != -1 && candidate_num2 == candidate_num_minus)
            || (candidate_num1 != -1 && candidate_num1 == candidate_num_minus)
        {
            return Err("Cannot vote for multiple candidates at once".to_string());
        }

        // Candidate with 2 votes
        if candidate_num2 >= 0 {
            self.tally[candidate_num2 as usize] += 2;
        }
        if candidate_num1 >= 0 {
            self.tally[candidate_num1 as usize] += 1;
        }
        if candidate_num_minus >= 0 {
            self.tally[candidate_num_minus as usize] -= 1;
        }

        Ok(())
    }

    /// Closes this ballot so that it no longer collects votes.
    /// Only the ballot creator can close voting.
    pub fn close(&mut self, ctx: &Context) -> Result<()> {
        if self.admin != ctx.sender() {
            return Err("You cannot close the ballot.".to_string());
        }
        self.accepting_votes = false;
        Ok(())
    }

    /// Returns the index of the candidate with the most votes.
    /// This method can only be called after voting has closed.
    pub fn winner(&self, _ctx: &Context) -> Result<u32> {
        if self.accepting_votes {
            return Err("Voting is not closed.".to_string());
        }
        Ok(self
            .tally
            .iter()
            .enumerate()
            .max_by_key(|(_i, v)| *v)
            .unwrap()
            .0 as u32)
    }

    /// Returns the number of votes cast for each candidate.
    /// This method can only be called after voting has closed.
    pub fn results(&self, _ctx: &Context) -> Result<Vec<i32>> {
        if self.accepting_votes {
            return Err("Voting is not closed.".to_string());
        }
        Ok(self.tally.clone())
    }
}

fn main() {
    oasis_std::service!(Ballot);
}

#[cfg(test)]
mod tests {
    // This is required even in Rust 2018. If omitted, rustc will not link in the testing
    // library and will produce a giant error message.
    extern crate oasis_test;

    use super::*;

    /// Creates a new account and a `Context` with the new account as the sender.
    fn create_account() -> (Address, Context) {
        let addr = oasis_test::create_account(0 /* initial balance */);
        let ctx = Context::default().with_sender(addr).with_gas(100_000);
        (addr, ctx)
    }

    #[test]
    fn functionality() {
        let (_admin, admin_ctx) = create_account();
        let (_voter, voter_ctx) = create_account();
        let (_voter2, voter2_ctx) = create_account();

        let description = "What's for dinner?";
        let candidates = vec![
            "beef".to_string(),
            "yogurt".to_string(),
            "cofee".to_string(),
        ];
        let mut ballot =
            Ballot::new(&admin_ctx, description.to_string(), candidates.clone());

        assert_eq!(ballot.description(&admin_ctx), description);
        assert_eq!(ballot.candidates(&admin_ctx), candidates);
        assert_eq!(ballot.voting_open(&admin_ctx), true);

        // Can't get winner before voting has closed.
        assert!(ballot.winner(&voter_ctx).is_err());

        ballot.vote(&voter_ctx, 0, 1, 2).unwrap();

        ballot.vote(&voter_ctx, 1, -1, -1).unwrap_err();
        ballot.vote(&admin_ctx, 0, 1, -1).unwrap();

        // Non-admin can't close ballot.
        ballot.close(&voter_ctx).unwrap_err();
        ballot.close(&admin_ctx).unwrap();

        // Votes can't be cast after ballot has closed.
        ballot.vote(&voter2_ctx, 0, -1, -1).unwrap_err();

        assert_eq!(ballot.voting_open(&voter_ctx), false);
        assert_eq!(ballot.winner(&voter_ctx).unwrap(), 0);
        assert_eq!(ballot.results(&voter_ctx).unwrap(), vec![4, 2, -1]);
    }
}
