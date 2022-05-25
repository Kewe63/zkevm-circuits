use super::Opcode;
use crate::circuit_input_builder::{CircuitInputStateRef, ExecStep};
use crate::Error;
use eth_types::GethExecStep;

/// Placeholder structure used to implement [`Opcode`] trait over it
/// corresponding to the [`OpcodeId::STOP`](crate::evm::OpcodeId::STOP)
/// `OpcodeId`. This is responsible of generating all of the associated
/// operations and place them inside the trace's
/// [`OperationContainer`](crate::operation::OperationContainer). In the case of
/// STOP, it simply does not add anything.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Stop;

impl Opcode for Stop {
    fn gen_associated_ops(
        state: &mut CircuitInputStateRef,
        geth_steps: &[GethExecStep],
        index: usize,
    ) -> Result<Vec<ExecStep>, Error> {
        let geth_step = &geth_steps[index];
        let exec_step = state.new_step(geth_step)?;
        state.handle_return(geth_step)?;
        Ok(vec![exec_step])
    }
}
