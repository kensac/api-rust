use std::cmp::Ordering;

use crate::prisma::Role;

impl Role {
    // Helper function to get the numeric value of each variant
    const fn value(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Volunteer => 1,
            Self::Team => 2,
            Self::Exec => 3,
            Self::Tech => 4,
            Self::Finance => 5,
        }
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Role {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}
