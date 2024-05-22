use scrypto_test::prelude::*;

use test_22_05_2024::hello_test::*;

#[test]
fn test_hello() {
    // Setup the environment
    let mut ledger = LedgerSimulatorBuilder::new().build();

    // Create an account
    let (public_key, _private_key, _account) = ledger.new_allocated_account();

    // Publish package
    let package_address = ledger.compile_and_publish(this_package!());

    // Test the `instantiate_hello` function.
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address,
            "Hello",
            "instantiate_hello",
            manifest_args!(),
        )
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    let result = ledger.component_state::<Hello>(component.into());

    println!("state name {:?}", result.to_owned());

}

// #[test]
// fn test_hello_with_test_environment() -> Result<(), RuntimeError> {
//     // Arrange
//     let mut env = TestEnvironment::new();
//     let package_address =
//         PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;
//
//     let mut hello = Hello::instantiate_hello(package_address, &mut env)?;
//
//     // Act
//     let bucket = hello.free_token(&mut env)?;
//
//     // Assert
//     let amount = bucket.amount(&mut env)?;
//     assert_eq!(amount, dec!("1"));
//
//     Ok(())
// }
