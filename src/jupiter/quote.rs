use reqwest::Url;
use serde::{Deserialize, Serialize};

/// Doc: https://dev.jup.ag/docs/swap-api/get-quote
pub struct QuoteParams {
    /// <Required>
    /// The input mint address
    input_mint: String,

    /// <Required>
    /// The output mint address
    output_mint: String,

    /// <Required>
    /// The amount to swap
    amount: u64,

    /// The number of basis points you can tolerate to lose during the time of execution. E.g., 1% = 100bps
    slippage_bps: Option<u64>,

    /// Possible values: [ExactIn, ExactOut]
    /// Default value: ExactIn
    /// In the case of ExactIn, the slippage is on the output token
    /// In the case of ExactOut, the slippage is on the input token
    /// Not all AMMs support ExactOut: Currently only Orca Whirlpool, Raydium CLMM, Raydium CPMM
    swap_mode: Option<String>,

    /// Multiple DEXes can be pass in by comma separating them,
    /// For example, dexes=Raydium, Orca+V2, Meteora+DLMM
    /// If a DEX is indicated, the route will only use that DEX
    /// Full list of DEXes here: https://lite-api.jup.ag/swap/v1/program-id-to-label
    dexes: Option<Vec<String>>,
    /// Multiple DEXes can be pass in by comma separating them,
    /// For example, excludeDexes=Raydium, Orca+V2, Meteora+DLMM
    /// If a DEX is indicated, the route will not use that DEX
    /// Full list of DEXes here: https://lite-api.jup.ag/swap/v1/program-id-to-label
    exclude_dexes: Option<Vec<String>>,
    /// Restrict intermediate tokens within a route to a set of more stable tokens
    /// This will help to reduce exposure to potential high slippage routes
    restrict_intermediate_tokens: Option<bool>,

    /// Direct Routes limits Jupiter routing to single hop routes only
    /// This may result in worse routes
    only_direct_routes: Option<bool>,

    /// Instead of using versioned transaction, this will use the legacy transaction
    /// Default: false
    as_legacy_transaction: Option<bool>,

    /// Take fees in basis points
    /// Used together with feeAccount in /swap, see：Adding Fees guide: https://dev.jup.ag/docs/swap-api/add-fees-to-swap
    platform_fee_bps: Option<u64>,

    /// Rough estimate of the max accounts to be used for the quote
    /// Useful if composing your own transaction or to be more precise in resource accounting for better routes
    /// Default: 64
    max_accounts: Option<u64>,

    /// If true, slippageBps will be overridden by Dynamic Slippage's estimated
    /// The value is returned in /swap endpoint
    dynamic_slippage: Option<bool>,
}

impl QuoteParams {
    pub fn new(input_mint: String, output_mint: String, amount: u64) -> Self {
        Self {
            input_mint,
            output_mint,
            amount,
            slippage_bps: None,
            swap_mode: None,
            dexes: None,
            exclude_dexes: None,
            restrict_intermediate_tokens: None,
            only_direct_routes: None,
            as_legacy_transaction: None,
            platform_fee_bps: None,
            max_accounts: None,
            dynamic_slippage: None,
        }
    }

    pub fn to_query(self) -> Vec<(String, String)> {
        let mut query = vec![
            ("inputMint".to_string(), self.input_mint.clone()),
            ("outputMint".to_string(), self.output_mint.clone()),
            ("amount".to_string(), self.amount.to_string()),
        ];

        if let Some(slippage_bps) = self.slippage_bps {
            query.push(("slippageBps".to_string(), slippage_bps.to_string()));
        }
        if let Some(swap_mode) = &self.swap_mode {
            query.push(("swapMode".to_string(), swap_mode.clone()));
        }
        if let Some(dexes) = &self.dexes {
            query.push(("dexes".to_string(), dexes.join(",")));
        }
        if let Some(exclude_dexes) = &self.exclude_dexes {
            query.push(("excludeDexes".to_string(), exclude_dexes.join(",")));
        }
        if let Some(true) = self.restrict_intermediate_tokens {
            query.push(("restrictIntermediateTokens".to_string(), "true".to_string()));
        }
        if let Some(true) = self.only_direct_routes {
            query.push(("onlyDirectRoutes".to_string(), "true".to_string()));
        }
        if let Some(true) = self.as_legacy_transaction {
            query.push(("asLegacyTransaction".to_string(), "true".to_string()));
        }
        if let Some(platform_fee_bps) = self.platform_fee_bps {
            query.push(("platformFeeBps".to_string(), platform_fee_bps.to_string()));
        }
        if let Some(max_accounts) = self.max_accounts {
            query.push(("maxAccounts".to_string(), max_accounts.to_string()));
        }
        if let Some(true) = self.dynamic_slippage {
            query.push(("dynamicSlippage".to_string(), "true".to_string()));
        }

        query
    }
}
pub async fn quote(url: &str, params: QuoteParams) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut url = Url::parse(&(url.to_string() + "/quote"))?;
    url.query_pairs_mut().extend_pairs(params.to_query());

    let response = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await?;

    let body = response.text().await?;

    #[cfg(debug_assertions)]
    {
        println!("Response: {}", body);
    }

    let _quote_rsp: QuoteResponse = serde_json::from_str(&body)?;
    Ok(_quote_rsp)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapInfo {
    #[serde(rename = "ammKey")]
    pub amm_key: String,
    pub label: String,
    #[serde(rename = "inputMint")]
    pub input_mint: String,
    #[serde(rename = "outputMint")]
    pub output_mint: String,
    #[serde(rename = "inAmount")]
    pub in_amount: String,
    #[serde(rename = "outAmount")]
    pub out_amount: String,
    #[serde(rename = "feeAmount")]
    pub fee_amount: String,
    #[serde(rename = "feeMint")]
    pub fee_mint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoutePlan {
    #[serde(rename = "swapInfo")]
    pub swap_info: SwapInfo,
    pub percent: i64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct PlatformFee {
    pub amount: String,
    #[serde(rename = "feeBps")]
    pub fee_bps: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteResponse {
    #[serde(rename = "inputMint")]
    pub input_mint: String,
    #[serde(rename = "inAmount")]
    pub in_amount: String,
    #[serde(rename = "outputMint")]
    pub output_mint: String,
    #[serde(rename = "outAmount")]
    pub out_amount: String,
    #[serde(rename = "otherAmountThreshold")]
    pub other_amount_threshold: String,
    #[serde(rename = "swapMode")]
    pub swap_mode: String,
    #[serde(rename = "slippageBps")]
    pub slippage_bps: i64,
    #[serde(rename = "platformFee")]
    pub platform_fee: Option<PlatformFee>,
    #[serde(rename = "priceImpactPct")]
    pub price_impact_pct: String,
    #[serde(rename = "routePlan")]
    pub route_plan: Vec<RoutePlan>,
    #[serde(rename = "contextSlot")]
    pub context_slot: Option<i64>,
    #[serde(rename = "timeTaken")]
    pub time_taken: Option<f64>,
}