/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category ClosePolicy
 * @category generated
 */
export const closePolicyStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number[] /* size: 8 */
}>(
  [['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)]],
  'ClosePolicyInstructionArgs'
)
/**
 * Accounts required by the _closePolicy_ instruction
 *
 * @property [_writable_] policy
 * @property [**signer**] authority
 * @category Instructions
 * @category ClosePolicy
 * @category generated
 */
export type ClosePolicyInstructionAccounts = {
  policy: web3.PublicKey
  authority: web3.PublicKey
  systemProgram?: web3.PublicKey
}

export const closePolicyInstructionDiscriminator = [
  55, 42, 248, 229, 222, 138, 26, 252,
]

/**
 * Creates a _ClosePolicy_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category ClosePolicy
 * @category generated
 */
export function createClosePolicyInstruction(
  accounts: ClosePolicyInstructionAccounts,
  programId = new web3.PublicKey('ocp4vWUzA2z2XMYJ3QhM9vWdyoyoQwAFJhRdVTbvo9E')
) {
  const [data] = closePolicyStruct.serialize({
    instructionDiscriminator: closePolicyInstructionDiscriminator,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.policy,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}