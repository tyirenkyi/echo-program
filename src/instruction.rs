use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum EchoInstruction {
  Echo {
    data: Vec<u8>
  },
  InitializeAuthorizedEcho,
  AuthorizeEcho,
  InitializeVendingMachineEcho,
  VendingMachineEcho,
}
