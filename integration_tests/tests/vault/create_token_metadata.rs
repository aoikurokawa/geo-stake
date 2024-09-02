#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use borsh::BorshDeserialize;
    use litesvm::LiteSVM;
    use solana_sdk::{
        native_token::LAMPORTS_PER_SOL, pubkey, signature::Keypair, signer::Signer,
        transaction::Transaction,
    };
    use vault_sdk::inline_mpl_token_metadata;

    fn read_vault_program() -> Vec<u8> {
        let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        so_path.push("../target/deploy/vault.so");
        println!("SO path: {:?}", so_path.to_str());
        std::fs::read(so_path).unwrap()
    }

    fn read_mpl_token_metadata_program() -> Vec<u8> {
        let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        so_path.push("tests/fixtures/mpl_token_metadata.so");
        println!("SO path: {:?}", so_path.to_str());
        std::fs::read(so_path).unwrap()
    }

    #[test]
    fn test_create_token_metadata_ok() {
        let mut svm = LiteSVM::new();
        svm.add_program(vault_program::id(), &read_vault_program());
        svm.add_program(
            inline_mpl_token_metadata::id(),
            &read_mpl_token_metadata_program(),
        );

        let payer_kp = Keypair::new();
        let payer_pk = payer_kp.pubkey();

        svm.airdrop(&payer_pk, 10 * LAMPORTS_PER_SOL).unwrap();

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
            &payer_pk,
            &metadata_pubkey,
            &payer_pk,
            &spl_token::id(),
            name.to_string(),
            symbol.to_string(),
            uri.to_string(),
        );

        let blockhash = svm.latest_blockhash();
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer_pk),
            &[&mint_account, &payer_kp],
            blockhash,
        );

        let tx_result = svm.send_transaction(tx).unwrap();
        // assert!(tx_result.is_ok());

        // let token_metadata = get_token_metadata(&mint_account.pubkey()).unwrap();
        let token_metadata_account = svm.get_account(&metadata_pubkey).unwrap();
        let metadata = crate::helpers::token::Metadata::deserialize(
            &mut token_metadata_account.data.as_slice(),
        )
        .unwrap();

        assert!(metadata.name.starts_with(name));
        assert!(metadata.symbol.starts_with(symbol));
        assert!(metadata.uri.starts_with(uri));
    }

    // pub async fn get_token_metadata(
    //     &mut self,
    //     vrt_mint: &Pubkey,
    // ) -> Result<crate::helpers::token::Metadata, TestError> {
    //     let metadata_pubkey = inline_mpl_token_metadata::pda::find_metadata_account(vrt_mint).0;
    //     let token_metadata_account = self
    //         .banks_client
    //         .get_account(metadata_pubkey)
    //         .await?
    //         .unwrap();
    //     let metadata = crate::helpers::token::Metadata::deserialize(
    //         &mut token_metadata_account.data.as_slice(),
    //     )
    //     .unwrap();
    //     Ok(metadata)
    // }

    // #[tokio::test]
    // async fn test_create_token_metadata_wrong_vrt_mint_fails() {
    //     let fixture = TestBuilder::new().await;

    //     let mut vault_program_client = fixture.vault_program_client();

    //     let (
    //         _config_admin,
    //         VaultRoot {
    //             vault_pubkey,
    //             vault_admin,
    //         },
    //     ) = vault_program_client
    //         .setup_config_and_vault(99, 100, 0)
    //         .await
    //         .unwrap();

    //     let random_mint = Keypair::new();
    //     vault_program_client
    //         .create_token_mint(&random_mint)
    //         .await
    //         .unwrap();

    //     let vault = vault_program_client.get_vault(&vault_pubkey).await.unwrap();

    //     // Create token metadata
    //     let name = "restaking JTO";
    //     let symbol = "rJTO";
    //     let uri = "https://www.jito.network/restaking/";

    //     let metadata_pubkey =
    //         inline_mpl_token_metadata::pda::find_metadata_account(&vault.vrt_mint).0;

    //     let result = vault_program_client
    //         .create_token_metadata(
    //             &vault_pubkey,
    //             &vault_admin,
    //             &random_mint.pubkey(),
    //             &vault_admin,
    //             &metadata_pubkey,
    //             name.to_string(),
    //             symbol.to_string(),
    //             uri.to_string(),
    //         )
    //         .await;
    //     assert_eq!(
    //         result.unwrap_err().to_transaction_error().unwrap(),
    //         TransactionError::InstructionError(0, InstructionError::InvalidAccountData)
    //     );
    // }

    // #[tokio::test]
    // async fn test_create_token_metadata_wrong_metadata_fails() {
    //     let fixture = TestBuilder::new().await;

    //     let mut vault_program_client = fixture.vault_program_client();

    //     let (
    //         _config_admin,
    //         VaultRoot {
    //             vault_pubkey,
    //             vault_admin,
    //         },
    //     ) = vault_program_client
    //         .setup_config_and_vault(99, 100, 0)
    //         .await
    //         .unwrap();

    //     let vault = vault_program_client.get_vault(&vault_pubkey).await.unwrap();

    //     // Create token metadata
    //     let name = "restaking JTO";
    //     let symbol = "rJTO";
    //     let uri = "https://www.jito.network/restaking/";

    //     let result = vault_program_client
    //         .create_token_metadata(
    //             &vault_pubkey,
    //             &vault_admin,
    //             &vault.vrt_mint,
    //             &vault_admin,
    //             &Pubkey::new_unique(),
    //             name.to_string(),
    //             symbol.to_string(),
    //             uri.to_string(),
    //         )
    //         .await;
    //     assert_eq!(
    //         result.unwrap_err().to_transaction_error().unwrap(),
    //         TransactionError::InstructionError(0, InstructionError::InvalidAccountData)
    //     );
    // }

    // #[tokio::test]
    // async fn test_wrong_admin_signed() {
    //     let fixture = TestBuilder::new().await;

    //     let mut vault_program_client = fixture.vault_program_client();

    //     let (
    //         _config_admin,
    //         VaultRoot {
    //             vault_pubkey,
    //             vault_admin,
    //         },
    //     ) = vault_program_client
    //         .setup_config_and_vault(99, 100, 0)
    //         .await
    //         .unwrap();

    //     let vault = vault_program_client.get_vault(&vault_pubkey).await.unwrap();

    //     // Create token metadata
    //     let name = "restaking JTO";
    //     let symbol = "rJTO";
    //     let uri = "https://www.jito.network/restaking/";

    //     let metadata_pubkey =
    //         inline_mpl_token_metadata::pda::find_metadata_account(&vault.vrt_mint).0;

    //     let bad_admin = Keypair::new();
    //     let response = vault_program_client
    //         .create_token_metadata(
    //             &vault_pubkey,
    //             &bad_admin,
    //             &vault.vrt_mint,
    //             &vault_admin,
    //             &metadata_pubkey,
    //             name.to_string(),
    //             symbol.to_string(),
    //             uri.to_string(),
    //         )
    //         .await;

    //     assert_vault_error(response, VaultError::VaultAdminInvalid);
    // }
}
