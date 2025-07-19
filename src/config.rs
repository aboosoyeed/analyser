/// Configuration structures for customizable behavior.
///
/// This module provides configuration options for various aspects of the chess
/// analyzer, allowing users to customize engine settings, display options, and
/// default behaviors without modifying the source code.

use crate::constants::defaults;

/// Color scheme configuration for board display.
#[derive(Debug, Clone)]
pub struct ColorScheme {
    /// Color code for white pieces
    pub white_pieces: String,
    /// Color code for black pieces  
    pub black_pieces: String,
    /// Color code for empty squares
    pub empty_squares: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            white_pieces: "yellow".to_string(),
            black_pieces: "blue".to_string(),
            empty_squares: "gray".to_string(),
        }
    }
}

/// Engine configuration settings.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Search depth for position analysis
    pub depth: u8,
    /// Timeout for engine operations in milliseconds
    pub timeout_ms: Option<u32>,
    /// Engine executable path (defaults to "stockfish" in PATH)
    pub engine_path: Option<String>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            depth: defaults::DEFAULT_ENGINE_DEPTH,
            timeout_ms: None,
            engine_path: None,
        }
    }
}

/// File path configuration.
#[derive(Debug, Clone)]
pub struct FileConfig {
    /// Default PGN file path for analysis
    pub default_pgn_path: String,
    /// Default output directory for generated files
    pub output_directory: Option<String>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            default_pgn_path: defaults::DEFAULT_PGN_PATH.to_string(),
            output_directory: None,
        }
    }
}

/// Main configuration struct containing all customizable settings.
/// 
/// This struct allows users to configure various aspects of the chess analyzer
/// including engine behavior, display settings, and file paths.
/// 
/// # Examples
/// 
/// ```rust
/// use analyzer::config::{Config, EngineConfig};
/// 
/// // Use default configuration
/// let config = Config::default();
/// 
/// // Customize engine depth
/// let mut custom_config = Config::default();
/// custom_config.engine.depth = 20;
/// 
/// // Custom configuration with specific settings
/// let config = Config {
///     engine: EngineConfig {
///         depth: 18,
///         timeout_ms: Some(5000),
///         engine_path: Some("/usr/local/bin/stockfish".to_string()),
///     },
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Engine-related configuration
    pub engine: EngineConfig,
    /// Display and color configuration
    pub display: ColorScheme,
    /// File path configuration
    pub files: FileConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            engine: EngineConfig::default(),
            display: ColorScheme::default(),
            files: FileConfig::default(),
        }
    }
}

impl Config {
    /// Creates a new Config with default values.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Creates a Config optimized for fast analysis.
    /// 
    /// Uses lower search depth for quicker results.
    pub fn fast() -> Self {
        Self {
            engine: EngineConfig {
                depth: 8,
                timeout_ms: Some(2000),
                engine_path: None,
            },
            ..Default::default()
        }
    }
    
    /// Creates a Config optimized for deep analysis.
    /// 
    /// Uses higher search depth for more accurate results.
    pub fn deep() -> Self {
        Self {
            engine: EngineConfig {
                depth: 22,
                timeout_ms: Some(30000),
                engine_path: None,
            },
            ..Default::default()
        }
    }
    
    /// Sets a custom PGN file path.
    pub fn with_pgn_path(mut self, path: String) -> Self {
        self.files.default_pgn_path = path;
        self
    }
    
    /// Sets a custom engine depth.
    pub fn with_engine_depth(mut self, depth: u8) -> Self {
        self.engine.depth = depth;
        self
    }
}