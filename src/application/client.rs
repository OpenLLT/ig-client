/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 19/10/25
******************************************************************************/
use crate::application::auth::WebsocketInfo;
use crate::application::interfaces::account::AccountService;
use crate::application::interfaces::market::MarketService;
use crate::application::interfaces::order::OrderService;
use crate::error::AppError;
use crate::model::http::HttpClient;
use crate::model::requests::RecentPricesRequest;
use crate::model::requests::{
    ClosePositionRequest, CreateOrderRequest, CreateWorkingOrderRequest, UpdatePositionRequest,
};
use crate::model::responses::{
    ClosePositionResponse, CreateOrderResponse, CreateWorkingOrderResponse, UpdatePositionResponse,
};
use crate::model::responses::{
    DBEntryResponse, HistoricalPricesResponse, MarketNavigationResponse, MarketSearchResponse,
    MultipleMarketDetailsResponse,
};
use crate::model::streaming::{
    StreamingAccountDataField, StreamingMarketField, StreamingPriceField,
    get_streaming_account_data_fields, get_streaming_market_fields, get_streaming_price_fields,
};
use crate::prelude::{
    AccountActivityResponse, AccountFields, AccountsResponse, OrderConfirmationResponse,
    PositionsResponse, TradeFields, TransactionHistoryResponse, WorkingOrdersResponse,
};
use crate::presentation::market::{MarketData, MarketDetails};
use crate::presentation::price::PriceData;
use async_trait::async_trait;
use lightstreamer_rs::client::{LightstreamerClient, Transport};
use lightstreamer_rs::subscription::{
    ChannelSubscriptionListener, Snapshot, Subscription, SubscriptionMode,
};
use lightstreamer_rs::utils::setup_signal_hook;
use serde_json::Value;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, Notify, RwLock, mpsc};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

const MAX_CONNECTION_ATTEMPTS: u64 = 3;

/// Main client for interacting with IG Markets API
///
/// This client provides a unified interface for all IG Markets API operations,
/// including market data, account management, and order execution.
pub struct Client {
    http_client: Arc<HttpClient>,
}

impl Client {
    /// Creates a new client instance
    ///
    /// # Returns
    /// A new Client with default configuration
    pub fn new() -> Self {
        let http_client = Arc::new(HttpClient::default());
        Self { http_client }
    }

    /// Gets WebSocket connection information for Lightstreamer
    ///
    /// # Returns
    /// * `WebsocketInfo` containing server endpoint, authentication tokens, and account ID
    pub async fn get_ws_info(&self) -> WebsocketInfo {
        self.http_client.get_ws_info().await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MarketService for Client {
    async fn search_markets(&self, search_term: &str) -> Result<MarketSearchResponse, AppError> {
        let path = format!("markets?searchTerm={}", search_term);
        info!("Searching markets with term: {}", search_term);
        let result: MarketSearchResponse = self.http_client.get(&path, Some(1)).await?;
        debug!("{} markets found", result.markets.len());
        Ok(result)
    }

    async fn get_market_details(&self, epic: &str) -> Result<MarketDetails, AppError> {
        let path = format!("markets/{epic}");
        info!("Getting market details: {}", epic);
        let market_value: Value = self.http_client.get(&path, Some(3)).await?;
        let market_details: MarketDetails = serde_json::from_value(market_value)?;
        debug!("Market details obtained for: {}", epic);
        Ok(market_details)
    }

    async fn get_multiple_market_details(
        &self,
        epics: &[String],
    ) -> Result<MultipleMarketDetailsResponse, AppError> {
        if epics.is_empty() {
            return Ok(MultipleMarketDetailsResponse::default());
        } else if epics.len() > 50 {
            return Err(AppError::InvalidInput(
                "The maximum number of EPICs is 50".to_string(),
            ));
        }

        let epics_str = epics.join(",");
        let path = format!("markets?epics={}", epics_str);
        debug!(
            "Getting market details for {} EPICs in a batch",
            epics.len()
        );

        let response: MultipleMarketDetailsResponse = self.http_client.get(&path, Some(2)).await?;

        Ok(response)
    }

    async fn get_historical_prices(
        &self,
        epic: &str,
        resolution: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalPricesResponse, AppError> {
        let path = format!(
            "prices/{}?resolution={}&from={}&to={}",
            epic, resolution, from, to
        );
        info!("Getting historical prices for: {}", epic);
        let result: HistoricalPricesResponse = self.http_client.get(&path, Some(3)).await?;
        debug!("Historical prices obtained for: {}", epic);
        Ok(result)
    }

    async fn get_historical_prices_by_date_range(
        &self,
        epic: &str,
        resolution: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<HistoricalPricesResponse, AppError> {
        let path = format!("prices/{}/{}/{}/{}", epic, resolution, start_date, end_date);
        info!(
            "Getting historical prices for epic: {}, resolution: {}, from: {} to: {}",
            epic, resolution, start_date, end_date
        );
        let result: HistoricalPricesResponse = self.http_client.get(&path, Some(2)).await?;
        debug!(
            "Historical prices obtained for epic: {}, {} data points",
            epic,
            result.prices.len()
        );
        Ok(result)
    }

    async fn get_recent_prices(
        &self,
        params: &RecentPricesRequest<'_>,
    ) -> Result<HistoricalPricesResponse, AppError> {
        let mut query_params = Vec::new();

        if let Some(res) = params.resolution {
            query_params.push(format!("resolution={}", res));
        }
        if let Some(f) = params.from {
            query_params.push(format!("from={}", f));
        }
        if let Some(t) = params.to {
            query_params.push(format!("to={}", t));
        }
        if let Some(max) = params.max_points {
            query_params.push(format!("max={}", max));
        }
        if let Some(size) = params.page_size {
            query_params.push(format!("pageSize={}", size));
        }
        if let Some(num) = params.page_number {
            query_params.push(format!("pageNumber={}", num));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        let path = format!("prices/{}{}", params.epic, query_string);
        info!("Getting recent prices for epic: {}", params.epic);
        let result: HistoricalPricesResponse = self.http_client.get(&path, Some(3)).await?;
        debug!(
            "Recent prices obtained for epic: {}, {} data points",
            params.epic,
            result.prices.len()
        );
        Ok(result)
    }

    async fn get_historical_prices_by_count_v1(
        &self,
        epic: &str,
        resolution: &str,
        num_points: i32,
    ) -> Result<HistoricalPricesResponse, AppError> {
        let path = format!("prices/{}/{}/{}", epic, resolution, num_points);
        info!(
            "Getting historical prices (v1) for epic: {}, resolution: {}, points: {}",
            epic, resolution, num_points
        );
        let result: HistoricalPricesResponse = self.http_client.get(&path, Some(1)).await?;
        debug!(
            "Historical prices (v1) obtained for epic: {}, {} data points",
            epic,
            result.prices.len()
        );
        Ok(result)
    }

    async fn get_historical_prices_by_count_v2(
        &self,
        epic: &str,
        resolution: &str,
        num_points: i32,
    ) -> Result<HistoricalPricesResponse, AppError> {
        let path = format!("prices/{}/{}/{}", epic, resolution, num_points);
        info!(
            "Getting historical prices (v2) for epic: {}, resolution: {}, points: {}",
            epic, resolution, num_points
        );
        let result: HistoricalPricesResponse = self.http_client.get(&path, Some(2)).await?;
        debug!(
            "Historical prices (v2) obtained for epic: {}, {} data points",
            epic,
            result.prices.len()
        );
        Ok(result)
    }

    async fn get_market_navigation(&self) -> Result<MarketNavigationResponse, AppError> {
        let path = "marketnavigation";
        info!("Getting top-level market navigation nodes");
        let result: MarketNavigationResponse = self.http_client.get(path, Some(1)).await?;
        debug!("{} navigation nodes found", result.nodes.len());
        debug!("{} markets found at root level", result.markets.len());
        Ok(result)
    }

    async fn get_market_navigation_node(
        &self,
        node_id: &str,
    ) -> Result<MarketNavigationResponse, AppError> {
        let path = format!("marketnavigation/{}", node_id);
        info!("Getting market navigation node: {}", node_id);
        let result: MarketNavigationResponse = self.http_client.get(&path, Some(1)).await?;
        debug!("{} child nodes found", result.nodes.len());
        debug!("{} markets found in node {}", result.markets.len(), node_id);
        Ok(result)
    }

    async fn get_all_markets(&self) -> Result<Vec<MarketData>, AppError> {
        let max_depth = 6;
        info!(
            "Starting comprehensive market hierarchy traversal (max {} levels)",
            max_depth
        );

        let root_response = self.get_market_navigation().await?;
        info!(
            "Root navigation: {} nodes, {} markets at top level",
            root_response.nodes.len(),
            root_response.markets.len()
        );

        let mut all_markets = root_response.markets.clone();
        let mut nodes_to_process = root_response.nodes.clone();
        let mut processed_levels = 0;

        while !nodes_to_process.is_empty() && processed_levels < max_depth {
            let mut next_level_nodes = Vec::new();
            let mut level_market_count = 0;

            info!(
                "Processing level {} with {} nodes",
                processed_levels,
                nodes_to_process.len()
            );

            for node in &nodes_to_process {
                match self.get_market_navigation_node(&node.id).await {
                    Ok(node_response) => {
                        let node_markets = node_response.markets.len();
                        let node_children = node_response.nodes.len();

                        if node_markets > 0 || node_children > 0 {
                            debug!(
                                "Node '{}' (level {}): {} markets, {} child nodes",
                                node.name, processed_levels, node_markets, node_children
                            );
                        }

                        all_markets.extend(node_response.markets);
                        level_market_count += node_markets;
                        next_level_nodes.extend(node_response.nodes);
                    }
                    Err(e) => {
                        tracing::error!(
                            "Failed to get markets for node '{}' at level {}: {:?}",
                            node.name,
                            processed_levels,
                            e
                        );
                    }
                }
            }

            info!(
                "Level {} completed: {} markets found, {} nodes for next level",
                processed_levels,
                level_market_count,
                next_level_nodes.len()
            );

            nodes_to_process = next_level_nodes;
            processed_levels += 1;
        }

        info!(
            "Market hierarchy traversal completed: {} total markets found across {} levels",
            all_markets.len(),
            processed_levels
        );

        Ok(all_markets)
    }

    async fn get_vec_db_entries(&self) -> Result<Vec<DBEntryResponse>, AppError> {
        info!("Getting all markets from hierarchy for DB entries");

        let all_markets = self.get_all_markets().await?;
        info!("Collected {} markets from hierarchy", all_markets.len());

        let mut vec_db_entries: Vec<DBEntryResponse> = all_markets
            .iter()
            .map(DBEntryResponse::from)
            .filter(|entry| !entry.epic.is_empty())
            .collect();

        info!("Created {} DB entries from markets", vec_db_entries.len());

        // Collect unique symbols
        let unique_symbols: std::collections::HashSet<String> = vec_db_entries
            .iter()
            .map(|entry| entry.symbol.clone())
            .filter(|symbol| !symbol.is_empty())
            .collect();

        info!(
            "Found {} unique symbols to fetch expiry dates for",
            unique_symbols.len()
        );

        let mut symbol_expiry_map: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();

        for symbol in unique_symbols {
            if let Some(entry) = vec_db_entries
                .iter()
                .find(|e| e.symbol == symbol && !e.epic.is_empty())
            {
                match self.get_market_details(&entry.epic).await {
                    Ok(market_details) => {
                        let expiry_date = market_details
                            .instrument
                            .expiry_details
                            .as_ref()
                            .map(|details| details.last_dealing_date.clone())
                            .unwrap_or_else(|| market_details.instrument.expiry.clone());

                        symbol_expiry_map.insert(symbol.clone(), expiry_date);
                        info!(
                            "Fetched expiry date for symbol {}: {}",
                            symbol,
                            symbol_expiry_map.get(&symbol).unwrap()
                        );
                    }
                    Err(e) => {
                        tracing::error!(
                            "Failed to get market details for epic {} (symbol {}): {:?}",
                            entry.epic,
                            symbol,
                            e
                        );
                        symbol_expiry_map.insert(symbol.clone(), entry.expiry.clone());
                    }
                }
            }
        }

        for entry in &mut vec_db_entries {
            if let Some(expiry_date) = symbol_expiry_map.get(&entry.symbol) {
                entry.expiry = expiry_date.clone();
            }
        }

        info!("Updated expiry dates for {} entries", vec_db_entries.len());
        Ok(vec_db_entries)
    }
}

#[async_trait]
impl AccountService for Client {
    async fn get_accounts(&self) -> Result<AccountsResponse, AppError> {
        info!("Getting account information");
        let result: AccountsResponse = self.http_client.get("accounts", Some(1)).await?;
        debug!(
            "Account information obtained: {} accounts",
            result.accounts.len()
        );
        Ok(result)
    }

    async fn get_positions(&self) -> Result<PositionsResponse, AppError> {
        debug!("Getting open positions");
        let result: PositionsResponse = self.http_client.get("positions", Some(2)).await?;
        debug!("Positions obtained: {} positions", result.positions.len());
        Ok(result)
    }

    async fn get_positions_w_filter(&self, filter: &str) -> Result<PositionsResponse, AppError> {
        debug!("Getting open positions with filter: {}", filter);
        let mut positions = self.get_positions().await?;

        positions
            .positions
            .retain(|position| position.market.epic.contains(filter));

        debug!(
            "Positions obtained after filtering: {} positions",
            positions.positions.len()
        );
        Ok(positions)
    }

    async fn get_working_orders(&self) -> Result<WorkingOrdersResponse, AppError> {
        info!("Getting working orders");
        let result: WorkingOrdersResponse = self.http_client.get("workingorders", Some(2)).await?;
        debug!(
            "Working orders obtained: {} orders",
            result.working_orders.len()
        );
        Ok(result)
    }

    async fn get_activity(
        &self,
        from: &str,
        to: &str,
    ) -> Result<AccountActivityResponse, AppError> {
        let path = format!("history/activity?from={}&to={}&pageSize=500", from, to);
        info!("Getting account activity");
        let result: AccountActivityResponse = self.http_client.get(&path, Some(3)).await?;
        debug!(
            "Account activity obtained: {} activities",
            result.activities.len()
        );
        Ok(result)
    }

    async fn get_activity_with_details(
        &self,
        from: &str,
        to: &str,
    ) -> Result<AccountActivityResponse, AppError> {
        let path = format!(
            "history/activity?from={}&to={}&detailed=true&pageSize=500",
            from, to
        );
        info!("Getting detailed account activity");
        let result: AccountActivityResponse = self.http_client.get(&path, Some(3)).await?;
        debug!(
            "Detailed account activity obtained: {} activities",
            result.activities.len()
        );
        Ok(result)
    }

    async fn get_transactions(
        &self,
        from: &str,
        to: &str,
    ) -> Result<TransactionHistoryResponse, AppError> {
        const PAGE_SIZE: u32 = 200;
        let mut all_transactions = Vec::new();
        let mut current_page = 1;
        #[allow(unused_assignments)]
        let mut last_metadata = None;

        loop {
            let path = format!(
                "history/transactions?from={}&to={}&pageSize={}&pageNumber={}",
                from, to, PAGE_SIZE, current_page
            );
            info!("Getting transaction history page {}", current_page);

            let result: TransactionHistoryResponse = self.http_client.get(&path, Some(2)).await?;

            let total_pages = result.metadata.page_data.total_pages as u32;
            last_metadata = Some(result.metadata);
            all_transactions.extend(result.transactions);

            if current_page >= total_pages {
                break;
            }
            current_page += 1;
        }

        debug!(
            "Total transaction history obtained: {} transactions",
            all_transactions.len()
        );

        Ok(TransactionHistoryResponse {
            transactions: all_transactions,
            metadata: last_metadata
                .ok_or_else(|| AppError::InvalidInput("Could not retrieve metadata".to_string()))?,
        })
    }
}

#[async_trait]
impl OrderService for Client {
    async fn create_order(
        &self,
        order: &CreateOrderRequest,
    ) -> Result<CreateOrderResponse, AppError> {
        info!("Creating order for: {}", order.epic);
        let result: CreateOrderResponse = self
            .http_client
            .post("positions/otc", order, Some(2))
            .await?;
        debug!("Order created with reference: {}", result.deal_reference);
        Ok(result)
    }

    async fn get_order_confirmation(
        &self,
        deal_reference: &str,
    ) -> Result<OrderConfirmationResponse, AppError> {
        let path = format!("confirms/{}", deal_reference);
        info!("Getting confirmation for order: {}", deal_reference);
        let result: OrderConfirmationResponse = self.http_client.get(&path, Some(1)).await?;
        debug!("Confirmation obtained for order: {}", deal_reference);
        Ok(result)
    }

    async fn get_order_confirmation_w_retry(
        &self,
        deal_reference: &str,
        retries: u64,
        delay_ms: u64,
    ) -> Result<OrderConfirmationResponse, AppError> {
        let mut attempts = 0;
        loop {
            match self.get_order_confirmation(deal_reference).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    attempts += 1;
                    if attempts > retries {
                        return Err(e);
                    }
                    warn!(
                        "Failed to get order confirmation (attempt {}/{}): {}. Retrying in {} ms...",
                        attempts, retries, e, delay_ms
                    );
                    sleep(Duration::from_millis(delay_ms)).await;
                }
            }
        }
    }

    async fn update_position(
        &self,
        deal_id: &str,
        update: &UpdatePositionRequest,
    ) -> Result<UpdatePositionResponse, AppError> {
        let path = format!("positions/otc/{}", deal_id);
        info!("Updating position: {}", deal_id);
        let result: UpdatePositionResponse = self.http_client.put(&path, update, Some(2)).await?;
        debug!(
            "Position updated: {} with deal reference: {}",
            deal_id, result.deal_reference
        );
        Ok(result)
    }

    async fn close_position(
        &self,
        close_request: &ClosePositionRequest,
    ) -> Result<ClosePositionResponse, AppError> {
        info!("Closing position");

        // IG API requires POST with _method: DELETE header for closing positions
        // This is a workaround for HTTP client limitations with DELETE + body
        let result: ClosePositionResponse = self
            .http_client
            .post_with_delete_method("positions/otc", close_request, Some(1))
            .await?;

        debug!("Position closed with reference: {}", result.deal_reference);
        Ok(result)
    }

    async fn create_working_order(
        &self,
        order: &CreateWorkingOrderRequest,
    ) -> Result<CreateWorkingOrderResponse, AppError> {
        info!("Creating working order for: {}", order.epic);
        let result: CreateWorkingOrderResponse = self
            .http_client
            .post("workingorders/otc", order, Some(2))
            .await?;
        debug!(
            "Working order created with reference: {}",
            result.deal_reference
        );
        Ok(result)
    }

    async fn delete_working_order(&self, deal_id: &str) -> Result<(), AppError> {
        let path = format!("workingorders/otc/{}", deal_id);
        let result: CreateWorkingOrderResponse =
            self.http_client.delete(path.as_str(), Some(2)).await?;
        debug!(
            "Working order created with reference: {}",
            result.deal_reference
        );
        Ok(())
    }
}

/// Streaming client for IG Markets real-time data.
///
/// This client manages two Lightstreamer connections for different data types:
/// - **Market streamer**: Handles market data (prices, market state), trade updates (CONFIRMS, OPU, WOU),
///   and account updates (positions, orders, balance). Uses the default adapter.
/// - **Price streamer**: Handles detailed price data (bid/ask levels, sizes, multiple currencies).
///   Uses the "Pricing" adapter.
///
/// Each connection type can be managed independently and runs in parallel.
pub struct StreamerClient {
    account_id: String,
    market_streamer_client: Option<Arc<Mutex<LightstreamerClient>>>,
    price_streamer_client: Option<Arc<Mutex<LightstreamerClient>>>,
    // Flags indicating whether there is at least one active subscription for each client
    has_market_stream_subs: bool,
    has_price_stream_subs: bool,
}

impl StreamerClient {
    /// Creates a new streaming client instance.
    ///
    /// This initializes both streaming clients (market and price) but does not
    /// establish connections yet. Connections are established when `connect()` is called.
    ///
    /// # Returns
    ///
    /// Returns a new `StreamerClient` instance or an error if initialization fails.
    pub async fn new() -> Result<Self, AppError> {
        let http_client_raw = Arc::new(RwLock::new(Client::new()));
        let http_client = http_client_raw.read().await;
        let ws_info = http_client.get_ws_info().await;
        let password = ws_info.get_ws_password();

        // Market data client (no adapter specified - uses default)
        let market_streamer_client = Arc::new(Mutex::new(LightstreamerClient::new(
            Some(ws_info.server.as_str()),
            None,
            Some(&ws_info.account_id),
            Some(&password),
        )?));

        let price_streamer_client = Arc::new(Mutex::new(LightstreamerClient::new(
            Some(ws_info.server.as_str()),
            None,
            Some(&ws_info.account_id),
            Some(&password),
        )?));

        // Force WebSocket streaming transport on both clients to satisfy IG requirements
        {
            let mut client = market_streamer_client.lock().await;
            client
                .connection_options
                .set_forced_transport(Some(Transport::WsStreaming));
        }
        {
            let mut client = price_streamer_client.lock().await;
            client
                .connection_options
                .set_forced_transport(Some(Transport::WsStreaming));
        }

        Ok(Self {
            account_id: ws_info.account_id.clone(),
            market_streamer_client: Some(market_streamer_client),
            price_streamer_client: Some(price_streamer_client),
            has_market_stream_subs: false,
            has_price_stream_subs: false,
        })
    }

    /// Creates a default streaming client instance.
    pub async fn default() -> Result<Self, AppError> {
        Self::new().await
    }

    /// Subscribes to market data updates for the specified instruments.
    ///
    /// This method creates a subscription to receive real-time market data updates
    /// for the given EPICs and returns a channel receiver for consuming the updates.
    ///
    /// # Arguments
    ///
    /// * `epics` - List of instrument EPICs to subscribe to
    /// * `fields` - Set of market data fields to receive (e.g., BID, OFFER, etc.)
    ///
    /// # Returns
    ///
    /// Returns a receiver channel for `PriceData` updates, or an error if
    /// the subscription setup failed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut receiver = client.market_subscribe(
    ///     vec!["IX.D.DAX.DAILY.IP".to_string()],
    ///     fields
    /// ).await?;
    ///
    /// tokio::spawn(async move {
    ///     while let Some(price_data) = receiver.recv().await {
    ///         println!("Price update: {:?}", price_data);
    ///     }
    /// });
    /// ```
    pub async fn market_subscribe(
        &mut self,
        epics: Vec<String>,
        fields: HashSet<StreamingMarketField>,
    ) -> Result<mpsc::UnboundedReceiver<PriceData>, AppError> {
        // Mark that we have at least one subscription on the market streamer
        self.has_market_stream_subs = true;

        let fields = get_streaming_market_fields(&fields);
        let market_epics: Vec<String> = epics
            .iter()
            .map(|epic| "MARKET:".to_string() + epic)
            .collect();
        let mut subscription =
            Subscription::new(SubscriptionMode::Merge, Some(market_epics), Some(fields))?;

        subscription.set_data_adapter(None)?;
        subscription.set_requested_snapshot(Some(Snapshot::Yes))?;

        // Create channel listener that converts ItemUpdate to PriceData
        let (listener, item_receiver) = ChannelSubscriptionListener::create_channel();
        subscription.add_listener(Box::new(listener));

        // Configure client and add subscription
        let client = self.market_streamer_client.as_ref().ok_or_else(|| {
            AppError::WebSocketError("market streamer client not initialized".to_string())
        })?;

        {
            let mut client = client.lock().await;
            client
                .connection_options
                .set_forced_transport(Some(Transport::WsStreaming));
            LightstreamerClient::subscribe(client.subscription_sender.clone(), subscription).await;
        }

        // Create a channel for PriceData and spawn a task to convert ItemUpdate to PriceData
        let (price_tx, price_rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            let mut receiver = item_receiver;
            while let Some(item_update) = receiver.recv().await {
                let price_data = PriceData::from(&item_update);
                let _ = price_tx.send(price_data);
            }
        });

        info!(
            "Market subscription created for {} instruments",
            epics.len()
        );
        Ok(price_rx)
    }

    /// Subscribes to trade updates for the account.
    ///
    /// This method creates a subscription to receive real-time trade confirmations,
    /// order updates (OPU), and working order updates (WOU) for the account,
    /// and returns a channel receiver for consuming the updates.
    ///
    /// # Returns
    ///
    /// Returns a receiver channel for `TradeFields` updates, or an error if
    /// the subscription setup failed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut receiver = client.trade_subscribe().await?;
    ///
    /// tokio::spawn(async move {
    ///     while let Some(trade_fields) = receiver.recv().await {
    ///         println!("Trade update: {:?}", trade_fields);
    ///     }
    /// });
    /// ```
    pub async fn trade_subscribe(
        &mut self,
    ) -> Result<mpsc::UnboundedReceiver<TradeFields>, AppError> {
        // Mark that we have at least one subscription on the market streamer
        self.has_market_stream_subs = true;

        let account_id = self.account_id.clone();
        let fields = Some(vec![
            "CONFIRMS".to_string(),
            "OPU".to_string(),
            "WOU".to_string(),
        ]);
        let trade_items = vec![format!("TRADE:{account_id}")];

        let mut subscription =
            Subscription::new(SubscriptionMode::Distinct, Some(trade_items), fields)?;

        subscription.set_data_adapter(None)?;
        subscription.set_requested_snapshot(Some(Snapshot::Yes))?;

        // Create channel listener
        let (listener, item_receiver) = ChannelSubscriptionListener::create_channel();
        subscription.add_listener(Box::new(listener));

        // Configure client and add subscription (reusing market_streamer_client)
        let client = self.market_streamer_client.as_ref().ok_or_else(|| {
            AppError::WebSocketError("market streamer client not initialized".to_string())
        })?;

        {
            let mut client = client.lock().await;
            client
                .connection_options
                .set_forced_transport(Some(Transport::WsStreaming));
            LightstreamerClient::subscribe(client.subscription_sender.clone(), subscription).await;
        }

        // Create a channel for TradeFields and spawn a task to convert ItemUpdate to TradeFields
        let (trade_tx, trade_rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            let mut receiver = item_receiver;
            while let Some(item_update) = receiver.recv().await {
                let trade_data = crate::presentation::trade::TradeData::from(&item_update);
                let _ = trade_tx.send(trade_data.fields);
            }
        });

        info!("Trade subscription created for account: {}", account_id);
        Ok(trade_rx)
    }

    /// Subscribes to account data updates.
    ///
    /// This method creates a subscription to receive real-time account updates including
    /// profit/loss, margin, equity, available funds, and other account metrics,
    /// and returns a channel receiver for consuming the updates.
    ///
    /// # Arguments
    ///
    /// * `fields` - Set of account data fields to receive (e.g., PNL, MARGIN, EQUITY, etc.)
    ///
    /// # Returns
    ///
    /// Returns a receiver channel for `AccountFields` updates, or an error if
    /// the subscription setup failed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut receiver = client.account_subscribe(fields).await?;
    ///
    /// tokio::spawn(async move {
    ///     while let Some(account_fields) = receiver.recv().await {
    ///         println!("Account update: {:?}", account_fields);
    ///     }
    /// });
    /// ```
    pub async fn account_subscribe(
        &mut self,
        fields: HashSet<StreamingAccountDataField>,
    ) -> Result<mpsc::UnboundedReceiver<AccountFields>, AppError> {
        // Mark that we have at least one subscription on the market streamer
        self.has_market_stream_subs = true;

        let fields = get_streaming_account_data_fields(&fields);
        let account_id = self.account_id.clone();
        let account_items = vec![format!("ACCOUNT:{account_id}")];

        let mut subscription =
            Subscription::new(SubscriptionMode::Merge, Some(account_items), Some(fields))?;

        subscription.set_data_adapter(None)?;
        subscription.set_requested_snapshot(Some(Snapshot::Yes))?;

        // Create channel listener
        let (listener, item_receiver) = ChannelSubscriptionListener::create_channel();
        subscription.add_listener(Box::new(listener));

        // Configure client and add subscription (reusing market_streamer_client)
        let client = self.market_streamer_client.as_ref().ok_or_else(|| {
            AppError::WebSocketError("market streamer client not initialized".to_string())
        })?;

        {
            let mut client = client.lock().await;
            client
                .connection_options
                .set_forced_transport(Some(Transport::WsStreaming));
            LightstreamerClient::subscribe(client.subscription_sender.clone(), subscription).await;
        }

        // Create a channel for AccountFields and spawn a task to convert ItemUpdate to AccountFields
        let (account_tx, account_rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            let mut receiver = item_receiver;
            while let Some(item_update) = receiver.recv().await {
                let account_data = crate::presentation::account::AccountData::from(&item_update);
                let _ = account_tx.send(account_data.fields);
            }
        });

        info!("Account subscription created for account: {}", account_id);
        Ok(account_rx)
    }

    /// Subscribes to price data updates for the specified instruments.
    ///
    /// This method creates a subscription to receive real-time price updates including
    /// bid/ask prices, sizes, and multiple currency levels for the given EPICs,
    /// and returns a channel receiver for consuming the updates.
    ///
    /// # Arguments
    ///
    /// * `epics` - List of instrument EPICs to subscribe to
    /// * `fields` - Set of price data fields to receive (e.g., BID_PRICE1, ASK_PRICE1, etc.)
    ///
    /// # Returns
    ///
    /// Returns a receiver channel for `PriceData` updates, or an error if
    /// the subscription setup failed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut receiver = client.price_subscribe(
    ///     vec!["IX.D.DAX.DAILY.IP".to_string()],
    ///     fields
    /// ).await?;
    ///
    /// tokio::spawn(async move {
    ///     while let Some(price_data) = receiver.recv().await {
    ///         println!("Price update: {:?}", price_data);
    ///     }
    /// });
    /// ```
    pub async fn price_subscribe(
        &mut self,
        epics: Vec<String>,
        fields: HashSet<StreamingPriceField>,
    ) -> Result<mpsc::UnboundedReceiver<PriceData>, AppError> {
        // Mark that we have at least one subscription on the price streamer
        self.has_price_stream_subs = true;

        let fields = get_streaming_price_fields(&fields);
        let account_id = self.account_id.clone();
        let price_epics: Vec<String> = epics
            .iter()
            .map(|epic| format!("PRICE:{account_id}:{epic}"))
            .collect();

        // Debug what we are about to subscribe to (items and fields)
        tracing::debug!("Pricing subscribe items: {:?}", price_epics);
        tracing::debug!("Pricing subscribe fields: {:?}", fields);

        let mut subscription =
            Subscription::new(SubscriptionMode::Merge, Some(price_epics), Some(fields))?;

        // Allow overriding the Pricing adapter name via env var to match server config
        let pricing_adapter =
            std::env::var("IG_PRICING_ADAPTER").unwrap_or_else(|_| "Pricing".to_string());
        tracing::debug!("Using Pricing data adapter: {}", pricing_adapter);
        subscription.set_data_adapter(Some(pricing_adapter))?;
        subscription.set_requested_snapshot(Some(Snapshot::Yes))?;

        // Create channel listener
        let (listener, item_receiver) = ChannelSubscriptionListener::create_channel();
        subscription.add_listener(Box::new(listener));

        // Configure client and add subscription
        let client = self.price_streamer_client.as_ref().ok_or_else(|| {
            AppError::WebSocketError("price streamer client not initialized".to_string())
        })?;

        {
            let mut client = client.lock().await;
            client
                .connection_options
                .set_forced_transport(Some(Transport::WsStreaming));
            LightstreamerClient::subscribe(client.subscription_sender.clone(), subscription).await;
        }

        // Create a channel for PriceData and spawn a task to convert ItemUpdate to PriceData
        let (price_tx, price_rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            let mut receiver = item_receiver;
            while let Some(item_update) = receiver.recv().await {
                let price_data = PriceData::from(&item_update);
                let _ = price_tx.send(price_data);
            }
        });

        info!(
            "Price subscription created for {} instruments (account: {})",
            epics.len(),
            account_id
        );
        Ok(price_rx)
    }

    /// Connects all active Lightstreamer clients and maintains the connections.
    ///
    /// This method establishes connections for all streaming clients that have active
    /// subscriptions (market and price). Each client runs in its own task and
    /// all connections are maintained until a shutdown signal is received.
    ///
    /// # Arguments
    ///
    /// * `shutdown_signal` - Optional signal to gracefully shutdown all connections.
    ///   If None, a default signal handler for SIGINT/SIGTERM will be created.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when all connections are closed gracefully, or an error if
    /// any connection fails after maximum retry attempts.
    ///
    pub async fn connect(&mut self, shutdown_signal: Option<Arc<Notify>>) -> Result<(), AppError> {
        // Use provided signal or create a new one with signal hooks
        let signal = if let Some(sig) = shutdown_signal {
            sig
        } else {
            let sig = Arc::new(Notify::new());
            setup_signal_hook(Arc::clone(&sig)).await;
            sig
        };

        let mut tasks = Vec::new();

        // Connect market streamer only if there are active subscriptions
        if self.has_market_stream_subs {
            if let Some(client) = self.market_streamer_client.as_ref() {
                let client = Arc::clone(client);
                let signal = Arc::clone(&signal);
                let task =
                    tokio::spawn(
                        async move { Self::connect_client(client, signal, "Market").await },
                    );
                tasks.push(task);
            }
        } else {
            info!("Skipping Market streamer connection: no active subscriptions");
        }

        // Connect price streamer only if there are active subscriptions
        if self.has_price_stream_subs {
            if let Some(client) = self.price_streamer_client.as_ref() {
                let client = Arc::clone(client);
                let signal = Arc::clone(&signal);
                let task =
                    tokio::spawn(
                        async move { Self::connect_client(client, signal, "Price").await },
                    );
                tasks.push(task);
            }
        } else {
            info!("Skipping Price streamer connection: no active subscriptions");
        }

        if tasks.is_empty() {
            warn!("No streaming clients selected for connection (no active subscriptions)");
            return Ok(());
        }

        info!("Connecting {} streaming client(s)...", tasks.len());

        // Wait for all tasks to complete
        let results = futures::future::join_all(tasks).await;

        // Check if any task failed
        let mut has_error = false;
        for (idx, result) in results.iter().enumerate() {
            match result {
                Ok(Ok(_)) => {
                    debug!("Streaming client {} completed successfully", idx);
                }
                Ok(Err(e)) => {
                    error!("Streaming client {} failed: {:?}", idx, e);
                    has_error = true;
                }
                Err(e) => {
                    error!("Streaming client {} task panicked: {:?}", idx, e);
                    has_error = true;
                }
            }
        }

        if has_error {
            return Err(AppError::WebSocketError(
                "one or more streaming connections failed".to_string(),
            ));
        }

        info!("All streaming connections closed gracefully");
        Ok(())
    }

    /// Internal helper to connect a single Lightstreamer client with retry logic.
    async fn connect_client(
        client: Arc<Mutex<LightstreamerClient>>,
        signal: Arc<Notify>,
        client_type: &str,
    ) -> Result<(), AppError> {
        let mut retry_interval_millis: u64 = 0;
        let mut retry_counter: u64 = 0;

        while retry_counter < MAX_CONNECTION_ATTEMPTS {
            let connect_result = {
                let mut client = client.lock().await;
                client.connect_direct(Arc::clone(&signal)).await
            };

            // Convert error to String immediately to avoid Send issues
            let result_with_string_error = connect_result.map_err(|e| format!("{:?}", e));

            match result_with_string_error {
                Ok(_) => {
                    info!("{} streamer connected successfully", client_type);
                    break;
                }
                Err(error_msg) => {
                    // If server closed because there are no active subscriptions, treat as graceful
                    if error_msg.contains("No more requests to fulfill") {
                        info!(
                            "{} streamer closed gracefully: no active subscriptions (server reason: No more requests to fulfill)",
                            client_type
                        );
                        return Ok(());
                    }

                    error!("{} streamer connection failed: {}", client_type, error_msg);

                    if retry_counter < MAX_CONNECTION_ATTEMPTS - 1 {
                        tokio::time::sleep(std::time::Duration::from_millis(retry_interval_millis))
                            .await;
                        retry_interval_millis =
                            (retry_interval_millis + (200 * retry_counter)).min(5000);
                        retry_counter += 1;
                        warn!(
                            "{} streamer retrying (attempt {}/{}) in {:.2} seconds...",
                            client_type,
                            retry_counter + 1,
                            MAX_CONNECTION_ATTEMPTS,
                            retry_interval_millis as f64 / 1000.0
                        );
                    } else {
                        retry_counter += 1;
                    }
                }
            }
        }

        if retry_counter >= MAX_CONNECTION_ATTEMPTS {
            error!(
                "{} streamer failed after {} attempts",
                client_type, MAX_CONNECTION_ATTEMPTS
            );
            return Err(AppError::WebSocketError(format!(
                "{} streamer: maximum connection attempts ({}) exceeded",
                client_type, MAX_CONNECTION_ATTEMPTS
            )));
        }

        info!("{} streamer connection closed gracefully", client_type);
        Ok(())
    }

    /// Disconnects all active Lightstreamer clients.
    ///
    /// This method gracefully closes all streaming connections (market and price).
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all disconnections were successful.
    pub async fn disconnect(&mut self) -> Result<(), AppError> {
        let mut disconnected = 0;

        if let Some(client) = self.market_streamer_client.as_ref() {
            let mut client = client.lock().await;
            client.disconnect().await;
            info!("Market streamer disconnected");
            disconnected += 1;
        }

        if let Some(client) = self.price_streamer_client.as_ref() {
            let mut client = client.lock().await;
            client.disconnect().await;
            info!("Price streamer disconnected");
            disconnected += 1;
        }

        info!("Disconnected {} streaming client(s)", disconnected);
        Ok(())
    }
}
