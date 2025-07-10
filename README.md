# 🚰 Allfeat Faucet

This repository contains the implementation of a **faucet service** for the [Allfeat](https://allfeat.org) network. It includes an Axum-based backend and a Leptos-powered frontend that allows users to request test tokens 💸.

---

## 🗂️ Project Structure

- `backend/` – HTTP service exposing the REST API and WebSocket endpoints; also serves the web app
- `frontend/` – Web application built with [Leptos](https://github.com/leptos-rs/leptos) and [Trunk](https://trunkrs.dev)
- `shared/` – Shared types used by both frontend and backend

---

## ⚙️ Requirements

- [Rust](https://www.rust-lang.org/tools/install) (see `rust-toolchain.toml`)
- [`trunk`](https://trunkrs.dev) to build the frontend
- [`just`](https://github.com/casey/just) (optional) for task automation via the `justfile`

---

## 🛠️ Configuration

Copy the example env file and fill in the variables:

```bash
cp backend/.env.example backend/.env
```

Edit `.env`:

```env
BACKEND_PORT=3000
FAUCET_AMOUNT=10
SENDER_SEED=
NODE_ENDPOINT_URL=
CF_SECRET=
CF_SITEKEY=
```

---

## 🚀 Running the Project

**Development mode:**

```bash
just dev
```

**Production build:**

```bash
just start
```

These commands build the frontend and launch the backend on `http://localhost:3000`.

---

## 🔌 Main Endpoints

- `POST /api/transfer` – triggers token transfer after captcha validation
- `GET /ws` – WebSocket that streams live transaction status
- `GET /api/cf_sitekey` – provides the Turnstile public sitekey to the frontend

The static frontend is built into `frontend/dist` and served by the backend.

---

## 📄 Summary

This README provides a quick overview of how to configure, run, and understand the Allfeat Faucet — a test token distribution system for developers building on Allfeat 🛠️🎶
