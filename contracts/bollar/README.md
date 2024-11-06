
## Bollar Vault Contract

### Functions: Responsible for the issuance, lending, and selling of Bollar, and also a Liquidity Pool

The lists of functions:
- The controller of Bollar Vault is BollarDAO

- The total value of Bollar is related to BTC, about 50% of the staked BTC, and the face value of each Bollar is equivalent to Dollar (USD).

- Issue 50% of the value of Bollar according to the number of BTC pledged on the Babylon chain. For example, when BTC/USD is 70,000, 35,000 Bollars will be issued.

- Users who staked btc receive BYT (Bollar Yield Token), the number of BYT is related to the number of btc staked, BYT can be used to collect interests or trade,

- Responsible for Bollar entering the market, there are two ways: over-collateralize 120% or purchase with USD 1:1

    -  Users can borrow Bollar by over-collateralizing Babylon/USDC and other tokens. The over-collateralization ratio is temporarily set at 120%. If a user Stakes Babylon worth 1,200 U, users get 1,000 Bollar and 1,000 BYT. The staked assets are held by Bollar Vault for liquidation

    - Users use U 1:1 to purchase the same amount of Bollar

- Provide the function of using BYT to collect interest

- Record the lending and purchase records of Bollar

- Record the circulation of Bollar

- Record and display the details of Bollar's pledged assets

- Check the risk of pledged assets. When it is necessary to liquidate the pledged assets, transfer the pledged assets to the Agent contract for trading and exchange them for Bollar/USD