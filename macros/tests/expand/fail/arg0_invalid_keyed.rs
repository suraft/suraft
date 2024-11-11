fn main() {
    suraft_macros::expand!(
        !FOO,
        (K, T, V) => {K; T; V;},
    );

    suraft_macros::expand!(
        FOO,
        (K, T, V) => {K; T; V;},
    );
}
