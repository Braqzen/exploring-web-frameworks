use crate::{methods::Method, operation::Operation, settings::ProviderName};
use rand::{
    RngExt,
    distr::{Alphanumeric, SampleString},
    rng,
    seq::SliceRandom,
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;
use uuid::Uuid;

pub struct Randomiser;

impl Randomiser {
    pub fn requests(provider: &ProviderName, methods: &[Method]) -> Vec<(Method, u16)> {
        // SAFETY: Method assumes config forces at least 1 method to be enabled
        let mut requests = Self::try_randomise_methods(provider, methods);

        // Unlikely to loop much if at all
        while requests.is_none() {
            requests = Self::try_randomise_methods(provider, methods);
        }

        requests.unwrap()
    }

    pub fn operations() -> [(Operation, u16); 5] {
        let mut operations = Self::try_randomise_operations();

        // Realistically this will never trigger and if it does it likely won't loop a 2nd time
        while operations.is_none() {
            operations = Self::try_randomise_operations();
        }

        operations.unwrap()
    }

    pub fn secret_size() -> usize {
        if rng().random_range(0..=100) < 5 {
            rng().random_range(65506..=65550)
        } else {
            rng().random_range(10..=32)
        }
    }

    pub fn id() -> String {
        if rng().random_range(0..=100) < 50 {
            Uuid::new_v4().to_string()
        } else {
            Alphanumeric
                .sample_string(&mut rng(), rng().random_range(10..=32))
                .to_string()
        }
    }

    pub async fn sleep() {
        const MILLISECONDS: u64 = 1000;

        // Randomly sleep for 5 to 15 seconds after each randomisation to buffer updates
        let sleep_duration = rng().random_range(5 * MILLISECONDS..=15 * MILLISECONDS);

        sleep(Duration::from_millis(sleep_duration)).await;
    }

    /// Randomly create a distribution of methods to send to a server.
    ///
    /// I.e. call this method X% of the time.
    ///
    /// The call selection assumes that at least 1 method is non-zero so return None if all were set to 0.
    fn try_randomise_methods(
        provider: &ProviderName,
        methods: &[Method],
    ) -> Option<Vec<(Method, u16)>> {
        // Select a random number to use to spread out the method weights
        let mut tokens: u16 = 1000;

        // Track if at least 1 method is non-zero
        let mut non_zero = false;

        let mut requests = methods
            .iter()
            .map(|method| (method.clone(), 0))
            .collect::<Vec<_>>();

        requests.shuffle(&mut rng());

        // For the sake of logging (and reducing performance) track each value
        let mut post = 0;
        let mut get = 0;
        let mut patch = 0;
        let mut put = 0;
        let mut delete = 0;
        let mut head = 0;

        // Randomly set a weight for each method
        requests.iter_mut().for_each(|(method, weight)| {
            let random_weight = rng().random_range(0..=tokens);

            tokens -= random_weight;
            *weight = random_weight;

            match method {
                Method::Post => post = random_weight,
                Method::Get => get = random_weight,
                Method::Patch => patch = random_weight,
                Method::Put => put = random_weight,
                Method::Delete => delete = random_weight,
                Method::Head => head = random_weight,
            }

            if 0 < random_weight {
                non_zero = true;
            }
        });

        if non_zero {
            let total = post + get + patch + put + delete + head;
            info!(
                post = ((post as f64 / total as f64) * 1000.0).round() / 10.0,
                get = ((get as f64 / total as f64) * 1000.0).round() / 10.0,
                patch = ((patch as f64 / total as f64) * 1000.0).round() / 10.0,
                put = ((put as f64 / total as f64) * 1000.0).round() / 10.0,
                delete = ((delete as f64 / total as f64) * 1000.0).round() / 10.0,
                head = ((head as f64 / total as f64) * 1000.0).round() / 10.0,
                total,
                provider = provider.to_string(),
                "Randomised requests"
            );
            Some(requests)
        } else {
            None
        }
    }

    fn try_randomise_operations() -> Option<[(Operation, u16); 5]> {
        // Select a random number to use to spread out the operation weights
        let mut tokens: u16 = 1000;

        // Track if at least 1 operation is non-zero
        let mut non_zero = false;

        let mut operations = [
            (Operation::Compute, 0),
            (Operation::Merge, 0),
            (Operation::Sort, 0),
            (Operation::Transform, 0),
            (Operation::Filter, 0),
        ];

        operations.shuffle(&mut rng());

        // For the sake of logging (and reducing performance) track each value
        let mut compute = 0;
        let mut merge = 0;
        let mut sort = 0;
        let mut transform = 0;
        let mut filter = 0;

        // Randomly set a weight for each operation
        operations.iter_mut().for_each(|(operation, weight)| {
            let random_weight = rng().random_range(0..=tokens);

            tokens -= random_weight;
            *weight = random_weight;

            match operation {
                Operation::Compute => compute = random_weight,
                Operation::Merge => merge = random_weight,
                Operation::Sort => sort = random_weight,
                Operation::Transform => transform = random_weight,
                Operation::Filter => filter = random_weight,
            }

            if 0 < random_weight {
                non_zero = true;
            }
        });

        if non_zero {
            let total = compute + merge + sort + transform + filter;
            info!(
                compute = ((compute as f64 / total as f64) * 1000.0).round() / 10.0,
                merge = ((merge as f64 / total as f64) * 1000.0).round() / 10.0,
                sort = ((sort as f64 / total as f64) * 1000.0).round() / 10.0,
                transform = ((transform as f64 / total as f64) * 1000.0).round() / 10.0,
                filter = ((filter as f64 / total as f64) * 1000.0).round() / 10.0,
                total,
                "Randomised operations"
            );
            Some(operations)
        } else {
            None
        }
    }
}
