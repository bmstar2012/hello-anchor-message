`react app`

npx create-react-app app
npm install @project-serum/anchor @solana/web3.js
npm install @solana/wallet-adapter-react \
@solana/wallet-adapter-react-ui @solana/wallet-adapter-wallets \
@solana/wallet-adapter-base


`wallet`
- create
solana-keygen new --outfile ./my-keypair.json

- verify
solana-keygen verify A9mZF5t6jhRoXDamtF69NrxZpBbAX9VSs6WiDv6D9ebv ./my-keypair.json 

- airdrop
solana airdrop 1 A9mZF5t6jhRoXDamtF69NrxZpBbAX9VSs6WiDv6D9ebv --url https://api.devnet.solana.com
