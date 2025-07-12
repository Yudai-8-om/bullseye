# 🐂 BullsEye

Bullseye is built as a personal project to analyze earnings and financial trends of companies. (⚠️The scraping logic is not included in this repository)

## Tech Stack

- **Frontend:** React, TypeScript, TailwindCSS
- **Backend:** Rust (Axum, Diesel)
- **Database:** PostgreSQL
- **Deployment:** Docker, NGINX

## How to start

<ol>
    <li>Modify .env file as you like. </li>
    <li>Run <code>docker compose up --build</code></li>
    This will <ul>
    <li>set up Postgres & Chrome driver for scraping</li>
    <li>configure migrations for diesel</li>
    <li>start both frontend and backend binary</li>
    </ul>
    <li>Open <code>localhost</code> in your browser.</li>
</ol>

## Screenshot

![screenshot](./screenshot.png)

## Roadmap

- [x] Dockerize app
- [ ] Simplify scraper
- [ ] Better Screening features
- [ ] Plot metrics
- [ ] Implement task scheduler / crawler
