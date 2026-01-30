use serde::{Deserialize, Serialize};
use zela_std::{CustomProcedure, RpcError, zela_custom_procedure};

// Define an empty struct to serve as a binding to rockrpc_custom_procedure trait.
pub struct HelloWorld;

// Define input parameters for your method
#[derive(Deserialize, Debug)]
pub struct Input {
    first_number: i32,
    second_number: i32,
}

// Define output data of your method
#[derive(Serialize)]
pub struct Output {
    pub sum: i32,
}

impl CustomProcedure for HelloWorld {
    type Params = Input;
    type ErrorData = ();
    type SuccessData = Output;

    // Run method is the entry point of every custom procedure
    // It will be called once for each incoming request.
    async fn run(params: Self::Params) -> Result<Self::SuccessData, RpcError<Self::ErrorData>> {
        log::info!("Hello world!");
        log::debug!("params: {params:?}");

        // Dummy exception how to return an error
        if params.first_number == 0 {
            return Err(RpcError {
                code: 400,
                message: String::from("Example of an error -- number cannot be 0."),
                data: None,
            });
        }

        // Create a memory spike and cause processing delay
        let large_array: Vec<i32> = (0..10_000).collect();
        let mut sum_work = 0i64;
        for &num in &large_array {
            sum_work += num as i64;
            // Do some extra computation to slow things down
            sum_work = sum_work.wrapping_mul(params.first_number as i64);
            sum_work = sum_work.wrapping_add(params.second_number as i64);
        }
        
        // Use the result to prevent compiler optimization
        log::debug!("Completed heavy work: {}", sum_work % 1000);

        // Assemble response struct.
        // It will be serialized into the JSON response using serde_json.
        Ok(Output {
            sum: params.first_number + params.second_number,
        })
    }

    const LOG_MAX_LEVEL: log::LevelFilter = log::LevelFilter::Debug;
}

// This is an essential macro-call that enables us to run a procedure
zela_custom_procedure!(HelloWorld);
