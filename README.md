# xHousing

## Overview

In Abuja, Nigeria, the real estate market faces significant challenges that hinder both investors and developers. The high cost of entry, lack of access to liquid funds, and rising construction costs due to inflation create a market that is inaccessible to many potential investors and strained for developers. This situation results in insufficient housing availability, sub-standard construction quality, and expensive housing prices, both for purchase and rental. xHousing aims to address these multifaceted issues by leveraging blockchain technology to democratise real estate investment and provide developers with much-needed liquidity.

![Dashboard View](dashboard.png)
_Dashboard View_

![Properties View](properties.png)
_Properties View_

## Features

### Tokenization of Real Estate

**Fractional Ownership:** Real estate properties are tokenized, allowing multiple investors to own fractions of a property.
Semi-Fungible Tokens (SFTs): Each property has its own SFT collection representing fractional ownership.

### XHT Token

**Native Token:** Used for initial fundraising and rewarding participants.
Staking: Users can stake XHT and SFTs to earn rewards.

**Governance:** Stakers receive governance NFTs for voting rights and loyalty programs.

### Referral System

**Bonuses:** Referrers earn bonuses when users they referred claim staking rewards.

**Royalties:** Referrers earn a fraction of the royalties from secondary sales of SFTs.

### Rent Rewards

**Monthly Distribution:** Rent paid in various currencies is converted to XHT and distributed to SFT holders over a year.

**Burn Mechanism:** A fraction of the rent converted to XHT is burned, increasing the token's value.

## Impact and Relevance

### Democratization of Real Estate Investment

**Accessibility:** Allows individuals with limited capital to invest in real estate, traditionally a high-barrier market.

**Inclusivity:** Broadens the investor base and democratizes access to lucrative real estate investments.

### Support for Developers

**Liquidity:** Provides developers with immediate funds through token sales, ensuring timely and quality project completion.

**Affordable Housing:** Helps address the housing challenges in Abuja by enabling the development of high-quality, affordable housing projects.

### Economic Growth

**Boosting Investments:** By facilitating more investments in real estate, the platform contributes to economic stability and growth in the region.

**Job Creation:** Development projects lead to job creation and improved infrastructure in local communities.

## Smart Contracts

A series of smart contracts powers xHousing platform, each playing a vital role in its ecosystem:

1.  **Coinbase Contract**: [Link to README](./contracts/coinbase/README.md)

    The `Coinbase` contract is responsible for managing the distribution and economics of XHT tokens
    within the xHousing platform. It interacts with the `XHTModule` to perform token operations
    such as minting and burning. The contract ensures efficient token supply management by
    distributing XHT tokens to users participating in platform activities (e.g., fundraising,
    staking, referrals) and burning tokens to support deflationary mechanisms. Ownership and
    access control mechanisms are included to restrict certain operations to authorized parties,
    maintaining the integrity and security of the platform's token economy.

2.  **xHousing Contract**: [Link to README](./contracts/x-housing/README.md)

    The XHousing Contract is the main contract for the xHousing ecosystem.
    This contract owns and deploys xProject contracts which will represent the properties owned and managed by the xHousing project.
    The management of ecosystem users will also be done in this contract.

3.  **xProject Template Contract**: [Link to README](./contracts/x-project/README.md)

    The `xProject` contract template serves as the foundational blueprint for deploying
    individual real estate projects within the xHousing ecosystem.
    Each `xProject` contract represents a unique real estate development,
    managing its ownership, revenue distribution, and participant interactions.

4.  **xProject Funding Contract**: [Link to README](./contracts/x-project-funding/README.md)

    The `xProjectFunding` contract is designed to manage the crowdfunding process for real estate projects
    within the xHousing ecosystem. This contract facilitates the collection of funds from participants, handles
    participant registrations, deploys the `xProject` contract upon successful funding, and disburses tokens (XHT and SFT)
    to contributors. If the funding goal is not met by the deadline, it allows participants to withdraw their funds.

## Media

-   [Pitch Deck](Pitch.key)
-   [Presentation](https://youtu.be/-h8h9u_LK2c)

## MultiverseX Devnet Deployments

Contracts deployed on the MultiversX Devnet are:

1. [Coinbase](https://devnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgqmepkngjhgw4kff4u976rmdpjpv3uc4mp0fuskv4y8j): erd1qqqqqqqqqqqqqpgqmepkngjhgw4kff4u976rmdpjpv3uc4mp0fuskv4y8j
1. [xHousing](https://devnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgqs22spcvt0c4rl8zmfyufwxuqu2xneze40fus6t92fu): erd1qqqqqqqqqqqqqpgqs22spcvt0c4rl8zmfyufwxuqu2xneze40fus6t92fu
1. [xFunding](https://devnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgql5664ympcv9cfg7semytlsevw4jcu0gf0fusdshzex): erd1qqqqqqqqqqqqqpgql5664ympcv9cfg7semytlsevw4jcu0gf0fusdshzex
1. [xProjects Template](https://devnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgq2dwfaw667t5tp059jdfhwxqj34lwn8t80fusl3l40l): erd1qqqqqqqqqqqqqpgq2dwfaw667t5tp059jdfhwxqj34lwn8t80fusl3l40l

# Project Set Up

-   Clone this Repo
-   Cd into the folder
-   Create a `config.toml` file in the `interact-rs` folder with the following content set

```toml
contracts_owner_pem = "path-to-pem-file.pem" # Used to deploy all contracts
gateway = "the-multiversx-public-gateway-you-chose-or-alocal-one" # eg "https://devnet-gateway.multiversx.com"
```

-   Now run `cargo run start-ico`. This will deploy all the contracts and create an `interaction` folder
-   Inside the interaction folder a `state.toml` file will be created, copy the contents and populate the appropriate `x-housing-ui/src/config/**` file
-   Now cd into the `x-housing-ui` folder and run `yarn install`
-   Theb run `yarn build-{CONFIG}` where `CONFIG` can be `devnet`, `localnet`, etc
-   Finally run `yarn preview` then visit the page at `http://localnet:3000`
