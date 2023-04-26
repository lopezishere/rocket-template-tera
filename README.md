# rocket-template-tera

Based on [rocket-template](https://github.com/UpsettingBoy/rocket-template)

This repository contains a templete to quickly create a web project based on the [Rocket framework](https://rocket.rs/) + [Tera](https://tera.netlify.app) with [TailwindCSS](https://tailwindcss.com/) for writing the frontend.

More content will be added in wikis in the future.

## Requirements
- [Rustup](https://rustup.rs/) installed (1.23.0+).
- [NodeJS](https://nodejs.org/en/download/) (11+)

## Usage
1. Clone the repo.
1. Under the main folder execute `cargo run`.
1. Navigate to `localhost:8000`.

## Structure
The project is divided into 2 main folders: **public** and **src**.
- **public**: Here is all the frontend. Since Tera is being used, is recommended to read the [Tera Documentation](https://tera.netlify.app/docs/). TailwindCSS can be used.
- **src**: Backend code and backend <-> frontend linkage.