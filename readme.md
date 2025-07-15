# Copbot_rs

Copbot_rs is a proof-of-concept automated checkout bot written in Rust. It was designed to interact with the Supreme New York webstore to automate the process of finding and purchasing items upon their release. The application features a graphical user interface (GUI) built with GTK for managing profiles and initiating the bot.

**Disclaimer:** This is an old project and is no longer maintained. The website selectors, API endpoints, and checkout process it targets have likely changed, meaning **this bot will not work** in its current state. It is presented here for archival and educational purposes only.

## Features

*   **Graphical User Interface:** A simple GUI built with `gtk-rs` allows for easy configuration and operation.
*   **Profile Management:** Users can create, save, and manage multiple profiles for:
    *   Shipping information (name, address, etc.)
    *   Billing information (credit card details)
    *   Google (Gmail) accounts for automated login.
*   **Item Search:** Finds items based on user-provided keywords for the product name and color.
*   **Size and Preference Selection:** Allows users to specify a desired size ("Small", "Medium", "Large", etc.) or "Any".
*   **Automated Checkout:** The bot is designed to automatically add an item to the cart and fill in all checkout fields using the selected profiles.
*   **Drop Timer:** Includes a feature to wait for a specific, hardcoded drop time before starting the process.
*   **Hybrid Automation:** Utilizes a combination of direct HTTP requests with `reqwest` for speed (e.g., fetching product lists) and browser automation with `headless_chrome` for complex JavaScript-driven actions during checkout.

## How It Works

The bot's workflow is divided into several stages:

1.  **Initialization:** The application starts up, displaying the main GTK window and a separate log window.
2.  **Profile Configuration:** The user can add shipping, billing, and Gmail profiles via the "Profiles" menu. This information is serialized and stored locally in a `data.json` file.
3.  **Task Setup:** On the main window, the user inputs keywords for the desired item and its color, selects a size, and chooses the profiles to use for the attempt.
4.  **Bot Start:**
    *   If the "Wait for drop" option is checked, the bot will enter a countdown state, waiting for a hardcoded time before proceeding.
    *   The core bot logic runs in a separate thread to keep the GUI responsive.
5.  **Item Discovery:**
    *   The bot makes an HTTP request to the site's `mobile_stock.json` API to get a list of all available products.
    *   It then uses a keyword-matching algorithm (`bot_utils.rs`) to find the item that best matches the user's input.
6.  **Add to Cart & Checkout:**
    *   Once the target item and its specific style/size ID are identified, the bot launches a `headless_chrome` instance.
    *   If a Gmail profile is selected, it first logs into Google to establish an authenticated session, which can help with solving CAPTCHAs.
    *   It navigates to the product page, adds the item to the cart, and proceeds to the checkout page.
    *   It automatically fills all shipping and payment forms with the information from the selected profiles, simulating human typing to appear more legitimate.
    *   Finally, it attempts to submit the order.
7.  **Logging:** All actions, from startup to checkout status, are reported in real-time in the log window.

## Project Structure

```
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
