
## DBank Contract

### Functions: Responsible for the issuance, lending, and selling of Bollar, and also a Liquidity Pool
### DBank Contract: Accept user pledge/loan (receive pledged assets Babylon/usdc)
0. The controller of DBank Contract is BollarDAO
1. Accept user stake asset request, record user staked asset information and leverage multiple, such as 2/3/5/10 times, etc.
2. Use pledge asset information and leverage multiples and other parameters to create Intent contract (open an account), leverage multiples cannot be changed
3. Transfer user pledged Babylon tokens to Intent contract
4. Set the owner of Intent contract to the pledge user, and you can trade with ownership
5. Record Intent contract list
6. Platform function entrance