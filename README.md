# az-nuke

Simple Rust command to delete every resource group in a subscription

:bangbang: This will nuke all resource groups in a subscription *without asking confirmation*

## Dependencies

The azure CLI must be installed and logged into an account

Cargo is needed to build from source, binaries for windows and linux are also available in the release

## Install

```cargo install --git https://github.com/JMans15/az-nuke```

## Usage

```az-nuke [subscription-id]```
