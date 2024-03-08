# RustMeteredAPI

RustMeteredAPI is a flexible and efficient API framework built in Rust, designed to provide developers with a metered approach to managing their APIs. Whether you're offering limited free access or premium metered services, RustMeteredAPI offers the tools you need to control usage and monetize your API effectively.

## Features

- **Metered Usage**: Control access to your API based on predefined usage limits.
- **Free Tier**: Offer a limited free tier with restricted functionality to attract users.
- **Premium Tier**: Unlock higher functionality and generate more keys with the premium metered version.
- **Flexible Pricing**: Price your premium tier according to the usage and features offered.
- **Efficient Performance**: Built with Rust's performance and safety in mind, ensuring optimal performance even under heavy loads.

## Getting Started

To get started with RustMeteredAPI, follow these simple steps:

1. **Startup**: Building the cargo app:
    first clone the repository in your desired folder
   ```console
   cd MeteredApiRust
   cargo build
   cargo run
    ```

2. **Server Start**: The database used is surrealdb. install it from official documents:
    starting surreal server
   ```console
    surreal start memory -A --auth --user root --pass root --bind 0.0.0.0:8001
    ```
3. **Adding Routes**: Currently all routes are setup on the main.rs, you can add from there and will be moved in future version 

4. **Monetization**: Implement logic to handle premium tier usage and monetization strategies according to your business model.

## Pricing

Pricing for RustMeteredAPI's premium tier is flexible and can be tailored to suit your specific requirements. Whether you choose a pay-as-you-go model or subscription-based pricing, RustMeteredAPI provides the tools to manage billing and track usage effectively.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve RustMeteredAPI and make it even better.
