---
title: Overview
description: "Provides a high-level overview of the Open Creator Protocol."
---

# Overview

We understand the ecosystem has been waiting for a solution and are excited to share our feature-packed standard that
prioritizes principles of flexibility and customization for the creator. Open Creator Protocol, an open source and
feature-packed standard built on Solana Token Standard that enables creators to protect royalties.

* New collections can launch with our open-source Open Creator Protocol to protect royalties.
* The protocol is entirely open source and built to serve creators.
* Magic Eden will protect royalties on all collections who adopt the protocol. The protocol allows creators to ban
  marketplaces that have not protected royalties on their collection.
* Existing collections will have the ability to burn and re-mint their existing collections on the Open Creator
  Protocol.

OCP NFTs are based on `spl-managed-token`, `spl-token`, and `token metadata` programs.

- Token Mint (supply = 1, decimals = 0)
- Token Account
- Token Metadata

By definition, it's the same implementation of all the NFTs on solana. Everything is the same including
interacting with wallets (except "transfer", that users can use ME profile page to send tokens including OCP NFTs),
run token gated content, and prove token ownerships exactly like the normal Normal NFTs.

:::note **What about Metaplex?**
We intend to support Metaplex’s MIP-1 when it goes live in Q4 2022. MIP-1 will offer a
migration path towards royalty protection for existing collections. We also remain open-minded to adopting other
standards that receive market adoption and look forward to community feedback.
:::

## How to get started with Open Creator Protocol

* Get familiar with deploying the Open Creator Protocol by reading the [Tutorials](01-tutorials.md).
* New creators can mint their collection with the [Open Creator Protocol CLI tool](04-cli.md).
* If you are interested in applying to Launchpad and using Open Creator Protocol, [please fill out an inquiry form](https://airtable.com/shrMhMDpcvt9nB6cu). Of
  course, collections do not need to launch on Launchpad in order to use Open Creator Protocol.

## Features

We created this Open Creator Protocol because we wanted to provide tools to creators as soon as possible. We look
forward to a world where there are different standards and creators can choose what works best for them. See below for
the full feature list.

### Protected royalties

We will protect royalties on all collections who adopt the standard. The protocol allows creators
to ban marketplaces that have not protected royalties on their collection. For new collections that do not adopt the
standard, royalties will remain optional on Magic Eden. We will also welcome the inclusion of other future royalty
protection protocols that emerge and gain market adoption.

### Open source

This is a tool built on top of Solana’s SPL managed-token standard. Creators will be in control of creating
and managing the rules of their collection.

### Dynamic royalties

Creators can specify a relationship between an NFT’s sale price and royalty amount via a linear price curve. For
example, creators can reduce the nominal value of royalties for buyers who pay a higher price for the NFT.

### Freeze trading until mint is done

Creators can use Open Creator Protocol to limit trading until after mint is
complete (only time based)

### Create rules for NFT transfers

Creators can game-ify transferability for collections, including completely non-transferable tokens (or restrictions based
on time, # of transfers, or metadata name)

### Bulk transfers directly on Magic Eden

Magic Eden is also unveiling bulk transfers on the platform so collectors
can move their NFTs freely for collections using the Open Creator Protocol. These bulk transfers will be subject to the
token transferability rules the creator sets.
