/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 30/10/25
******************************************************************************/

//! Dynamic market streaming with thread-safe subscription management.
//!
//! This module provides a wrapper around `StreamerClient` that allows dynamic
//! addition and removal of market subscriptions from multiple threads.

use crate::application::client::StreamerClient;
use crate::error::AppError;
use crate::model::streaming::StreamingMarketField;
use crate::presentation::price::PriceData;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{Notify, RwLock, mpsc};
use tracing::{debug, info, warn};

/// Dynamic market streamer with thread-safe subscription management.
///
/// This struct wraps a `StreamerClient` and provides methods to dynamically
/// add, remove, and clear market subscriptions. All operations are thread-safe
/// and can be called from multiple threads concurrently.
///
/// # Examples
///
/// ```ignore
/// use ig_client::application::dynamic_streamer::DynamicMarketStreamer;
/// use ig_client::model::streaming::StreamingMarketField;
/// use std::collections::HashSet;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create fields to subscribe to
///     let fields = HashSet::from([
///         StreamingMarketField::Bid,
///         StreamingMarketField::Offer,
///     ]);
///
///     // Create the dynamic streamer
///     let mut streamer = DynamicMarketStreamer::new(fields).await?;
///
///     // Get the receiver for price updates
///     let mut receiver = streamer.get_receiver().await?;
///
///     // Add markets from different threads
///     let streamer_clone = streamer.clone();
///     tokio::spawn(async move {
///         streamer_clone.add("IX.D.DAX.DAILY.IP".to_string()).await.unwrap();
///     });
///
///     // Start receiving updates
///     tokio::spawn(async move {
///         while let Some(price_data) = receiver.recv().await {
///             println!("Price update: {}", price_data);
///         }
///     });
///
///     // Connect and run
///     streamer.connect(None).await?;
///     Ok(())
/// }
/// ```
pub struct DynamicMarketStreamer {
    /// Internal streamer client (recreated on epic changes)
    client: Arc<RwLock<Option<StreamerClient>>>,
    /// Set of EPICs currently subscribed
    epics: Arc<RwLock<HashSet<String>>>,
    /// Market fields to subscribe to
    fields: HashSet<StreamingMarketField>,
    /// Channel sender for price updates
    price_tx: Arc<RwLock<Option<mpsc::UnboundedSender<PriceData>>>>,
    /// Channel receiver for price updates (taken on first get_receiver call)
    price_rx: Arc<RwLock<Option<mpsc::UnboundedReceiver<PriceData>>>>,
    /// Flag indicating if the streamer is connected
    is_connected: Arc<RwLock<bool>>,
    /// Shutdown signal for current connection
    shutdown_signal: Arc<RwLock<Option<Arc<Notify>>>>,
}

impl DynamicMarketStreamer {
    /// Creates a new dynamic market streamer.
    ///
    /// # Arguments
    ///
    /// * `fields` - Set of market data fields to receive (e.g., BID, OFFER, etc.)
    ///
    /// # Returns
    ///
    /// Returns a new `DynamicMarketStreamer` instance or an error if initialization fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let fields = HashSet::from([
    ///     StreamingMarketField::Bid,
    ///     StreamingMarketField::Offer,
    /// ]);
    /// let streamer = DynamicMarketStreamer::new(fields).await?;
    /// ```
    pub async fn new(fields: HashSet<StreamingMarketField>) -> Result<Self, AppError> {
        let (price_tx, price_rx) = mpsc::unbounded_channel();

        Ok(Self {
            client: Arc::new(RwLock::new(None)),
            epics: Arc::new(RwLock::new(HashSet::new())),
            fields,
            price_tx: Arc::new(RwLock::new(Some(price_tx))),
            price_rx: Arc::new(RwLock::new(Some(price_rx))),
            is_connected: Arc::new(RwLock::new(false)),
            shutdown_signal: Arc::new(RwLock::new(None)),
        })
    }

    /// Adds a market EPIC to the subscription list.
    ///
    /// If the streamer is already connected, this will reconnect with the updated list.
    ///
    /// # Arguments
    ///
    /// * `epic` - The market EPIC to subscribe to
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the EPIC was added successfully.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// streamer.add("IX.D.DAX.DAILY.IP".to_string()).await?;
    /// ```
    pub async fn add(&self, epic: String) -> Result<(), AppError> {
        let mut epics = self.epics.write().await;

        if epics.contains(&epic) {
            debug!("EPIC {} already subscribed", epic);
            return Ok(());
        }

        epics.insert(epic.clone());
        info!("Added EPIC {} to subscription list", epic);
        drop(epics); // Release lock

        // If already connected, reconnect with new list
        let is_connected = *self.is_connected.read().await;
        if is_connected {
            self.reconnect().await?;
        }

        Ok(())
    }

    /// Removes a market EPIC from the subscription list.
    ///
    /// If the streamer is already connected, this will reconnect with the updated list.
    ///
    /// # Arguments
    ///
    /// * `epic` - The market EPIC to remove
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the EPIC was removed successfully.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// streamer.remove("IX.D.DAX.DAILY.IP".to_string()).await?;
    /// ```
    pub async fn remove(&self, epic: String) -> Result<(), AppError> {
        let mut epics = self.epics.write().await;

        let was_removed = epics.remove(&epic);
        if was_removed {
            info!("Removed EPIC {} from subscription list", epic);
        } else {
            debug!("EPIC {} was not in subscription list", epic);
        }
        drop(epics); // Release lock

        // If already connected and something was removed, reconnect
        if was_removed {
            let is_connected = *self.is_connected.read().await;
            if is_connected {
                self.reconnect().await?;
            }
        }

        Ok(())
    }

    /// Clears all market EPICs from the subscription list.
    ///
    /// Note: Due to Lightstreamer limitations, this does not unsubscribe from
    /// the server immediately. All EPICs will be removed from the internal list.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when all EPICs have been cleared.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// streamer.clear().await?;
    /// ```
    pub async fn clear(&self) -> Result<(), AppError> {
        let mut epics = self.epics.write().await;
        let count = epics.len();
        epics.clear();
        info!("Cleared {} EPICs from subscription list", count);
        Ok(())
    }

    /// Gets the current list of subscribed EPICs.
    ///
    /// # Returns
    ///
    /// Returns a vector containing all currently subscribed EPICs.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let epics = streamer.get_epics().await;
    /// println!("Subscribed to {} markets", epics.len());
    /// ```
    pub async fn get_epics(&self) -> Vec<String> {
        let epics = self.epics.read().await;
        epics.iter().cloned().collect()
    }

    /// Gets the receiver for price updates.
    ///
    /// This method can only be called once. Subsequent calls will return an error.
    ///
    /// # Returns
    ///
    /// Returns a receiver channel for `PriceData` updates, or an error if the
    /// receiver has already been taken.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut receiver = streamer.get_receiver().await?;
    /// tokio::spawn(async move {
    ///     while let Some(price_data) = receiver.recv().await {
    ///         println!("Price update: {}", price_data);
    ///     }
    /// });
    /// ```
    pub async fn get_receiver(&self) -> Result<mpsc::UnboundedReceiver<PriceData>, AppError> {
        let mut rx_lock = self.price_rx.write().await;
        rx_lock
            .take()
            .ok_or_else(|| AppError::InvalidInput("Receiver already taken".to_string()))
    }

    /// Reconnects the streamer with the current list of EPICs.
    ///
    /// This method disconnects the current client and creates a new one with
    /// the updated EPIC list.
    async fn reconnect(&self) -> Result<(), AppError> {
        info!("Reconnecting with updated EPIC list...");

        // Signal shutdown to current client
        {
            let shutdown_lock = self.shutdown_signal.read().await;
            if let Some(signal) = shutdown_lock.as_ref() {
                signal.notify_one();
            }
        }

        // Wait a bit for graceful shutdown
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Start new connection
        let epics = self.get_epics().await;
        if !epics.is_empty() {
            self.start_internal().await?;
        }

        Ok(())
    }

    /// Internal method to start connection.
    async fn start_internal(&self) -> Result<(), AppError> {
        let epics = self.get_epics().await;

        if epics.is_empty() {
            warn!("No EPICs to subscribe to");
            return Ok(());
        }

        info!("Starting connection with {} EPICs", epics.len());

        // Create new client
        let mut new_client = StreamerClient::new().await?;

        // Subscribe to all EPICs
        let fields = self.fields.clone();
        let mut receiver = new_client.market_subscribe(epics.clone(), fields).await?;

        // Forward updates to the main channel
        let price_tx = self.price_tx.read().await;
        if let Some(tx) = price_tx.as_ref() {
            let tx = tx.clone();
            tokio::spawn(async move {
                while let Some(price_data) = receiver.recv().await {
                    if tx.send(price_data).is_err() {
                        warn!("Failed to send price update: receiver dropped");
                        break;
                    }
                }
                debug!("Subscription forwarding task ended");
            });
        }

        // Store the new client
        *self.client.write().await = Some(new_client);

        // Create new shutdown signal
        let signal = Arc::new(Notify::new());
        *self.shutdown_signal.write().await = Some(Arc::clone(&signal));

        // Mark as connected
        *self.is_connected.write().await = true;

        // Spawn connection task in background
        let client = Arc::clone(&self.client);
        let is_connected = Arc::clone(&self.is_connected);

        tokio::spawn(async move {
            let result = {
                let mut client_guard = client.write().await;
                if let Some(ref mut c) = *client_guard {
                    c.connect(Some(signal)).await
                } else {
                    Ok(())
                }
            };

            // Mark as disconnected
            *is_connected.write().await = false;

            match result {
                Ok(_) => info!("Connection task completed successfully"),
                Err(e) => tracing::error!("Connection task failed: {:?}", e),
            }
        });

        info!("Connection task started in background");
        Ok(())
    }

    /// Starts the connection to the Lightstreamer server and subscribes to all initial EPICs.
    ///
    /// This method subscribes to all EPICs in the subscription list and then spawns a background
    /// task to maintain the connection. This allows dynamic subscription management while connected.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` immediately after starting the connection task.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Start connection
    /// streamer.start().await?;
    ///
    /// // Keep main thread alive
    /// tokio::signal::ctrl_c().await?;
    /// ```
    pub async fn start(&mut self) -> Result<(), AppError> {
        self.start_internal().await
    }

    /// Connects to the Lightstreamer server and blocks until shutdown.
    ///
    /// This is a convenience method that calls `start()` and then waits for a shutdown signal.
    /// Use `start()` if you need non-blocking behavior.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the connection is closed gracefully.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Connect and block until shutdown
    /// streamer.connect().await?;
    /// ```
    pub async fn connect(&mut self) -> Result<(), AppError> {
        self.start().await?;

        // Wait for SIGINT/SIGTERM
        use lightstreamer_rs::utils::setup_signal_hook;
        let signal = Arc::new(Notify::new());
        setup_signal_hook(Arc::clone(&signal)).await;
        signal.notified().await;

        // Disconnect
        self.disconnect().await?;

        Ok(())
    }

    /// Disconnects from the Lightstreamer server.
    ///
    /// This method gracefully closes the connection to the server.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the disconnection was successful.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// streamer.disconnect().await?;
    /// ```
    pub async fn disconnect(&mut self) -> Result<(), AppError> {
        // Signal shutdown
        {
            let shutdown_lock = self.shutdown_signal.read().await;
            if let Some(signal) = shutdown_lock.as_ref() {
                signal.notify_one();
            }
        }

        // Disconnect client
        let mut client_lock = self.client.write().await;
        if let Some(ref mut client) = *client_lock {
            client.disconnect().await?;
        }
        *client_lock = None;

        *self.is_connected.write().await = false;
        info!("Disconnected from Lightstreamer server");
        Ok(())
    }
}

impl Clone for DynamicMarketStreamer {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            epics: Arc::clone(&self.epics),
            fields: self.fields.clone(),
            price_tx: Arc::clone(&self.price_tx),
            price_rx: Arc::clone(&self.price_rx),
            is_connected: Arc::clone(&self.is_connected),
            shutdown_signal: Arc::clone(&self.shutdown_signal),
        }
    }
}
