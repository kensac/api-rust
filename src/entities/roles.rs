use std::cmp::Ordering;

use crate::prisma::Role;

impl Role {
    // Helper function to get the numeric value of each variant
    fn value(&self) -> i32 {
        match self {
            Role::None => 0,
            Role::Volunteer => 1,
            Role::Team => 2,
            Role::Exec => 3,
            Role::Tech => 4,
            Role::Finance => 5,
        }
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Ord for Role {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}
