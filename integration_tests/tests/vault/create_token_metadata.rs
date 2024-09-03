#[cfg(test)]
mod tests {
    use borsh::BorshDeserialize;
    use solana_program_test::{processor, ProgramTest};
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
    use vault_sdk::inline_mpl_token_metadata;

    #[tokio::test]
    async fn test_create_token_metadata_ok() {
        let mut program_test = ProgramTest::default();
        program_test.add_program(
            "vault_program",
            vault_program::id(),
            processor!(vault_program::process_instruction),
        );
        program_test.prefer_bpf(true);
        program_test.add_program("token_metadata", inline_mpl_token_metadata::id(), None);

        let mut context = program_test.start_with_context().await;

        let mint_account = Keypair::new();

        // Create token metadata
        let name = "restaking JTO";
        let symbol = "rJTO";
        let uri = "https://www.jito.network/restaking/";

        let metadata_pubkey =
            inline_mpl_token_metadata::pda::find_metadata_account(&mint_account.pubkey()).0;

        let ix = vault_sdk::sdk::create_token_metadata(
            &vault_program::id(),
            &mint_account.pubkey(),
            &context.payer.pubkey(),
            &metadata_pubkey,
            &context.payer.pubkey(),
            &spl_token_2022::id(),
            name.to_string(),
            symbol.to_string(),
            uri.to_string(),
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&context.payer.pubkey()),
            &[&mint_account, &context.payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        let token_metadata_account = context
            .banks_client
            .get_account(metadata_pubkey)
            .await
            .unwrap()
            .unwrap();
        let metadata = crate::helpers::token::Metadata::deserialize(
            &mut token_metadata_account.data.as_slice(),
        )
        .unwrap();

        assert!(metadata.name.starts_with(name));
        assert!(metadata.symbol.starts_with(symbol));
        assert!(metadata.uri.starts_with(uri));
    }
}
