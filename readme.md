copbot_rs
├── Cargo.toml          # Rust package definition and dependencies
├── resources
│   └── Copbot.glade    # GTK interface layout file
└── src
    ├── main.rs         # Main application entry point
    ├── gui.rs          # Handles all GTK GUI logic and event handling
    ├── bot.rs          # Core bot logic, orchestrates the purchase process
    ├── req_handler.rs  # Manages HTTP requests and browser automation for checkout
    ├── bot_utils.rs    # Helper functions and data structures for the bot
    ├── profileHandler.rs # Manages saving and loading of user profiles
    └── benchmark.rs    # (Empty) Placeholder for performance tests```

## Key Dependencies

This project relies on several key Rust crates:

*   `gtk`, `glib`, `gdk-pixbuf`: For building the graphical user interface.
*   `reqwest`: An ergonomic, asynchronous HTTP client for interacting with web APIs.
*   `tokio`: The asynchronous runtime for handling concurrent operations.
*   `headless_chrome`: For browser automation and handling JavaScript-heavy parts of the checkout.
*   `scraper`, `soup`: For parsing and extracting data from HTML.
*   `serde`, `serde_json`: For serializing and deserializing profile data to and from `data.json`.
*   `chrono`: For handling date and time, especially for the drop timer.

## Setup and Installation (Historical)

To run this project, you would have needed the following:

1.  **Rust Toolchain:** Install Rust via `rustup`.
2.  **GTK Libraries:** Install the GTK3 development libraries for your operating system.
3.  **Build the Project:**
    ```bash
    git clone <repository-url>
    cd copbot_rs
    cargo build --release
    ```
4.  **Run:** The executable would be located at `target/release/Copbot_rs`. A `data.json` file would be created in the same directory to store profiles.
