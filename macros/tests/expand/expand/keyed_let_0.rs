fn main() {
    suraft_macros::expand!(
        KEYED,
        (K, T, V) => {let K: T = V;},
    );
}
