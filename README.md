# orri


## Overview
orri is a web service for hosting websites.
A user can create a site by choosing a subdomain, creating routes and uploading files that will be served at those routes.
A hosted instance is available at [orri.dev](https://orri.dev)


## Data storage
No database is used at this point. Each site is stored in its own directory on the servers filesystem.
The directory contains any uploaded files and a `site.json` with the details of the site and its routes.


## Maintainability
Vanilla js and a css framework without a build step was deliberately choosen
to avoid having to change the frontend build system every 6 months.


## How to build
Use rust tooling: `cargo build`.  
Or nix: `nix build -f Cargo.nix rootCrate.build`.


## How to run
The backend is configured via environment variables, see `run.sh` for an example how to run a development server.
