@group(0)
@binding(0)
var<storage, read_write> v_indices: array<u32>; // this is used as both input and output for convenience

// The Collatz Conjecture states that for any integer n:
// If n is even, n = n/2
// If n is odd, n = 3n+1
// And repeat this process for each new n, you will always eventually reach 1.
// Though the conjecture has not been proven, no counterexample has ever been found.
// This function returns how many times this recurrence needs to be applied to reach 1.
fn next_power_of_2(n_base: u32) -> u32{
    var n: u32 = n_base;
    var i: u32 = 1u;
    if (n <= 1u) {
        return 1u;
    }
    loop {
        let q = 1u << i;
        if (q >= n_base) {
            return q;
        }
        else {
            i += 1u;
        }
    }
    return 0u;
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices[global_id.x] = next_power_of_2(v_indices[global_id.x]);
}
