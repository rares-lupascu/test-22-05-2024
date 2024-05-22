use scrypto::prelude::*;

#[blueprint]
mod hello {
    struct Hello {
        // Define what resources and data will be managed by Hello components
        name: String
    }

    impl Hello {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_hello() -> Global<Hello> {
            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            Self {
                name: "test".into()
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }
    }
}
