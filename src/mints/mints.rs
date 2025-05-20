#![allow(dead_code)]
struct TokenInfo {
    name: &'static str,
    mint: &'static str,
    decimals: u64,
}
const WSOL: TokenInfo = TokenInfo {
    name: "WSOL",
    mint: "So11111111111111111111111111111111111111112",
    decimals: 9,
};

const USDC: TokenInfo = TokenInfo {
    name: "USDC",
    mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    decimals: 6,
};

const USDT: TokenInfo = TokenInfo {
    name: "USDT",
    mint: "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
    decimals: 6,
};


const WETH: TokenInfo = TokenInfo {
    name: "WETH",
    mint: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    decimals: 8,
};

const MOODENG: TokenInfo = TokenInfo {
    name: "MOODENG",
    mint: "ED5nyyWEzpPPiWimP8vYm7sD7TD3LAt3Q3gRTWHzPJBY",
    decimals: 6,
};

const POPCAT: TokenInfo = TokenInfo {
    name: "POPCAT",
    mint: "7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr",
    decimals: 9,
};

const FARTCOIN: TokenInfo = TokenInfo {
    name: "FARTCOIN",
    mint: "9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump",
    decimals: 6,
};