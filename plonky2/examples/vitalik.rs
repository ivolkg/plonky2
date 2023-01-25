use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

/// An example of using Plonky2 to prove the example Vitalik
/// use in his blogpost https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649
/// I.e., x^3 + x + 5 = 35;
fn main() -> Result<()> {
    const D: usize = 2;
    // Use Poseidon hash
    type C = PoseidonGoldilocksConfig;
    // Use Plonky2 field
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    let private_input = builder.add_virtual_target();
    let square_target = builder.mul(private_input, private_input);
    let mut cur_target = builder.mul_add(square_target, private_input, private_input);
    let five_target = builder.constant(F::from_canonical_u32(5));
    cur_target = builder.add(cur_target, five_target);

    // Public input is just 35
    builder.register_public_input(cur_target);
    let mut pw = PartialWitness::new();
    let three = F::from_canonical_u32(3);
    println!("{}", three);
    pw.set_target(private_input, three);

    let data = builder.build::<C>();
    let proof = data.prove(pw)?;

    println!("The result of the equation is {}", proof.public_inputs[0]);

    data.verify(proof)
}
